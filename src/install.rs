use std::path::PathBuf;

use lifec::{
    plugins::{combine, OpenFile, Plugin, ThunkContext, WriteFile},
    Component, DenseVecStorage, Resources,
};
use rust_embed::RustEmbed;

/// Installs a file
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Install;


#[derive(RustEmbed)]
#[folder = "lib/sh"]
struct Shell;

impl Plugin<ThunkContext> for Install {
    fn symbol() -> &'static str {
        "install"
    }

    fn description() -> &'static str {
        "Loads an install config from `{src_dir}/{tool_name}/{src_type}-{block_name}.{ext}`"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        let block_name = context.block.block_name.to_string();

        if let Some(src_dir) = context.as_ref().find_text("src_dir") {
            if let Some(tool_name) = context.as_ref().find_text("tool_name") {
                if let Some(ext) = context.as_ref().find_text("ext") {
                    let src_type = context
                        .as_ref()
                        .find_text("src_type")
                        .unwrap_or("install".to_string());

                    let file_src = format!("{src_dir}/{tool_name}/{src_type}-{block_name}.{ext}");
                    context.as_mut().add_text_attr("file_src", file_src);

                    return combine::<OpenFile, WriteFile>()(context);
                }
            }
        } else if let Some(file_src) = context.as_ref().find_text("file_src") {

            if let Some(handle) = context.handle() {
                handle.block_on(async {
                    Self::resolve_lib_file(context, file_src).await;
                });
            }

            return combine::<OpenFile, WriteFile>()(context);
        }

        eprintln!("install skipped");
        None
    }
}


impl Install {
    async fn resolve_lib_file(context: &mut ThunkContext, file_src: impl AsRef<str>) {
        let path_buf = PathBuf::from(file_src.as_ref());
        if !path_buf.exists() {
            if file_src.as_ref().starts_with("lib/sh") {
                let file_name = file_src.as_ref().trim_start_matches("lib/sh/");
                if let Some(file) = Resources("lib/sh").read_binary::<Shell>(context, &file_name.to_string()).await {
                    context.as_mut().add_binary_attr("content", file.to_vec());
                }
            }
        }
    }
}