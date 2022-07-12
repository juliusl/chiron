use imgui::{MenuItem, Window};
use lifec::{
    editor::{Call, RuntimeEditor, WindowEvent}, Runtime, AttributeGraph, World, Entity, DispatcherBuilder, Extension, WorldExt,
};

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
    fn load_project(&mut self, file_path: impl AsRef<str>) -> Option<()> {
        if let Some(file) = AttributeGraph::load_from_file(file_path) {
            *self.0.project_mut().as_mut() = file;
            *self.0.project_mut() = self.0.project_mut().reload_source();
            Some(())
        } else {
            None
        }
    }

    /// Scans the project creating all engines found in the file
    fn create_engine_parts(&self, app_world: &World) -> Vec<Entity> {
        let mut engines = vec![];
        for (block_name, block) in self.0.project().iter_block() {
            if let Some(_) = block.get_block("call") {
                engines.push(block_name);
            }
        }

        let engines = engines.iter().map(|e| e.to_string());
        self.0
            .runtime()
            .create_engine_group::<Call>(app_world, engines.collect())
    }

    /// Creates the engine from a dropped_dir path
    fn create_default(&self, app_world: &World) -> Option<Entity> {
        self.0
            .runtime()
            .create_engine_group::<Call>(
                app_world,
                vec!["default"].iter().map(|s| s.to_string()).collect(),
            )
            .get(0)
            .and_then(|e| Some(*e))
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
                            self.create_engine_parts(app_world);
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

        self.0.on_ui(app_world, ui);
    }

    fn on_window_event(&'_ mut self, app_world: &World, event: &'_ lifec::editor::WindowEvent<'_>) {
        self.0.on_window_event(app_world, event);
        match event {
            WindowEvent::DroppedFile(dropped_file_path) => {
                if dropped_file_path.is_dir() {
                    let path = dropped_file_path.join(".runmd");
                    if path.exists() {
                        if let Some(_) = self.load_project(path.to_str().unwrap_or_default()) {
                            self.create_default(app_world);
                        }
                    }
                } else if "runmd" == dropped_file_path.extension().unwrap_or_default() {
                    if let Some(_) =
                        self.load_project(dropped_file_path.to_str().unwrap_or_default())
                    {
                        self.create_engine_parts(app_world);
                    }
                }
            }
            _ => {}
        }
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
