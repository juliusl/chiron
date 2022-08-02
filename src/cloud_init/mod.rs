mod make_mime;
use std::{path::PathBuf, str::FromStr};

mod installer;
pub use installer::Installer;

use lifec::plugins::ThunkContext;
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

/// Iterate cloud_init parts in the thunk_context
/// 
/// Caveats: returns only parts that exist
pub async fn find_parts(context: &ThunkContext) -> Vec<String> {
    let mut parts = vec![];
    for (_, part_value) in context.as_ref().find_symbol_values("part") {
        if let lifec::Value::TextBuffer(part_value) = part_value {
            context.update_status_only(format!("adding {part_value}")).await;
            parts.push(part_value);
        }
    }
    
    parts.iter()
        .filter_map(|p| PathBuf::from_str(p.as_str()).ok())
        .filter(|p| p.exists())
        .filter_map(|p| p.to_str().and_then(|s| Some(s.to_string())))
        .collect()
}