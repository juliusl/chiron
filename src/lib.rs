
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
use lifec::plugins::Config;
use lifec::plugins::Println;
use lifec::plugins::Process;
use lifec::plugins::Timer;
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
    runtime.install::<Println>();
    runtime.install::<Timer>();
    // -- System plugins
    runtime.install::<Process>();

    // --- lifec_poem plugins ---
    // -- Hosting code
    runtime.install::<StaticFiles>();
    runtime.install::<AppHost<Lab>>();

    // --- lifec_hyper plugins ---
    // -- Client code
    // this adds a "request" plugin to make https requests
    runtime.install::<HyperContext>();

    // -- lifec_registry plugins --
    runtime.install::<Login>();
    runtime.install::<Authenticate>();
    runtime.install::<Resolve>();
    runtime.install::<MirrorHost<Acr>>();

    // -- Cloud-init plugins --
    runtime.install::<MakeMime>();
    runtime.install::<ReadMime>();
    runtime.install::<Installer>();

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
    runtime.install::<Install>();
    runtime.install::<Lab>();

    // common default configs
    runtime.add_config(Config("empty", |_| {}));
    runtime
}