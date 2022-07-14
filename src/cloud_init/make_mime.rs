use hyper::{
    header::{ContentDisposition, ContentType, DispositionParam, DispositionType, Headers},
    mime::{Attr, Mime, SubLevel, TopLevel, Value},
};
use lifec::{
    plugins::{Plugin, ThunkContext},
    Component, DenseVecStorage,
};
use mime_multipart::{generate_boundary, write_multipart, Node, Part};
use phf::phf_map;
use std::{collections::hash_map::DefaultHasher, hash::Hasher, path::PathBuf};
use tokio::io::{self, AsyncWriteExt};

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct MakeMime;

impl Plugin<ThunkContext> for MakeMime {
    fn symbol() -> &'static str {
        "make_mime"
    }

    fn description() -> &'static str {
        "Formats a MIME message for cloud_init user-data"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|_| {
            let tc = context.clone();
            async move {
                if let Some(work_dir) = tc.as_ref().find_text("work_dir") {
                    if let Some(file_dst) = tc.as_ref().find_text("file_dst") {
                        tokio::fs::create_dir_all(PathBuf::from(&file_dst).parent().expect("couldn't create dirs")).await.ok();

                        match tokio::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(file_dst)
                            .await
                        {
                            Ok(file) => {
                                let mut parts = vec![];
                                for (_, part_value) in tc.as_ref().find_symbol_values("part") {
                                    if let lifec::Value::TextBuffer(part_value) = part_value {
                                        parts.push(part_value);
                                    }
                                }

                                match Self::make_mime(parts, work_dir, file).await {
                                    Ok(_) => {}
                                    Err(err) => {
                                        eprintln!("error: {}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("{err}");
                            },
                        }
                    }
                }

                None
            }
        })
    }
}

impl MakeMime {
    async fn make_mime(
        parts: Vec<String>,
        work_dir: impl AsRef<str>,
        mut file: tokio::fs::File,
    ) -> io::Result<()> {
        let mut nodes = vec![];

        for node in parts {
            // ex. define azcli part .text install-azcli.yml_jinja2
            if let Some((file_name, mime_type)) = node.split_once("_") {
                let file_path = PathBuf::from(work_dir.as_ref()).join(file_name);

                eprintln!("adding part from path {:?}", file_path);

                match tokio::fs::read_to_string(&file_path).await {
                    Ok(body) => match CLOUD_INIT_MIME_TYPES[mime_type].parse::<Mime>() {
                        Ok(mime_type) => {
                            let file_name = file_path
                                .strip_prefix(PathBuf::from(work_dir.as_ref()).parent().unwrap())
                                .unwrap_or(file_path.as_ref());
                            let filename = format!("{:?}", file_name).trim_matches('"').to_string();
                            let part = Self::format_mime_part(mime_type, filename, body);
                            let part = Node::Part(part);

                            nodes.push(part);
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }
        }

        // This follows the format that cpython uses to be consistent with cloud-init
        let boundary = generate_boundary();
        let mut hasher = DefaultHasher::new();
        hasher.write(&boundary);
        let boundary = format!("{}{}{}", "=".repeat(15), hasher.finish(), "==");

        let mut multipart_headers = Headers::new();
        let boundary_param = (Attr::Boundary, Value::Ext(format!(r#""{}""#, boundary)));
        multipart_headers.set(ContentType(Mime(
            TopLevel::Multipart,
            SubLevel::Ext("mixed".to_string()),
            vec![boundary_param],
        )));
        multipart_headers.set_raw("MIME-Version", vec![b"1.0".to_vec()]);

        let multipart_headers = multipart_headers.to_string().as_bytes().to_owned();

        file.write_all(&multipart_headers).await?;
        file.write(b"\n").await?;

        match write_multipart(
            &mut file.into_std().await,
            &boundary.as_bytes().to_vec(),
            &nodes,
        ) {
            Ok(_) => {}
            Err(_) => {}
        }

        Ok(())
    }

    /// Formats the cloud_init file into a MIME Part
    fn format_mime_part(mime_type: Mime, filename: String, body: String) -> Part {
        const DEFAULT_MAX_LINE_LENGTH: usize = 76;
        // Cloud-init's format doesn't include the charset for the filename disposition
        let file_name = DispositionParam::Ext("filename".to_string(), filename);

        Part {
            headers: {
                let mut headers = Headers::new();
                headers.set(ContentType(mime_type));
                headers.set_raw("MIME-Version", vec![b"1.0".to_vec()]);
                headers.set_raw("Content-Transfer-Encoding", vec![b"base64".to_vec()]);
                headers.set(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![file_name],
                });
                headers
            },
            body: {
                let mut output = vec![];
                let encoded = base64::encode(body.as_bytes());

                let mut lines = 0;
                loop {
                    let line: String = encoded
                        .chars()
                        .skip(lines * DEFAULT_MAX_LINE_LENGTH)
                        .take(DEFAULT_MAX_LINE_LENGTH)
                        .collect();
                    if line.is_empty() {
                        break;
                    }
                    output.push(line);
                    lines += 1;
                }

                let mut body = output.join("\r\n");
                body.push_str("\r\n");
                body.into_bytes()
            },
        }
    }
}

const CLOUD_INIT_MIME_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    "jinja2" => r#"text/jinja2; charset="utf8""#,
    // TODO: The only ones I know are correct are jinja2
    // "cloud-boothook" => r#"text/cloud-boothook; charset="utf8""#,
    // "cloud-config" => r#"text/cloud-config; charset="utf8""#,
    // "cloud-config-archive" => r#"text/cloud-config-archive; charset="utf8""#,
    // "cloud-config-jsonp" => r#"text/cloud-config-jsonp; charset="utf8""#,
    // "part-handler" => r#"text/part-handler; charset="utf8""#,
    // "upstart-job" => r#"text/upstart-job; charset="utf8""#,
    // "x-include-once-url" => r#"text/x-include-once-url; charset="utf8""#,
    // "x-include-url" => r#"text/x-include-url; charset="utf8""#,
    // "x-shellscript" => r#"text/x-shellscript; charset="utf8""#,
    // "x-shellscript-per-boot" => r#"text/x-shellscript-per-boot; charset="utf8""#,
    // "x-shellscript-per-instance" => r#"text/x-shellscript-per-instance; charset="utf8""#,
    // "x-shellscript-per-once" => r#"text/x-shellscript-per-once; charset="utf8""#,
};
