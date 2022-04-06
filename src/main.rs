use serde_yaml::Value;

fn main() {
    let object = yaml(r#"
tools:
- az_cli:
  - az create vm
- cloud_init:
  - install-kind.yml:jinja2
"#);

    if let Some(tools) = object.get("tools").and_then(|f| f.as_sequence()) {
        for t in tools {
            if let Some(tool) = t.get("az_cli") {
                if let Some(cmds) = tool.as_sequence() {
                    println!("az-cli: {:?}", cmds);
                }
            } else if let Some(tool) = t.get("cloud_init") {
                if let Some(cmds) = tool.as_sequence() {
                    println!("cloud-init: {:?}", cmds);
                }
            }
        }
    }
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

struct Tool {
    name: String,
    data: Vec<String>,
}

struct SettingData {
    name: String,
    value: Option<String>
}

trait Tooling {
    fn init(tool: Tool) -> Self;

    fn output(&self) -> Self;
}