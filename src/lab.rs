use std::{collections::BTreeMap, path::PathBuf};

use crate::{create_runtime, design::Design, host::Host};
use futures_util::StreamExt;
use lifec::{
    editor::RuntimeEditor,
    plugins::{Expect, Plugin, Project, ThunkContext},
    AttributeGraph, Runtime, RuntimeDispatcher, Value,
};
use lifec_poem::WebApp;
use poem::{
    endpoint::EmbeddedFilesEndpoint,
    get, handler,
    web::{
        websocket::{Message, WebSocket},
        Data, Html, Json, Path,
    },
    EndpointExt, IntoResponse, Route,
};
use serde::{Deserialize, Serialize};

/// Lab component hosts a portal for browsing .runmd in the design folder
#[derive(Default)]
pub struct Lab(ThunkContext);

impl Plugin<ThunkContext> for Lab {
    fn symbol() -> &'static str {
        "lab"
    }

    fn description() -> &'static str {
        "Starts a lab runtime w/ {project_src}"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|cancel_source| {
            let mut tc = context.clone();
            async move {
                if let Some(project_src) = tc.as_ref().find_text("project_src") {
                    if let Some(project) = Project::load_file(project_src) {
                        let block_name = tc.block.block_name.to_string();
                        if let Some(address) = tc.as_ref().find_text("address") {
                            let project =
                                project.with_block(&tc.block.block_name, "app_host", |c| {
                                    c.add_text_attr("address", &address);
                                });

                            let log = format!("Starting lab on {address}/{block_name}");

                            tc.update_status_only(&log).await;
                            eprintln!("{log}");

                            let runtime = create_runtime(project);
                            let runtime_editor = RuntimeEditor::new(runtime);
                            let mut extension = Host::from(runtime_editor);

                            tc.as_mut().add_bool_attr("proxy_dispatcher", true);

                            Runtime::start_with(&mut extension, Lab::symbol(), &tc, cancel_source);
                        }
                    }
                }

                Some(tc)
            }
        })
    }
}

impl WebApp for Lab {
    fn create(tc: &mut ThunkContext) -> Self {
        Self(tc.clone())
    }

    fn routes(&mut self) -> poem::Route {
        Route::new()
            .nest("/.run", EmbeddedFilesEndpoint::<Design>::new())
            .at("/:lab_name", get(index))
            .at("/lab/:name", get(lab.data(self.0.clone())))
            .at("/lab/:name/status", get(lab_status.data(self.0.clone())))
            .at("/labs", get(labs.data(self.0.clone())))
            .at("/dispatch/:name", get(dispatch.data(self.0.clone())))
    }
}

#[handler]
fn dispatch(
    Path(name): Path<String>,
    ws: WebSocket,
    dispatcher: Data<&ThunkContext>,
) -> impl IntoResponse {
    let dispatcher = dispatcher.clone();
    ws.on_upgrade(move |socket| async move {
        let (_, mut stream) = socket.split();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(mut text) = msg {
                    eprintln!("{name} dispatched a message: \n{text}");
                    let proxy_message = format!("{text}\nadd proxy .enable");

                    dispatcher.dispatch(proxy_message).await;
                }
            }
        });
    })
}

#[handler]
async fn lab(Path(name): Path<String>, dispatcher: Data<&ThunkContext>) -> String {
    if let Some(lab) = Design::get(format!("{name}/.runmd").as_str()) {
        match String::from_utf8(lab.data.to_vec()) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("{err}");
                String::default()
            }
        }
    } else if let Some(lab) = dispatcher.as_ref().find_binary(&name) {
        String::from_utf8(lab).ok().unwrap_or_default()
    } else if let Some(lab_dir) = dispatcher.as_ref().find_text("lab_dir") {
        tokio::fs::read_to_string(PathBuf::from(lab_dir).join(name).join(".runmd"))
            .await
            .ok()
            .unwrap_or_default()
    } else {
        String::default()
    }
}

#[derive(Default, Deserialize, Serialize)]
struct LabStatus {
    overview: String,
    expectations: Vec<String>,
}

