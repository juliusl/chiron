use rust_embed::RustEmbed;
use tracing::event;
use tracing::Level;

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

        event!(Level::DEBUG, "searching for labs in {}", dir.as_ref());
        if let Some(mut read_dir) = tokio::fs::read_dir(dir.as_ref()).await.ok() {
            while let Ok(Some(entry))  = read_dir.next_entry().await  {
                event!(Level::TRACE, "enumerated {:?}", entry);

                match entry.file_type().await {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            if let Some(mut read_dir) = tokio::fs::read_dir(entry.path()).await.ok() {
                                while let Ok(Some(inner_entry)) = read_dir.next_entry().await {
                                    event!(Level::TRACE, "enumerated {:?}", inner_entry);
                                    if let Some(_) = inner_entry.path().to_str().filter(|s| s.ends_with("/.runmd")) {
                                        let path = entry.file_name().to_str().unwrap_or_default().to_string();
                                        labs.push(format!("{path}/.runmd"));
                                    }
                                }
                            }
                        } else {   
                            if let Some(entry) = entry.path().to_str().filter(|s| s.ends_with("/.runmd")) {
                                labs.push(entry.to_string());
                            }
                        }
                    },
                    Err(err) => {
                        event!(Level::ERROR, "error searching dir {err}");
                    },
                }
             
            }
        }

        labs
    }
}