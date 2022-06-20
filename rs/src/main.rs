use std::{panic, path::Path};

mod helpers;
mod tooling;

use specs::{World, DispatcherBuilder};
use tooling::Tooling;
use tooling::cloud_init::CloudInit;

use lifec::plugins::demos::NodeDemo;
use lifec::{editor::*, AttributeGraph, Runtime};

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

    if let Some(file) = AttributeGraph::load_from_file(".runmd") {
        open(
            "demo",
            RuntimeEditor::new(Runtime::from(file)),
            NodeDemo::default(),
        );
    }
}

struct Chiron;

impl Extension for Chiron {
    fn configure_app_world(world: &mut World) {
        NodeDemo::configure_app_world(world);
    }

    fn configure_app_systems(dispatcher: &mut DispatcherBuilder) {
        NodeDemo::configure_app_systems(dispatcher);
    }

    fn on_ui(&'_ mut self, app_world: &World, ui: &'_ imgui::Ui<'_>) {
        
    }
}