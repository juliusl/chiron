use lifec::plugins::{ThunkContext, Plugin, Println, combine, Expect};


#[derive(Default)]
pub struct Check;

impl Plugin<ThunkContext> for Check {
    fn symbol() -> &'static str {
        "check"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        combine::<Expect, Println>()(context)
    }
}