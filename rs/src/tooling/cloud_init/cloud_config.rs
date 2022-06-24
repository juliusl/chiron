use lifec::{
    plugins::{BlockContext, Project},
    AttributeGraph, RuntimeState,
};
use serde_yaml::Value;
use specs::storage::HashMapStorage;
use specs::Component;
use std::fmt::Display;

/// models the cloud-init cloud config and a selection of it's modules
#[derive(Default, Clone, Component)]
#[storage(HashMapStorage)]
pub struct CloudConfig {
    block: BlockContext,
    package_update: Option<bool>,
    package_upgrade: Option<bool>,
    packages: Option<Vec<String>>,
    runcmd: Option<Vec<String>>,
}

impl CloudConfig {
    pub fn from_yaml(config: serde_yaml::Value) -> Option<Self> {
        Some(Self {
            block: BlockContext::default(),
            package_update: {
                config["package_update"].as_bool()
            },
            package_upgrade: {
                config["package_upgrade"].as_bool()
            },
            runcmd: {
                Self::parse_string_seq(&config, "runcmd")
            },
            packages: {
                Self::parse_string_seq(&config, "packages")
            },
        })
    }

    fn parse_string_seq(config: &serde_yaml::Value, symbol: impl AsRef<str>) -> Option<Vec<String>> {
        config[symbol.as_ref()].as_sequence().and_then(|cmds| {
            Some(
                cmds.iter()
                    .filter_map(|c| c.as_str())
                    .map(|c| c.to_string())
                    .collect(),
            )
        })
    }
}

impl AsRef<AttributeGraph> for CloudConfig {
    fn as_ref(&self) -> &AttributeGraph {
        self.block.as_ref()
    }
}

impl AsMut<AttributeGraph> for CloudConfig {
    fn as_mut(&mut self) -> &mut AttributeGraph {
        self.block.as_mut()
    }
}

impl From<AttributeGraph> for CloudConfig {
    fn from(graph: AttributeGraph) -> Self {
        let block = BlockContext::from(graph);
 
        Self {
            block,
            runcmd: None,
            packages: None,
            package_update: None,
            package_upgrade: None,
        }
    }
}

impl Display for CloudConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        if let Some(package_update) = self.package_update {
            writeln!(f, "package_update: {}", package_update)?;
        }

        if let Some(package_upgrade) = self.package_upgrade {
            writeln!(f, "package_upgrade: {}", package_upgrade)?;
        }

        if let Some(packages) = &self.packages {
            writeln!(f, "packages:")?;
            for package in packages.iter() {
                writeln!(f, " - {}", package)?;
            }
        }

        if let Some(runcmd) = &self.runcmd {
            writeln!(f, "runcmd:")?;
            for command in runcmd.iter() {
                writeln!(f, " - {}", command)?;
            }
        }

        Ok(())
    }
}

impl RuntimeState for CloudConfig {
    type Dispatcher = AttributeGraph;
}
