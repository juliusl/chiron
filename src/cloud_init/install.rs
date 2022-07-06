use lifec::{plugins::{ThunkContext, Plugin, OpenFile}, Component, DenseVecStorage};

/// Component that gets an install fragment from lib/cloud_init
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Install; 

impl Plugin<ThunkContext> for Install {
    fn symbol() -> &'static str {
        "install"
    }

    fn description() -> &'static str {
        "Opens an install config from {src_dir}/cloud_init"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        let block_name =  context.block.block_name.to_string();
        
        if let Some(src_dir) = context.as_ref().find_text("src_dir") {
            let src_type = context.as_ref().find_text("src_type").unwrap_or("install".to_string());
            let file_src = format!("{}/cloud_init/{}-{}.yml", src_dir, src_type, block_name);
            context.as_mut()
                .add_text_attr("file_src", file_src);
            
            OpenFile::call_with_context(context)
        } else {
            None 
        }
    }
}
