use lifec::{plugins::{Plugin, ThunkContext, Project, WriteFile, OpenFile, OpenDir, Process, Remote, Timer, Config}, Runtime, editor::{Call, RuntimeEditor}};
use lifec_poem::StaticFiles;

use crate::{install::Install, cloud_init::{MakeMime, ReadMime}, elm::MakeElm, Main, host::Host};

/// Lab component hosts a runtime
#[derive(Default)]
pub struct Lab;

impl Plugin<ThunkContext> for Lab {
    fn symbol() -> &'static str {
        "lab"
    }

    fn description() -> &'static str {
        "Hosts a lab runtime"
    }

    fn call_with_context(context: &mut ThunkContext) -> Option<lifec::plugins::AsyncContext> {
        context.clone().task(|cancel_source| {
            let tc = context.clone();
            async move {
                if let Some(project_src) = tc.as_ref().find_text("project_src") {
                    if let Some(project) = Project::load_file(project_src) {
                        let mut runtime = Runtime::new(project);
                        runtime.install::<Call, WriteFile>();
                        runtime.install::<Call, OpenFile>();
                        runtime.install::<Call, Runtime>();
                        runtime.install::<Call, OpenDir>();
                        runtime.install::<Call, Process>();
                        runtime.install::<Call, Remote>();
                        runtime.install::<Call, Timer>();
                        // Installs a tool
                        runtime.install::<Call, Install>();
                        // Cloud-init tools
                        runtime.install::<Call, MakeMime>();
                        runtime.install::<Call, ReadMime>();
                        // Hosting code
                        runtime.install::<Call, StaticFiles>();
                        runtime.install::<Call, MakeElm>();
                        // 
                        runtime.add_config(Config("cloud_init", |tc|{ 
                            tc.as_mut()
                                .with_text("tool_name", "cloud_init")
                                .with_text("ext", "yml")
                                .with_text("work_dir", ".config/cloud_init")
                                .with_text("node_title", "Install cloud_init parts")
                                .add_text_attr("src_dir", "lib");
                        }));
                
                        runtime.add_config(Config("cloud_init_exit", |tc|{ 
                            tc.as_mut()
                                .with_text("tool_name", "cloud_init")
                                .with_text("ext", "yml")
                                .with_text("work_dir", ".config/cloud_init")
                                .with_text("node_title", "Install cloud_init exit")
                                .with_text("src_dir", "lib")
                                .add_text_attr("src_type", "exit");
                        }));
                
                        runtime.add_config(Config("cloud_init_enter", |tc|{ 
                            tc.as_mut()
                                .with_text("tool_name", "cloud_init")
                                .with_text("ext", "yml")
                                .with_text("work_dir", ".config/cloud_init")
                                .with_text("node_title", "Install cloud_init enter")
                                .with_text("src_dir", "lib")
                                .add_text_attr("src_type", "enter");
                        }));
                
                        runtime.add_config(Config("elm_portal", |tc| {
                            tc.as_mut()
                                .with_text("elm_src", "lib/elm/portal/src/Main.elm")
                                .add_text_attr("elm_dst", "lib/elm/portal/portal.js");
                        }));
                        let mut main = Host(RuntimeEditor::new(runtime), false);
                        
                        Runtime::start_with(
                            &mut main,
                            "lab",
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