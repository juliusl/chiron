use lifec::plugins::{Plugin, ThunkContext, Process};

#[derive(Default)]
pub struct MakeElm;

impl Plugin<ThunkContext> for MakeElm {
    fn symbol() -> &'static str {
        "make_elm"
    }

    fn description() -> &'static str {
        "Compiles an elm file from a text attribute `elm_src` and outputs the .js file to `elm_dst`"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        if let Some(elm_src) = context.as_ref().find_text("elm_src") {
            if let Some(elm_dst) = context.as_ref().find_text("elm_dst") {

                context.as_mut().with_text("command", format!("elm make {elm_src} --output {elm_dst}"));

                return Process::call_with_context(context);
            }
        }
        
        None
    }
}