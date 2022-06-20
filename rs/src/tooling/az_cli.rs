use super::Tooling;

/// Built in az cli tool 
pub struct AzCli;

impl Tooling for AzCli {
    fn install<T: AsRef<std::path::Path>>(self, user_tool_data: T) -> Self {
        todo!()
    }

    fn tool_symbol() -> &'static str {
        todo!()
    }

    fn init(self, config: &str) -> Self {
        todo!()
    }
    
}

impl Default for AzCli {
    fn default() -> Self {
        Self {  }
    }
}