mod make_mime;
use lifec::{AttributeGraph, plugins::ThunkContext};
pub use make_mime::MakeMime;

mod read_mime;
pub use read_mime::ReadMime;

mod user_data;
pub use user_data::UserData;

pub fn env(context: &mut ThunkContext) {
    context.as_mut()
        .with_text("tool_name", "cloud_init")
        .with_text("ext", "yml")
        .with_text("work_dir", ".config/cloud_init")
        .with_text("node_title", "Install cloud_init parts")
    .add_text_attr("src_dir", "lib");
}