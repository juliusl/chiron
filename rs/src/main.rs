use std::{panic, path::Path};

mod helpers;
mod tooling;

use tooling::Tooling;
use tooling::cloud_init::CloudInit;

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
}
