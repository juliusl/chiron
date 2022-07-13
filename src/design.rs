use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "design"]
pub struct Design;


impl Design {
    pub fn labs() -> Vec<String> {
        Design::iter()
            .filter(|p| p.ends_with("/.runmd"))
            .map(|s| s.to_string())
            .collect()
    }
}