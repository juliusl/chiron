use lifec::Component;
use lifec::DenseVecStorage;
use lifec::plugins::Plugin;
use lifec::plugins::ThunkContext;

use super::find_parts;

/// Creates an installer using cloud_init parts
///
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Installer;

impl Plugin<ThunkContext> for Installer {
    fn symbol() -> &'static str {
        "installer"
    }

    fn description() -> &'static str {
        "Assembles an installer for a group of clout_init parts."
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|_| {
            let tc = context.clone();
            async move {
                let parts = find_parts(&tc).await;
                for part in parts {
                    match tokio::fs::read_to_string(part).await {
                        Ok(content) => {
                            let mut all_packages = vec![];
                            let mut all_runcmds = vec![];
                            if let Some(part) = serde_yaml::to_value(content.as_str()).ok() {
                                // Since parts need to be dispatched in order, these parts should already end up being sorted
                                // Even if this is not the case, the parts themselves should be fairly idempotent, that is,
                                // if the desired state already exists, calling the particular command results in a no-op
                                
                                // Parse packages
                                if let Some(packages) = part.get("packages") {
                                    if packages.is_sequence() {
                                        if let Some(packages) = packages.as_sequence() {
                                            for cmd in packages.iter().filter_map(|c| c.as_str()) {
                                                all_packages.push(cmd);
                                            }
                                        }
                                    }
                                }

                                // Parse write files
                                if let Some(write_files) = part.get("write_files") {
                                    if write_files.is_sequence() {
                                        if let Some(write_file) = write_files.as_sequence() {
                                            for file in write_file.iter().filter_map(|w| w.as_mapping()) {
                                                if let (Some(content), Some(path)) = (
                                                    file.get(&serde_yaml::to_value("content").expect(""))
                                                        .and_then(|v| v.as_str()), 
                                                    file.get(&serde_yaml::to_value("path").expect(""))
                                                        .and_then(|v| v.as_str())
                                                ) {
                                                    
                                                }
                                            }
                                        }
                                    }
                                }

                                // Parse runcmd
                                if let Some(runcmd) = part.get("runcmd") {
                                    if runcmd.is_sequence() {
                                        if let Some(runcmd) = runcmd.as_sequence() {
                                            for cmd in runcmd.iter().filter_map(|c| c.as_str()) {
                                                all_runcmds.push(cmd);
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(_) => todo!(),
                    }
                }

                None 
            }
        })
    }
}
