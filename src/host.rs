use imgui::{MenuItem, Window};
use lifec::{
    editor::{Call, RuntimeEditor, WindowEvent},
    *,
};

pub struct Host(pub RuntimeEditor, pub bool);

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

impl Host {
    /// Creates a new host engine group
    fn create_host(
        &self, 
        app_world: &World
    ) -> Vec<Entity> {
        self.0.runtime().create_engine_group::<Call>(
            app_world,
            vec!["host", "setup", "setup_enter", "setup_exit"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }

    /// Creates the engine from a dropped_dir path
    fn create_default(
        &self,
        app_world: &World,
    ) -> Option<Entity> {
        self.0.runtime().create_engine_group::<Call>(
            app_world,
            vec!["default"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ).get(0).and_then(|e| Some(*e))
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
                        if MenuItem::new("Create host").build(ui) {
                            self.create_host(app_world);
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
                        if let Some(file) = AttributeGraph::load_from_file(
                            format!("{:?}", path).trim_matches('"'),
                        ) {
                            *self.0.project_mut().as_mut() = file;
                            *self.0.project_mut() = self.0.project_mut().reload_source();
                             self.create_default(app_world);
                        }
                    }
                } else if "runmd" == dropped_file_path.extension().unwrap_or_default() {
                    if let Some(file) =
                        AttributeGraph::load_from_file(dropped_file_path.to_str().unwrap_or_default())
                    {
                        *self.0.project_mut().as_mut() = file;
                        *self.0.project_mut() = self.0.project_mut().reload_source();
                         self.create_host(app_world);
                    }
                }
            }
            _ => {}
        }
    }

    fn on_run(&'_ mut self, app_world: &World) {
        self.0.on_run(app_world);
    }

    fn on_maintain(&'_ mut self, app_world: &mut World) {
        if self.1 {
            app_world.delete_all();
            self.1 = false;
        }
    }
}
