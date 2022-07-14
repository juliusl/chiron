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

    pub async fn find_labs(dir: impl AsRef<str>) -> Vec<String> {
        let mut labs = vec![];
        if let Some(mut read_dir) = tokio::fs::read_dir(dir.as_ref()).await.ok() {
            while let Ok(Some(entry))  = read_dir.next_entry().await  {
                if let Some(entry) = entry.file_name().to_str().filter(|s| s.ends_with("/.runmd")) {
                    labs.push(entry.to_string());
                }
            }
        }

        labs
    }
}