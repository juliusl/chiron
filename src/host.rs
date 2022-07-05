use imgui::{Window, MenuItem};
use lifec::{*, editor::{RuntimeEditor, Call, WindowEvent}, plugins::Connection};

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

    fn on_run(&'_ mut self, app_world: &World) {
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
