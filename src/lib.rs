
mod cloud_init;
pub use cloud_init::Installer;
pub use cloud_init::MakeMime;
pub use cloud_init::ReadMime;

mod install;
pub use install::Install;

mod host;
pub use host::Host;

mod lab;
pub use lab::Lab;

mod design;

mod acr;
pub use acr::Acr;
use lifec::Runtime;
use lifec::editor::Call;
use lifec::editor::Fix;
use lifec::plugins::Config;
use lifec::plugins::Expect;
use lifec::plugins::Missing;
use lifec::plugins::OpenDir;
use lifec::plugins::OpenFile;
use lifec::plugins::Println;
use lifec::plugins::Process;
use lifec::plugins::Project;
use lifec::plugins::Redirect;
use lifec::plugins::Remote;
use lifec::plugins::Timer;
use lifec::plugins::WriteFile;
use lifec_hyper::HyperContext;
use lifec_poem::AppHost;
use lifec_poem::StaticFiles;
use lifec_registry::Authenticate;
use lifec_registry::Login;
use lifec_registry::MirrorHost;
use lifec_registry::Resolve;


pub fn create_runtime(project: Project) -> Runtime {
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