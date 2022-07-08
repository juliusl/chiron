use lifec::{
    editor::{RuntimeEditor, Call},
    plugins::{Plugin, Project, ThunkContext},
    Runtime,
};
use lifec_poem::{WebApp, StaticFiles, AppHost};
use poem::{handler, get, web::Path};
use crate::{
    host::Host, create_runtime,
};

/// Lab component hosts a runtime
#[derive(Default)]
pub struct Lab (
    StaticFiles,
);

impl Plugin<ThunkContext> for Lab {
    fn symbol() -> &'static str {
        "lab"
    }

    fn description() -> &'static str {
        "Starts a lab runtime w/ {project_src}"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|cancel_source| {
            let tc = context.clone();
            async move {
                if let Some(project_src) = tc.as_ref().find_text("project_src") {
                    if let Some(project) = Project::load_file(project_src) {
                        let mut runtime = create_runtime(project);
                        runtime.install::<Call, AppHost<Lab>>();
                        let mut extension = Host(
                            RuntimeEditor::new(runtime), 
                            false
                        );

                        eprintln!("{}", tc.block.block_name);

                        let block_symbol = "lab";
                        Runtime::start_with(
                            &mut extension, 
                            block_symbol, 
                            &tc, 
                            cancel_source
                        );
                    }
                }

                Some(tc)
            }
        })
    }
}

#[handler]
async fn lab(Path(name): Path<String>) -> String {
    match tokio::fs::read_to_string(format!(".run/{name}/.runmd")).await {
        Ok(content) => {
            content
        },
        Err(err) => {
            eprintln!("{err}");
            String::default()
        },
    }
}

impl WebApp for Lab {
    fn create(context: &mut ThunkContext) -> Self {
        Self(StaticFiles::create(context))
    }

    fn routes(&mut self) -> poem::Route {
        let Self(lab_file) = self;

        lab_file
            .routes()
            .at("/lab/:name", get(lab))
    }
}