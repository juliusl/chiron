use std::{fs, path::Path};

use serde_yaml::Value;

/// Tool's are initialized by a list of strings which the tool will interpret to initialize it's state
#[derive(Debug)]
pub struct Tool {
    name: String,
    data: Vec<String>,
}

pub trait Tooling {
    /// returns the symbol used for this tool, must be valid yaml
    fn tool_symbol() -> &'static str;

    /// installs this tool to the user's home directory
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

    /// Ensures a directory exists, and then passes back the directory
    fn ensure_dir<T: AsRef<Path>>(root: T) -> String {
        let user_tool_dir = root
            .as_ref()
            .join(Self::tool_symbol())
            .to_str()
            .unwrap()
            .to_owned();

        if let Err(e) = fs::create_dir_all(&user_tool_dir) {
            panic!("{}", e);
        }

        user_tool_dir
    }

    /// Parses the tools field in the config
    fn parse_tools(object: serde_yaml::Value, installed: Vec<&str>) -> Vec<Tool> {
        let mut referenced_tools: Vec<Tool> = vec![];

        let tools = &object["tools"];

        for tool in installed {
            if let Some(tools) = tools[tool].as_sequence() {
                let settings: Vec<String> = tools
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect();

                referenced_tools.push(
                    Tool {
                        name: tool.to_string(),
                        data: settings,
                    });
            }
        }

        referenced_tools
    }

    /// Converts a string to a serde_yaml object
    fn yaml(i: &str) -> serde_yaml::Value {
        serde_yaml::from_str(i).expect("string was not valid yaml")
    }
}
