use hyper::{
    header::{ContentDisposition, ContentType, DispositionParam, DispositionType, Headers},
    mime::{Attr, Mime, SubLevel, TopLevel, Value},
};
use mime_multipart::{self, generate_boundary, write_multipart, Node, Part};
use phf::phf_map;
use std::{
    collections::hash_map::DefaultHasher,
    hash::Hasher,
    io::Write,
};

use std::{
    fs,
    path::{Path, PathBuf},
};

use super::Tooling;

/// Built in cloud init tool
pub struct CloudInit {
    user_local: String,
    user_cache: String,
}

impl Tooling for CloudInit {
    /// Creates folders for cloud_init in the user's .local and .cache folders
    fn install<T: AsRef<Path>>(mut self, user_home: T) -> Self {
        self.user_local = Self::with_local_dir(&user_home);
        self.user_cache = Self::with_cache_dir(&user_home);
        self
    }

    /// From config, format all referenced cloud-init files into a MIME message
    /// Save as user_data in the user's .cache folder (user_cache)
    fn init(self, config: &str) -> Self {
        let settings = Self::parse_tools(Self::yaml(config), vec![Self::symbol()]);

        for s in settings {
            if s.name == "cloud_init" {
                self.make_mime(s.data);
            }
        }

        self
    }

    fn symbol() -> &'static str {
        "cloud_init"
    }
}

impl Default for CloudInit {
    fn default() -> Self {
        Self {
            user_local: Default::default(),
            user_cache: Default::default(),
        }
    }
}

impl CloudInit {
    /// make_mime generates a multi-part mixed mime_body message which will be proccessed by downstream cloud_init
    /// This tool will be initialized with a list of files that it will compile into a user_data message
    /// that is intended for cloud_init
    /// These files must exist in self.toolRoot/cloud_init/
    fn make_mime(&self, data: Vec<String>) {
        if let Ok(mut f) = fs::File::create(PathBuf::from(&self.user_cache).join("user_data")) {
            let nodes: Vec<Node> = data
                .iter()
                .filter_map(|l| {
                    // Format of a settings is <filename>:<type> 
                    // only files located in cloud_init folder are valid
                    let parts: Vec<&str> = l.split(":").collect();
                    let file_name = parts[0];
                    let mime_type = parts[1];
                    let file_path = PathBuf::from(&self.user_local).join(file_name);

                    self.attach_mime(mime_type, file_path)
                })
                .collect();

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

            if let Err(err) = writeln!(&mut f, "{}", multipart_headers.to_string()) {
                panic!("{}", err);
            }

            if let Err(err) = write_multipart(&mut f, &boundary.as_bytes().to_vec(), &nodes) {
                panic!("{}", err);
            }

            if let Err(err) = writeln!(&mut f, "\n") {
                panic!("{}", err);
            }
        }
    }

    /// attach_mime formats a MIME message based on the content in filepath
    /// uses base64 encoding with a max lin length for the body
    /// designed to mimic cloud-init's make-mime format
    fn attach_mime<T: AsRef<Path>>(&self, mime_type_str: &str, filepath: T) -> Option<Node> {
        if let (Ok(body), Ok(mime_type), Some(filename)) = (
            fs::read_to_string(&filepath),
            CLOUD_INIT_MIME_TYPES[mime_type_str].parse(),
            // Strip the prefix so that the filename is just <tool>/<file>
            filepath
                .as_ref()
                .strip_prefix(PathBuf::from(&self.user_local).parent().unwrap())
                .unwrap_or(filepath.as_ref())
                .to_str(),
        ) {
            Some(Node::Part(Self::format_mime_part(
                mime_type,
                filename.to_string(),
                body,
            )))
        } else {
            None
        }
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

// TODO: The only ones I know are correct are jinja2
const CLOUD_INIT_MIME_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
        "cloud-boothook" => r#"text/cloud-boothook; charset="utf8""#,
        "cloud-config" => r#"text/cloud-config; charset="utf8""#,
        "cloud-config-archive" => r#"text/cloud-config-archive; charset="utf8""#,
        "cloud-config-jsonp" => r#"text/cloud-config-jsonp; charset="utf8""#,
        "jinja2" => r#"text/jinja2; charset="utf8""#,
        "part-handler" => r#"text/part-handler; charset="utf8""#,
        "upstart-job" => r#"text/upstart-job; charset="utf8""#,
        "x-include-once-url" => r#"text/x-include-once-url; charset="utf8""#,
        "x-include-url" => r#"text/x-include-url; charset="utf8""#,
        "x-shellscript" => r#"text/x-shellscript; charset="utf8""#,
        "x-shellscript-per-boot" => r#"text/x-shellscript-per-boot; charset="utf8""#,
        "x-shellscript-per-instance" => r#"text/x-shellscript-per-instance; charset="utf8""#,
        "x-shellscript-per-once" => r#"text/x-shellscript-per-once; charset="utf8""#,
};