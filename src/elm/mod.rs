mod make;
use lifec::plugins::ThunkContext;
pub use make::MakeElm;

pub fn env(context: &mut ThunkContext) {
    context.as_mut()
        .with_text("tool_name", "elm")
        .with_text("work_dir", ".config/elm")
    .add_text_attr("src_dir", "lib");
}