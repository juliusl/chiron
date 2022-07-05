use std::env::{self};

use lifec::{
    plugins::{Project, OpenFile, WriteFile, Process}, 
    editor::Call,
    open, 
    Runtime, 
    start
};


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
                "host"
                );
            }
        } else {
            open("chiron", 
            Runtime::new(project.clone()), 
            Host::from(runtime)
            )
        }
    }
}
