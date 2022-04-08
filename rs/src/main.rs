use std::ops::Deref;
use std::{fmt::Display, fs, io::Error, panic, path::Path};

use serde_yaml::Value;

pub mod helpers;
mod tooling;

use tooling::az_cli::AzCli;
use tooling::cloud_init::CloudInit;

fn main() {
    let config = 
        r#"
tools:
- cloud_init:
  - install-golang.yml:jinja2
  - install-kind.yml:jinja2
"#;

    if let Ok(home_dir) = std::env::var("HOME") {
        let home_dir = Path::new(&home_dir);

        // Install Tools
        let cloud_init = CloudInit::default().install(home_dir);

        cloud_init.init(config);
    } else {
        panic!("Could not read HOME env variable");
    }
}

/// Parse a YAML definition blob and extract tool settings
pub fn parse_tools(object: serde_yaml::Value, installed: Vec<&str>) -> Vec<Tool> {
    let mut referenced_tools: Vec<Tool> = vec![];

    if let Some(tools) = object.get("tools").and_then(Value::as_sequence) {
        for t in tools {
            if let Some(tool) = installed.iter().find(|v| t.get(**v).is_some()) {
                if let Some(settings) = t.get(*tool).and_then(Value::as_sequence) {
                    let settings: Vec<String> = settings
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect();

                    referenced_tools.push(Tool {
                        name: tool.to_string(),
                        data: settings,
                    });
                }
            }
        }
    }

    referenced_tools
}

fn yaml(i: &str) -> serde_yaml::Value {
    serde_yaml::from_str(i).unwrap()
}

struct Component {
    id: String,
    display_name: Option<String>,
    description: Option<String>,
    depends_on: Vec<String>,
    tools: Vec<Tool>,
    requires: Vec<SettingData>,
}
struct SettingData {
    name: String,
    value: Option<String>,
}

#[derive(Debug)]
pub enum ToolError {
    CouldNotCreateUserToolDir(Error),
}

impl std::error::Error for ToolError {}

impl Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Tool's are initialized by a list of strings which the tool will interpret to initialize it's state
#[derive(Debug)]
pub struct Tool {
    name: String,
    data: Vec<String>,
}

pub trait Tooling {
    /// returns the symbol used for this tool, must be valid yaml
    fn symbol() -> &'static str;

    /// install returns the setting key for this tool
    fn install<T: AsRef<Path>>(self, user_home: T) -> Self;

    /// initialize tooling from the tool settings
    fn init(self, config: &str) -> Self;

    /// Ensures a directory in the local folder is created for this directory
    fn with_local_dir<T: AsRef<Path>>(user_home: T) -> String {
        Self::ensure_dir(user_home.as_ref().join(".local/share/chiron"))
    }

    /// Ensures a directory in the cache folder is created for this directory
    fn with_cache_dir<T: AsRef<Path>>(user_home: T) -> String {
        Self::ensure_dir(user_home.as_ref().join(".cache/chiron"))
    }

    /// Ensures a directory in the config folder is created for this directory
    fn with_config_dir<T: AsRef<Path>>(user_home: T) -> String {
        Self::ensure_dir(user_home.as_ref().join(".config/chiron"))
    }

    /// Ensures a directory exists
    fn ensure_dir<T: AsRef<Path>>(root: T) -> String {
        let user_tool_dir = root
            .as_ref()
            .join(Self::symbol())
            .to_str()
            .unwrap()
            .to_owned();

        if let Err(e) = fs::create_dir_all(&user_tool_dir) {
            panic!("{}", e);
        }

        user_tool_dir
    }

    fn parse_tools(object: serde_yaml::Value, installed: Vec<&str>) -> Vec<Tool> {
        let mut referenced_tools: Vec<Tool> = vec![];

        if let Some(tools) = object.get("tools").and_then(Value::as_sequence) {
            for t in tools {
                if let Some(tool) = installed.iter().find(|v| t.get(**v).is_some()) {
                    if let Some(settings) = t.get(*tool).and_then(Value::as_sequence) {
                        let settings: Vec<String> = settings
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect();

                        referenced_tools.push(Tool {
                            name: tool.to_string(),
                            data: settings,
                        });
                    }
                }
            }
        }

        referenced_tools
    }

    fn yaml(i: &str) -> serde_yaml::Value {
        serde_yaml::from_str(i).unwrap()
    }
}
