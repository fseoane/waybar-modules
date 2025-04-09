use serde::Deserialize;
use waybar_cffi::{
    gtk::{prelude::ContainerExt, Label},
    waybar_module, InitInfo, Module,
};

struct CpuGraph;

impl Module for CpuGraph {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();
       
        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();
        let mut current_net = sysinfo::Networks::new_with_refreshed_list();
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();
       
        let label = Label::new(Some(&format!(
            "Hello {}!",
            config.name.as_deref().unwrap_or("World")
        )));
       
       
       
       
        container.add(&label);

        CpuGraph
    }
    /// Called when the module should be updated.
    fn update(&mut self) {}

    /// Called when the module should be refreshed in response to a signal.
    fn refresh(&mut self, signal: i32) {}

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}
}

waybar_module!(CpuGraph);

#[derive(Deserialize)]
struct Config {
    name: Option<String>,
}
