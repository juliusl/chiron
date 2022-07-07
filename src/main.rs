use imgui::Window;
use lifec::{
    plugins::{Project, OpenFile, WriteFile, Process, Timer, Config}, 
    editor::Call,
};
use lifec_poem::{StaticFiles, WebApp, AppHost};
use lifec::*;
use poem::{handler, web::Path, Route, get};
use shinsu::NodeEditor;
use std::env;

mod cloud_init;
use cloud_init::{MakeMime, Install};
use cloud_init::ReadMime;

mod host;
use host::Host;

mod elm;
use elm::MakeElm;

fn main() {
    if let Some(project) = Project::runmd() {
        let mut runtime = Runtime::new(project.clone());
        runtime.install::<Call, Timer>();
        runtime.install::<Call, Process>();
        runtime.install::<Call, OpenFile>();
        runtime.install::<Call, WriteFile>();
        runtime.install::<Call, Install>();
        runtime.install::<Call, MakeMime>();
        runtime.install::<Call, ReadMime>();
        runtime.install::<Call, StaticFiles>();
        runtime.install::<Call, AppHost<Empty>>();
        runtime.install::<Call, MakeElm>();

        runtime.add_config(Config("cloud_init", |tc|{ 
            tc.as_mut()
                .with_text("work_dir", ".config/cloud_init")
                .with_text("node_title", "Install cloud_init parts")
                .add_text_attr("src_dir", "lib");
        }));

        runtime.add_config(Config("cloud_init_exit", |tc|{ 
            tc.as_mut()
                .with_text("work_dir", ".config/cloud_init")
                .with_text("node_title", "Install cloud_init exit")
                .with_text("src_dir", "lib")
                .add_text_attr("src_type", "exit");
        }));

        runtime.add_config(Config("cloud_init_enter", |tc|{ 
            tc.as_mut()
                .with_text("work_dir", ".config/cloud_init")
                .with_text("node_title", "Install cloud_init enter")
                .with_text("src_dir", "lib")
                .add_text_attr("src_type", "enter");
        }));

        runtime.add_config(Config("elm_portal", |tc| {
            tc.as_mut()
                .with_text("elm_src", "lib/elm/portal/src/Main.elm")
                .add_text_attr("elm_dst", "lib/elm/portal/portal.js");
        }));

        let args: Vec<String> = env::args().collect();
        
        if let Some(arg) = args.get(1) {
            if arg == "--host" {
                start(
                Host::from(runtime), 
                &[
                    "app",
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
    fn configure_app_world(world: &mut World) {
        NodeEditor::configure_app_world(world);
        Host::configure_app_world(world);
    }

    fn configure_app_systems(dispatcher: &mut DispatcherBuilder) {
        NodeEditor::configure_app_systems(dispatcher);
        Host::configure_app_systems(dispatcher);
    }

    fn on_window_event(&'_ mut self, app_world: &World, event: &'_ lifec::editor::WindowEvent<'_>) {
        self.0.on_window_event(app_world, event);
        self.1.on_window_event(app_world, event);
    }

    fn on_ui(&'_ mut self, app_world: &World, ui: &'_ imgui::Ui<'_>) {
        self.0.on_ui(app_world, ui);

        Window::new("Chiron Tools")
            .menu_bar(true)
            .size([800.0, 600.0], imgui::Condition::Appearing)
            .build(ui, ||{
                self.1.on_ui(app_world, ui);
            });
    }

    fn on_run(&'_ mut self, app_world: &World) {
        self.0.on_run(app_world);
        self.1.on_run(app_world);
    }
    
    fn on_maintain(&'_ mut self, app_world: &mut World) {
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

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

impl WebApp for Empty {
    fn create(_: &mut plugins::ThunkContext) -> Self {
        Empty{}
    }

    fn routes(&mut self) -> poem::Route {
        Route::new().at("/hello/:name", get(hello))
    }
}