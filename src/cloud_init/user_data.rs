use lifec::plugins::{Plugin, ThunkContext};

pub struct UserData;

impl Plugin<ThunkContext> for UserData {
    fn symbol() -> &'static str {
        "user_data"
    }

    fn call_with_context(_context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        todo!()
    }
}
