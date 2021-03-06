use imgui::{MenuItem, Window};
use lifec::{
    editor::RuntimeEditor,
    plugins::ThunkContext,
    AttributeGraph, DispatcherBuilder, Extension, Resources, Runtime,
    RuntimeDispatcher, World, WorldExt,
};

use crate::design::Design;

/// This type wraps the runtime editor as the underlying extension
/// Can be executed standalone w/o the main window
pub struct Host(
    pub RuntimeEditor,
    /// Clear entities
    Option<()>,
);

impl From<RuntimeEditor> for Host {
    fn from(editor: RuntimeEditor) -> Self {
        Host(editor, None)
    }
}

impl From<Runtime> for Host {
    fn from(runtime: Runtime) -> Self {
        Host::from(RuntimeEditor::new(runtime))
    }
}

impl AsRef<Runtime> for Host {
    fn as_ref(&self) -> &Runtime {
        &self.0.runtime()
    }
}

impl Host {
    fn load_project_from_content(&mut self, content: impl AsRef<str>) -> bool {
        let mut graph = AttributeGraph::from(0);
        if graph.batch_mut(content.as_ref()).is_ok() {
            *self.0.project_mut().as_mut() = graph;
            *self.0.project_mut() = self.0.project_mut().reload_source();
            true
        } else {
            false
        }
    }

    /// Signals the host to clear all entities stored in the app_world
    fn clear_entities(&mut self) {
        self.1 = Some(());
    }
}

impl Extension for Host {
    fn configure_app_world(world: &mut World) {
        RuntimeEditor::configure_app_world(world);
    }

    fn configure_app_systems(dispatcher: &mut DispatcherBuilder) {
        RuntimeEditor::configure_app_systems(dispatcher);
    }

    fn on_ui(&'_ mut self, app_world: &World, ui: &'_ imgui::Ui<'_>) {
        Window::new("Chiron Tools")
            .menu_bar(true)
            .size([800.0, 600.0], imgui::Condition::Appearing)
            .build(ui, || {
                ui.menu_bar(|| {
                    ui.menu("Actions", || {
                        if MenuItem::new("Scan for engine parts").build(ui) {
                            self.0.create_engine_parts(app_world);
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Scans the current project for all engines, adding each to the current runtime.");
                        }

                        if MenuItem::new("Reset").build(ui) {
                            self.clear_entities();
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("Deletes all entities, and components in storage.");
                        }
                    });
                });
            });

        Window::new("Start here")
            .size([300.0, 300.0], imgui::Condition::Appearing)
            .build(ui, || {
                if ui.button("Start help portal") {
                    let tokio_runtime = app_world.read_resource::<tokio::runtime::Runtime>();

                    tokio_runtime.block_on(async {
                        if let Some(portal) = Resources("design")
                            .read_binary::<Design>(
                                &ThunkContext::default(),
                                &"design/portal/.runmd".to_string(),
                            )
                            .await
                        {
                            if let Some(data) = String::from_utf8(portal.to_vec()).ok() {
                                if self.load_project_from_content(data) {
                                    if let Some(created) = self.0.create_default(app_world) {
                                        Runtime::start_event(created, app_world);
                                    }
                                }
                            }
                        }
                    });
                }

                ui.text_wrapped(
                    "This will launch a portal that hosts interactive documentation for this tool",
                );
            });

        self.0.on_ui(app_world, ui);
    }

    fn on_window_event(&'_ mut self, app_world: &World, event: &'_ lifec::editor::WindowEvent<'_>) {
        self.0.on_window_event(app_world, event);
    }

    fn on_run(&'_ mut self, app_world: &World) {
        self.0.on_run(app_world);
    }

    fn on_maintain(&'_ mut self, _app_world: &mut World) {
        // Clear entities
        if let Some(()) = self.1.take() {
            _app_world.delete_all();
        }
    }
}
