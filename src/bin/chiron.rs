use chiron::{create_runtime, Host};
use clap::{Args, Parser, Subcommand};
use imgui::Window;
use lifec::{plugins::Project, Runtime, open, combine, Extension, World, DispatcherBuilder, App, System};
use lifec_shell::Shell;
use shinsu::NodeEditor;
use tracing::{event, Level};
use std::{path::PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[clap(name = "chiron")]
#[clap(about = "Developer tool, for building interactive scripts and labs.", long_about = None)]
struct Cli {
    /// If no subcommand is passed, starts the tool gui if possible
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initializes a chiron template
    #[clap(arg_required_else_help = true)]
    Init,
    /// Starts the runtime by loading a project .runmd file and passing the names of each engine block to start.
    Start(Start),
}

#[derive(Debug, Args)]
struct Start {
    /// Path to a .runmd file, Defaults to .runmd in the current directory
    #[clap(long, short)]
    project_src: Option<String>,
    /// Engine block names to start. The blocks must be defined in the .runmd project file.
    engines: Vec<String>,
}

fn main() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();

    let cli = Cli::parse();

    match cli {
        Cli {
            command: Some(Commands::Start(start)),
        } => {
            let Start {
                project_src,
                engines,
            } = start;

            let project = if let Some(project_src) = project_src {
                let project_src_path = PathBuf::from(&project_src);
                if project_src_path.exists() {
                    Project::load_file(project_src)
                } else {
                    event!(Level::INFO, "Trying to load project from .runmd");
                    Project::runmd()
                }
            } else {
                event!(Level::INFO, "Trying to load project from .runmd");
                Project::runmd()
            };

            if let Some(project) = project {
                let runtime = create_runtime(project);
                lifec::start(Host::from(runtime), engines);
            } else {
                event!(Level::ERROR, "Did not find any project src");
            }
        }
        Cli {
            command: Some(Commands::Init),
        } => {
            eprintln!("init called");
        }
        _ => {
            if let Some(project) = Project::runmd() {
                let runtime = Runtime::new(project);
                open(
                    "chiron",
                    Empty,
                    combine(
                        Main(Host::from(runtime), NodeEditor::default()),
                        Shell::default(),
                    ),
                )
            }
        }
    }

    return;
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
            .build(ui, || {
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

    fn edit_ui(&mut self, _: &imgui::Ui) {}

    fn display_ui(&self, _: &imgui::Ui) {}
}

impl<'a> System<'a> for Empty {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {}
}
