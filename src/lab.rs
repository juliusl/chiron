use crate::{create_runtime, design::Design, host::Host};
use futures_util::StreamExt;
use lifec::{
    editor::RuntimeEditor,
    plugins::{Plugin, Project, ThunkContext},
    Runtime,
};
use lifec_poem::WebApp;
use poem::{
    endpoint::EmbeddedFilesEndpoint,
    get, handler,
    web::{
        websocket::{Message, WebSocket},
        Data, Html, Path,
    },
    EndpointExt, IntoResponse, Route,
};

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
            .at("/lab/:name", get(lab))
            .at("/labs", get(labs))
            .at("/dispatch/:name", get(dispatch.data(self.0.clone())))
    }
}

#[handler]
fn dispatch(
    Path(name): Path<String>,
    ws: WebSocket,
    sender: Data<&ThunkContext>,
) -> impl IntoResponse {
    let sender = sender.clone();
    ws.on_upgrade(move |socket| async move {
        let (_, mut stream) = socket.split();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(mut text) = msg {
                    eprintln!("{name} dispatched a message: \n{text}");
                    let proxy_message = format!("{text}\nadd proxy .enable");

                    sender.dispatch(proxy_message).await;
                }
            }
        });
    })
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
fn labs() -> String {
    Design::labs().join("\n")
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
