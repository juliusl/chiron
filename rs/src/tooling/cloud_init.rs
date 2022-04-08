use hyper::{
    header::{ContentDisposition, ContentType, DispositionParam, DispositionType, Headers},
    mime::{Attr, Mime, SubLevel, TopLevel, Value},
};
use mime_multipart::{self, generate_boundary, write_multipart, Node, Part};
use std::{collections::hash_map::DefaultHasher, hash::Hasher, io::Write};

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::Tooling;

/// Built in cloud init tool
pub struct CloudInit {
    user_local: String,
    user_cache: String,
}

impl Tooling for CloudInit {
    fn install<T: AsRef<Path>>(mut self, user_home: T) -> Self {
        self.user_local = Self::with_local_dir(&user_home);
        self.user_cache = Self::with_cache_dir(&user_home);
        self
    }

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
    fn make_mime(&self, data: Vec<String>) {
        // let user_data = MultiPart::mixed().singlepart(jinja2);

        // let m: Message<MultiPart<&str>> = Message::builder().mime_body(user_data);

        // This tool will be initialized with a list of files that it will compile into a user_data message
        // that is intended for cloud_init
        // These files must exist in self.toolRoot/cloud_init/

        if let Ok(mut f) = fs::File::create(PathBuf::from(&self.user_cache).join("user_data")) {
            let nodes: Vec<Node> = data
                .iter()
                .filter_map(|l| {
                    let parts: Vec<&str> = l.split(":").collect();

                    let file_name = parts[0];
                    let mime_type = parts[1];
                    let file_path = PathBuf::from(&self.user_local).join(file_name);

                    self.attach_mime(&format!("text/{}", mime_type), file_path)
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

            if let Err(err) = write!(&mut f, "{}", multipart_headers.to_string()) {
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

    /// attach_mime returns a single valid file-attachment
    fn attach_mime<T: AsRef<Path>>(&self, mime_type_str: &str, filepath: T) -> Option<Node> {
        const DEFAULT_MAX_LINE_LENGTH: usize = 76;

        let mime_type: Mime = format!("{}; charset=utf8", mime_type_str)
            .parse()
            .expect("Invalid MIME-Type formatting");

        // Convert the pathbuf to a &str so it can be encoded and processed
        match filepath.as_ref().strip_prefix(
            PathBuf::from(&self.user_local)
                .parent()
                .expect("user_local should have a parent folder"),
        ) {
            Ok(file_id) => {
                if let Some(file_id) = file_id.to_str() {
                    let file_name =
                        DispositionParam::Ext("filename".to_string(), file_id.to_string());

                    // Reads the content of the file attachment to a string
                    // If successful generates a MIME part that can be attached to the body of the message
                    if let Ok(b) = fs::read_to_string(filepath) {
                        let part = Part {
                            headers: {
                                let mut headers = Headers::new();
                                headers.set(ContentType(mime_type));
                                headers.set_raw("MIME-Version", vec![b"1.0".to_vec()]);
                                headers
                                    .set_raw("Content-Transfer-Encoding", vec![b"base64".to_vec()]);
                                headers.set(ContentDisposition {
                                    disposition: DispositionType::Attachment,
                                    parameters: vec![file_name],
                                });
                                headers
                            },
                            body: {
                                let mut output = vec![];
                                let encoded = base64::encode(b.as_bytes());

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
                        };

                        return Some(Node::Part(part));
                    }
                }
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
        None
    }
}
