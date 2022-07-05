use imgui::{Window, MenuItem};
use lifec::{Runtime, Extension, editor::{RuntimeEditor, Call, WindowEvent}, plugins::{Plugin, ThunkContext, Connection}, AttributeGraph};
use poem::{Route, Server, endpoint::StaticFilesEndpoint, listener::TcpListener};
use specs::{Component, DispatcherBuilder, World, WorldExt};
use specs::storage::DefaultVecStorage;
use tokio::select;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;

/// Host component add's a poem server to the entity
#[derive(Default, Component)]
#[storage(DefaultVecStorage)]
pub struct Host(RuntimeEditor, bool);

impl From<Runtime> for Host {
    fn from(runtime: Runtime) -> Self {
        Host(RuntimeEditor::new(runtime), false)
    }
}

impl AsRef<Runtime> for Host {
    fn as_ref(&self) -> &Runtime {
        &self.0.runtime()
    }
}

impl Extension for Host {
    fn configure_app_world(world: &mut lifec::plugins::World) {
        RuntimeEditor::configure_app_world(world);
    }

    fn configure_app_systems(dispatcher: &mut DispatcherBuilder) {
        RuntimeEditor::configure_app_systems(dispatcher);
    }

    fn on_ui(&'_ mut self, app_world: &World, ui: &'_ imgui::Ui<'_>) {
        Window::new("Chiron Tools")
            .menu_bar(true)
            .size([800.0, 600.0], imgui::Condition::Appearing)
            .build(ui, ||{
                ui.menu_bar(|| {
                    ui.menu("Actions", ||{
                        if MenuItem::new("Create portal host").build(ui) {
                            if let Some(first) = self.0.runtime().create_engine::<Call>(app_world, "host") {
                                app_world.write_component::<Connection>()
                                    .insert(first, Connection::default()).ok();
                        }
                        if let Some(first) = self.0.runtime().create_engine::<Call>(app_world, "setup") {
                            app_world.write_component::<Connection>()
                                .insert(first, Connection::default()).ok();
                        }
                    }
                });
            });
        });

        self.0.on_ui(app_world, ui);
    }

    fn on_window_event(&'_ mut self, app_world: &World, event: &'_ lifec::editor::WindowEvent<'_>) {
        match event {
            WindowEvent::DroppedFile(path) => {
                if "runmd" == path.extension().unwrap_or_default() {
                    if let Some(file) =
                        AttributeGraph::load_from_file(path.to_str().unwrap_or_default())
                    {
                        *self.0.project_mut().as_mut() = file;
                        *self.0.project_mut() = self.0.project_mut().reload_source();
                        self.1 = true;
                    }
                }
            }
            _ => {}
        }

        self.0.on_window_event(app_world, event)
    }

    fn on_run(&'_ mut self, app_world: &lifec::plugins::World) {
        self.0.on_run(app_world);
    }

    fn on_maintain(&'_ mut self, app_world: &mut World) {
        if self.1 {
            app_world.delete_all();
            if let Some(first) = self.0.runtime().create_engine::<Call>(app_world, "host") {
                    app_world.write_component::<Connection>()
                        .insert(first, Connection::default()).ok();
            }
            if let Some(first) = self.0.runtime().create_engine::<Call>(app_world, "setup") {
                app_world.write_component::<Connection>()
                    .insert(first, Connection::default()).ok();
            }
            self.1 = false;
        }
    }
}

impl Plugin<ThunkContext> for Host {
    fn symbol() -> &'static str {
        "host"
    }

    fn description() -> &'static str {
        "Starts a static-file server host for a file directory."
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<(JoinHandle<ThunkContext>, Sender<()>)> {
        context.clone().task(|cancel_source| {
            let tc = context.clone();
            async {
                if let Some(work_dir) = tc.as_ref().find_text("work_dir") {
                    tc.update_status_only(format!("Serving work_dir {}", work_dir)).await;
                    let app = Route::new().nest(
                        "/",
                        StaticFilesEndpoint::new(
                            work_dir
                        ),
                    );

                    if let Some(address) = tc.as_ref().find_text("address") {
                        tc.update_status_only(format!("Starting {}", address)).await;
                        select! {
                            result = Server::new(
                                TcpListener::bind(address))
                                .run(app) => {
                                    match result {
                                        Ok(_) => {
                                            tc.update_status_only("Server is exiting").await; 
                                        },
                                        Err(err) => {
                                            tc.update_status_only(format!("Server error exit {}", err)).await;
                                        },
                                    }
                            }
                            _ = cancel_source => {
                                tc.update_status_only("Cancelling, server is exiting").await; 
                            }
                        }
                    }
                }

                Some(tc)
            }
        })
    }
}
