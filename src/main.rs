use lifec_hyper::HyperContext;
use lifec_poem::{StaticFiles, AppHost};
use lifec::{
    plugins::{Project, OpenFile, WriteFile, Process, Timer, Config, Println, OpenDir, Remote, Expect, Missing, Redirect}, 
    editor::{Call, Fix},
    *
};
use lifec_registry::{Authenticate, Login, Resolve, MirrorHost};
use lifec_shell::Shell;
use shinsu::NodeEditor;
use imgui::Window;
use tracing_subscriber::EnvFilter;
use std::env;

mod cloud_init;
use cloud_init::MakeMime;
use cloud_init::ReadMime;
use cloud_init::Installer;

mod install;
use install::Install;

mod host;
use host::Host;

mod lab;
use lab::Lab;

mod design;

mod acr;
use acr::Acr;

fn main() {
    if let Some(project) = Project::runmd() {
        let runtime = create_runtime(project);
      
        let args: Vec<String> = env::args().collect();
        
        tracing_subscriber::fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .compact()
            .init();

        if let Some(arg) = args.get(1) {
            if arg == "--host" {
                start(
                Host::from(runtime), 
                &[
                    "host",
                ]);
            } else if arg == "--mirror" {
                start(
                    Host::from(runtime),
                    &[
                        "mirror",
                    ]
                )
            }
        } else {
            open("chiron", 
            Empty, 
                combine(Main(
                    Host::from(runtime), 
                    NodeEditor::default()
                ), 
                Shell::default())
            )
        }
    }
}

fn create_runtime(project: Project) -> Runtime {
    let mut runtime = Runtime::new(project);

    // --- lifec plugins ---
    // -- Filesystem plugins
    runtime.install::<Call, WriteFile>();
    runtime.install::<Call, OpenFile>();
    runtime.install::<Call, OpenDir>();
    // -- Utility plugins
    runtime.install::<Call, Println>();
    runtime.install::<Call, Timer>();
    // -- System plugins
    runtime.install::<Call, Process>();
    runtime.install::<Call, Remote>();
    runtime.install::<Call, Expect>();
    runtime.install::<Call, Runtime>();
    runtime.install::<Call, Redirect>();
    runtime.install::<Fix, Missing>();

    // --- lifec_poem plugins ---
    // -- Hosting code
    runtime.install::<Call, StaticFiles>();
    runtime.install::<Call, AppHost<Lab>>();
    
    // --- lifec_hyper plugins ---
    // -- Client code
    // this adds a "request" plugin to make https requests
    runtime.install::<Call, HyperContext>();

    // -- lifec_registry plugins -- 
    runtime.install::<Call, Login>();
    runtime.install::<Call, Authenticate>();
    runtime.install::<Call, Resolve>();
    runtime.install::<Call, MirrorHost<Acr>>();

    // -- Cloud-init plugins --
    runtime.install::<Call, MakeMime>();
    runtime.install::<Call, ReadMime>();
    runtime.install::<Call, Installer>();

    // -- Cloud-init configs
    runtime.add_config(Config("cloud_init", |tc| {
        cloud_init::env(tc);
    }));

    runtime.add_config(Config("cloud_init_exit", |tc| {
        tc.as_mut().add_text_attr("src_type", "exit");
        cloud_init::env(tc);
    }));

    runtime.add_config(Config("cloud_init_enter", |tc| {        
        tc.as_mut().add_text_attr("src_type", "enter");
        cloud_init::env(tc);
    }));

    // --- chiron plugins ---
    runtime.install::<Call, Install>();
    runtime.install::<Call, Lab>();

    // common default configs
    runtime.add_config(Config("empty", |_| {}));
    runtime
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

    fn enable_depth_stencil<'a>(&self) -> bool {
        true
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
