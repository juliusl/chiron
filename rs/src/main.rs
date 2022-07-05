use imgui::Window;
use lifec::{
    plugins::{Project, OpenFile, WriteFile, Process}, 
    editor::Call,
    open, 
    start,
    App,
    Runtime,  
    Extension
};
use shinsu::NodeEditor;
use specs::{DispatcherBuilder, System};
use std::env;

mod tooling;

mod host;
use host::Host;

fn main() {
    if let Some(project) = Project::runmd() {
        let mut runtime = Runtime::new(project.clone());
        runtime.install::<Call, Process>();
        runtime.install::<Call, OpenFile>();
        runtime.install::<Call, WriteFile>();
        runtime.install::<Call, Host>();

        let args: Vec<String> = env::args().collect();
        
        if let Some(arg) = args.get(1) {
            if arg == "--host" {
                start(
                Host::from(runtime), 
                &[
                    "setup", 
                    "host"
                ]);
            }
        } else {
            open("chiron", 
            Empty, 
            Main(
                Host::from(runtime), 
                NodeEditor::default()
            ))
        }
    }
}

struct Main(Host, NodeEditor); 

impl Extension for Main {
    fn configure_app_world(world: &mut lifec::plugins::World) {
        NodeEditor::configure_app_world(world);
        Host::configure_app_world(world);
    }

    fn configure_app_systems(dispatcher: &mut DispatcherBuilder) {
        NodeEditor::configure_app_systems(dispatcher);
        Host::configure_app_systems(dispatcher);
    }

    fn on_window_event(&'_ mut self, app_world: &specs::World, event: &'_ lifec::editor::WindowEvent<'_>) {
        self.0.on_window_event(app_world, event);
        self.1.on_window_event(app_world, event);
    }

    fn on_ui(&'_ mut self, app_world: &specs::World, ui: &'_ imgui::Ui<'_>) {
        self.0.on_ui(app_world, ui);

        Window::new("Chiron Tools")
            .menu_bar(true)
            .size([800.0, 600.0], imgui::Condition::Appearing)
            .build(ui, ||{
                self.1.on_ui(app_world, ui);
            });
    }

    fn on_run(&'_ mut self, app_world: &specs::World) {
        self.0.on_run(app_world);
        self.1.on_run(app_world);
    }
    
    fn on_maintain(&'_ mut self, app_world: &mut specs::World) {
        self.0.on_maintain(app_world);
        self.1.on_maintain(app_world);
    }
}


/// TODO placeholder
struct Empty;

impl App for Empty {
    fn name() -> &'static str {
        "empty"
    }

    fn edit_ui(&mut self, _: &imgui::Ui) {
    }

    fn display_ui(&self, _: &imgui::Ui) {
    }
}

impl<'a> System<'a> for Empty {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
    }
}