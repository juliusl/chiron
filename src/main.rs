use serde_yaml::Value;

fn main() {
    let config = yaml(r#"
tools:
- az_cli:
  - az create vm
  - az create network
- cloud_init:
  - install-kind.yml:jinja2
"#);

    let installed_tools = vec![
        AzCli{}.install(), 
        CloudInit{}.install()
    ];

    let referenced_tools = parse_tools(config, installed_tools);

    // prints [Tool { name: "az_cli", data: ["az create vm", "az create network"] }, Tool { name: "cloud_init", data: ["install-kind.yml:jinja2"] }]
    println!("{:?}", referenced_tools)
}


/// Parse a YAML definition blob and extract tool settings
pub fn parse_tools(object: serde_yaml::Value, installed: Vec<&str>) -> Vec<Tool> {
    let mut referenced_tools: Vec<Tool> = vec![];
    
    if let Some(tools) = object.get("tools").and_then(Value::as_sequence) {
        for t in tools {
            if let Some(tool) = installed.iter().find(|v| t.get(**v).is_some()) {
                if let Some(settings) = t.get(*tool).and_then(Value::as_sequence) {
                  let settings: Vec<String> = settings.iter()
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

fn yaml(i: &str) -> serde_yaml::Value { serde_yaml::from_str(i).unwrap() }

struct Component {
    id: String,
    display_name: Option<String>,
    description: Option<String>,
    depends_on: Vec<String>,
    tools: Vec<Tool>,
    requires: Vec<SettingData>,
}

#[derive(Debug)]
pub struct Tool {
    name: String,
    data: Vec<String>,
}

struct SettingData {
    name: String,
    value: Option<String>
}


/// Built in az cli tool 
struct AzCli;

impl Tooling for AzCli {
    fn install(&self) -> &str {
        "az_cli"
    }

    fn init(&self, tool: Tool) -> Self {
        todo!()
    }
}

/// Built in cloud init tool
struct CloudInit;

impl Tooling for CloudInit {
    fn install(&self) -> &str {
        "cloud_init"
    }

    fn init(&self, tool: Tool) -> Self {
        todo!()
    }
}

trait Tooling {
    /// install returns the setting key for this tool
    fn install(&self) -> &str;

    /// initialize tooling from the tool settings
    fn init(&self, tool: Tool) -> Self;
}