#[handler]
async fn lab_status(Path(name): Path<String>, dispatcher: Data<&ThunkContext>) -> Json<LabStatus> {
    if let Some(_lab) = Design::get(format!("{name}/.runmd").as_str()) {
        match String::from_utf8(_lab.data.to_vec()) {
            Ok(content) => {
                let graph = AttributeGraph::from(0);
                let graph = graph.batch(content).ok().unwrap_or_default();
                let project = Project::from(graph);
                let mut status = LabStatus::default();
                if let Some(block) = project.find_block(name) {
                    if let Some(lab_block) = block.get_block("lab") {
                        let overview = lab_block.find_text("overview").unwrap_or_default();
                        status.overview = overview;
                    }
                }

                for (block_name, block) in project.iter_block() {
                    if let Some(mut expect) = block.get_block("expect") {
                        let mut deps = BTreeMap::<String, String>::default();

                        for (name, value) in expect.clone().find_symbol_values("which") {
                            if Expect::should_expect(name, "which") {
                                if let Value::TextBuffer(dep) = value {
                                    deps.insert(dep, "ok".to_string());
                                }
                            }
                        }

                        let mut dispatcher = dispatcher.clone();
                        *dispatcher.as_mut() = expect;
                        if let Some(task) = Expect::call_with_context(&mut dispatcher) {
                            match task.0.await {
                                Ok(result) => {
                                    if let Some(error_context) = result.get_errors() {
                                        for (name, error) in error_context.errors() {
                                            if let Some(prev) =
                                                deps.insert(name.to_string(), error.to_string())
                                            {
                                                eprintln!("{name} {prev} -> {error}");
                                            }
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }

                        for (name, dep_status) in deps {
                            status
                                .expectations
                                .push(format!("{block_name} - {name} {dep_status}"));
                        }
                    }
                }

                Json(status)
            }
            Err(err) => {
                eprintln!("{err}");
                Json(LabStatus::default())
            }
        }
    } else {
        Json(LabStatus::default())
    }
}

#[handler]
async fn labs(dispatcher: Data<&ThunkContext>) -> String {
    let mut builtin = Design::labs();

    let mut labs : Vec<String> = dispatcher.as_ref().iter_attributes().filter_map(|a| match a.value() {
        Value::BinaryVector(_) => Some(a.name().to_string()),
        _ => None,
    }).collect();

     builtin.append(&mut labs);

    if let Some(lab_dir) = dispatcher.as_ref().find_text("lab_dir") {
        builtin.append(&mut Design::find_labs(lab_dir).await);
    }

    builtin.join("\n")
}

#[handler]
fn index(Path(lab_name): Path<String>) -> Html<String> {
    let html = format!(
        r###"
<!DOCTYPE HTML>
<html>

<head>
	<meta charset="UTF-8">
	<style>
		body {{
			padding: 0;
			margin: 0;
		}}
	</style>
	<script src=".run/{lab_name}/portal.js"></script>
</head>

<body>
	<main></main>
    <script src="https://unpkg.com/requirejs"></script>
	<script src="https://unpkg.com/monaco-editor@0.33.0/min/vs/loader.js"></script>
	<script>
		require.config({{ paths: {{ 'vs': 'https://unpkg.com/monaco-editor@0.33.0/min/vs' }} }});
		window.MonacoEnvironment = {{ getWorkerUrl: () => proxy }};

		let proxy = URL.createObjectURL(new Blob(
		[`
	  		self.MonacoEnvironment = {{
		  		baseUrl: 'https://unpkg.com/monaco-editor@0.33.0/min/'
	  		}};
	  		importScripts('https://unpkg.com/monaco-editor@0.33.0/min/vs/base/worker/workerMain.js');
		`], 
	  	 {{ 
			type: 'text/javascript' 
		}}));
		
		customElements.define('code-editor', class extends HTMLElement {{
			constructor() {{ super(); }}
			connectedCallback() {{ this.init(); }}
			attributeChangedCallback() {{ this.init(); }}
			static get observedAttributes() {{ return ['value', 'language']; }}

			init() {{
				this.style.display = "inline-block";
				this.style.height = "100%";
				this.style.width = "100%";
				require(["vs/editor/editor.main"], this.create.bind(this));
			}}

			create() {{
				const value = this.getAttribute('value');
				const language = this.getAttribute('language');
				if (this.editor === undefined) {{
					this.editor = monaco.editor.create(this, {{ value: value, language: language, theme: 'vs-dark' }});
				}} else {{
					this.editor.getModel().setValue(value);
					monaco.editor.setModelLanguage(this.editor.getModel(), language);
				}}
				this.editor.layout();
			}}
		}});

        let ws  = new WebSocket("ws://localhost:3000/dispatch/{lab_name}");
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

        app.ports.dispatchRunmd.subscribe(function (message) {{
            ws.send(message);
        }})
	</script>
</body>
</html>
"###
    );
    Html(html)
}
