use hyper::{
    header::{ContentType, Headers},
    mime::{Attr, Mime, SubLevel, TopLevel, Value},
};
use lifec::{plugins::{Plugin, ThunkContext}, Component, DenseVecStorage};
use logos::{Lexer, Logos};
use mime_multipart::{read_multipart_body, Node};
use std::fmt::Write;
use tokio::fs::File;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct ReadMime;

impl Plugin<ThunkContext> for ReadMime {
    fn symbol() -> &'static str {
        "read_mime"
    }

    fn description() -> &'static str {
        "Decodes a MIME message from a cloud init user-data blob"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|_| {
            let mut tc = context.clone();
            async move {
                if let Some(content) = tc.as_ref().find_text("content") {
                    let mut multipart_headers = Headers::new();
                    let lines: Vec<&str> = content.lines().collect();
                    let content_type = lines[0];
                    let mime = lines[1];
                    let mut lexer = MimeHeaders::lexer(content_type);
                    if let MimeHeaders::MultipartContentType(boundary) =
                        lexer.next().expect("Content-Type header not found")
                    {
                        let boundary_param = (Attr::Boundary, Value::Ext(boundary));
                        multipart_headers.set(ContentType(Mime(
                            TopLevel::Multipart,
                            SubLevel::Ext("mixed".to_string()),
                            vec![boundary_param],
                        )));
                    }

                    if mime == "MIME-Version: 1.0" {
                        multipart_headers.set_raw("MIME-Version", vec![b"1.0".to_vec()]);
                    }

                    let mut all_nodes = vec![];
                    if let Some(file_src) = tc.as_ref().find_text("file_src") {
                        if let Ok(f) = File::open(&file_src).await {
                            if let Some(nodes) = read_multipart_body(
                                &mut f.try_into_std().ok().expect("file should convert"),
                                &multipart_headers,
                                true,
                            )
                            .ok()
                            {
                                for node in nodes {
                                    all_nodes.push(node);
                                }
                            }
                        }
                    }

                    for node in all_nodes {
                        if let Node::File(file) = node {
                            match tokio::fs::read_to_string(&file.path).await {
                                Ok(content) => {
                                    let mut full_content = String::default();
                                    for line in content.lines() {
                                        write!(full_content, "{}", line).ok().expect("should work");
                                    }

                                    if let Some(file_dst) = file.filename().ok().and_then(|f| f) {
                                        tc.as_mut()
                                            .with_text("file_dst", file_dst)
                                            .add_binary_attr("content", full_content.as_bytes());
                                    }
                                }
                                Err(_) => {

                                },
                            }
                        }
                    }
                }

                Some(tc)
            }
        })
    }
}

/// Parse the boundary value from a multipart content type
#[derive(Logos, Debug, Hash, Clone, PartialEq, PartialOrd)]
enum MimeHeaders {
    #[token("Content-Type: multipart/mixed; boundary=\"", from_multipart_header)]
    MultipartContentType(String),
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn from_multipart_header(lex: &mut Lexer<MimeHeaders>) -> Option<String> {
    let boundary = lex.remainder().trim_end_matches("\"");

    Some(boundary.to_string())
}
