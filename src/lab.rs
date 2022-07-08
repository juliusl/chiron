use crate::{create_runtime, host::Host, run::Run, design::Design};
use lifec::{
    editor::{Call, RuntimeEditor},
    plugins::{Plugin, Project, ThunkContext},
    Runtime,
};
use lifec_poem::{AppHost, WebApp};
use poem::{
    get, handler,
    web::{Html, Path}, Route,
    endpoint::EmbeddedFilesEndpoint
};

/// Lab component hosts a runtime for browsing .runmd in the design folder
#[derive(Default)]
pub struct Lab;

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
                        let mut extension = Host(RuntimeEditor::new(runtime), false);

                        eprintln!("{}", tc.block.block_name);

                        let block_symbol = "lab";
                        Runtime::start_with(&mut extension, block_symbol, &tc, cancel_source);
                    }
                }

                Some(tc)
            }
        })
    }
}

impl WebApp for Lab {
    fn create(_: &mut ThunkContext) -> Self {
        Self{}
    }

    fn routes(&mut self) -> poem::Route {
        Route::new()
            .nest("/.run", EmbeddedFilesEndpoint::<Run>::new())
            .at("/:lab_name", get(index))
            .at("/lab/:name", get(lab))
    }
}

#[handler]
async fn lab(Path(name): Path<String>) -> String {
    if let Some(lab) = Design::get(format!("{name}/.runmd").as_str()) {
        match String::from_utf8(lab.data.to_vec()) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("{err}");
                String::default()
            }
        }
    } else {
        String::default()
    }
}

#[handler]
fn index(Path(lab_name): Path<String>) -> Html<String> {
    let html = format!(
        r#"
<!DOCTYPE HTML>
<html>

<head>
	<meta charset="UTF-8">
	<title>Chiron Labs - {lab_name}</title>
	<style>
		body {{
			padding: 0;
			margin: 0;
		}}
	</style>
	<script src=".run/{lab_name}/elm.min.js"></script>
</head>

<body>
	<main></main>
	<script>
		var app = Elm.Main.init({{ 
            node: document.querySelector('main'),
            flags: '{lab_name}'
        }});

		app.ports.dispatchEditorCmd.subscribe(function (message) {{
			switch (message) {{
				case "save":
					let editor = document.querySelector('code-editor').editor;
					app.ports.saveContent.send(editor.getModel().getValue())
					break;
			}}
		}});
	</script>
</body>
</html>
"#
    );

    Html(html)
}
