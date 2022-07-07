use lifec::{
    plugins::{combine, OpenFile, Plugin, ThunkContext, WriteFile},
    Component, DenseVecStorage,
};

/// Installs a file
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Install;

impl Plugin<ThunkContext> for Install {
    fn symbol() -> &'static str {
        "install"
    }

    fn description() -> &'static str {
        "Loads an install config from {src_dir}/{tool_name}/{src_type}-{block_name}.{ext}"
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
        } else if let Some(_) = context.as_ref().find_text("file_src") {
            return combine::<OpenFile, WriteFile>()(context);
        }

        eprintln!("install skipped");
        None
    }
}
