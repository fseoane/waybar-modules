use serde::Deserialize;
use waybar_cffi::{
    gtk::{ffi::GtkTooltip, prelude::ContainerExt, Label},
    waybar_module, InitInfo, Module,
};

const CPU_COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const CPU_CHARS:&str="bcdefghij";

#[derive(Deserialize)]
struct Config {
    history: Option<i32>,
    interval: Option<i32>,
    interface: Option<String>,
    temperature_item: Option<String>,

}

// CMNT stands for Cpu. Memory, Network, Temperature
struct CMNTGraph{
    cpu:Vec<f32>,
    mem:Vec<f32>,
    net_up:Vec<u64>,
    net_down:Vec<u64>,
    temp:Vec<u64>,
}

impl Module for CMNTGraph {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();

        let interface = config.interface.as_dref().unwrap_or("ëth0");
        let history = config.history.as_dref().unwrap_or(10);
        let interval = config.interval.as_dref().unwrap_or(5);
        let temperature_item = config.temperature_item.as_dref().unwrap_or("");

        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();
        let mut current_net = sysinfo::Networks::new_with_refreshed_list();
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        let mut cmnt_graph = CMNTGraph{
            cpu: Vec::new(),
            mem: Vec::new(),
            net_up: Vec::new(),
            net_down: Vec::new(),
            temp: Vec::new(),
        };

        for i in 1..history{
            // Call each function to get all the values we need
            let cpu_avg = get_cpu_use(&current_sys);
            let mem_prcnt = get_mem_use(&current_sys);
            let mut temperature = 0;
            if temperature_item.len() >0 {
                temperature = get_temp_item(&current_comp,&temperature_item);
            }
            else {
                temperature = get_avg_temp(&current_comp);
            }

            let ntwk_dwn ;
            let ntwk_up ;
            if interface == "total" {
                ntwk_dwn = get_tot_ntwk_dwn(&current_net,&interval);
                ntwk_up = get_tot_ntwk_up(&current_net,&interval);
            }
            else{
                ntwk_dwn = get_iface_ntwk_dwn(&current_net,&interval,&interface);
                ntwk_up = get_iface_ntwk_up(&current_net,&interval,&interface);
            }

            cmnt_graph.cpu.push(format! ("{:.1}",cpu_avg));
            cmnt_graph.mem.push(format! ("{:.1}",mem_prcnt));
            cmnt_graph.net_up.push(format! ("{:.1}",ntwk_up));
            cmnt_graph.net_down.push(format! ("{:.1}",ntwk_down));
            cmnt_graph.temp.push(format! ("{:.1}",format! ("{:.1}",temperature)));

        }

        let label = Label::new(Some(get_cpu_chart(&cmnt_graph.cpu)));

        let tooltip = GtkTooltip::new(Some(&format!(
            "Hello {}!",
            config.name.as_deref().unwrap_or("World")
        )));

        container.add(&label);
        container.add(&tooltip);

        cmnt_graph

    }
    /// Called when the module should be updated.
    fn update(&mut self) {
        // current_sys.refresh_all();
        // current_net.refresh();
        // if cmdn_config.temperature_item.len() >0 {
        //     current_comp.refresh();
        // }
    }

    /// Called when the module should be refreshed in response to a signal.
    fn refresh(&mut self, signal: i32) {}

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}
}

waybar_module!(CMNTGraph);


// -------------------------------------------------------------------------
// Get the CPU chart
fn get_cpu_chart(cpu_stats: &Vec<f32>) -> &str {

    let mut cpu_chart: String = String::from("");
    let cpu_avg_percent:f32 =  (cpu_stats.iter().sum() / cpu_stats.len());

    // Put all of the core loads into a vector
    for cpu_stat in cpu_stats.iter(){
        let cpu_stat_0_to_9 = ((*cpu_stat * (cpu_stats.len() as f32 - 1.0)) / 100.0) as i8;
        cpu_char.push(format!("<span color='{}'>{}</span>"),CPU_COLOR[cpu_stat_0_to_9],CPU_CHARS[cpu_stat_0_to_9]);
    }

    cpu_chart.as_str()
}

// -------------------------------------------------------------------------
// Get the average core usage
fn get_cpu_use(req_sys: &sysinfo::System) -> f32{
    // Put all of the core loads into a vector
    let mut cpus: Vec<f32> = Vec::new();
    for core in req_sys.cpus() { cpus.push(core.cpu_usage()); }

    // Get the average load
    let cpu_tot: f32 = cpus.iter().sum();
    let cpu_avg: f32 = cpu_tot / cpus.len() as f32;

    cpu_avg
}
// -------------------------------------------------------------------------

// Divide the used RAM by the total RAM
fn get_mem_use(req_sys: &sysinfo::System) -> f32{
    (req_sys.used_memory() as f32) / (req_sys.total_memory() as f32) * 100.
}
// -------------------------------------------------------------------------

// Get the network (down)  usage for an interface
fn get_iface_ntwk_dwn(  req_net: &sysinfo::Networks,
                        polling_secs: &i32,
                        iface: &str) -> u64{

    // Get the total bytes recieved by every network interface
    let mut rcv_tot: Vec<u64> = Vec::new();
    for (interface_name, ntwk) in req_net.list() {
        if interface_name == iface {
            //println!("{:?} rx:{} in {} secs --> {} KBps", interface_name,ntwk.received(),polling_secs,((ntwk.received() as i32 /polling_secs) / 1000) as i32 );
            rcv_tot.push(ntwk.received() as u64);
        }
    }

    // Total them and convert the bytes to KB
    let ntwk_tot: u64 = rcv_tot.iter().sum();
    //let ntwk_processed = (((ntwk_tot*8)/(*polling_secs as u64)) / 1024) as u64;
    let ntwk_processed = (((ntwk_tot)/(*polling_secs as u64)) / 1000) as u64;
    ntwk_processed
}
// -------------------------------------------------------------------------
// Get the network (up) usage for an interface
fn get_iface_ntwk_up(   req_net: &sysinfo::Networks,
                        polling_secs: &i32,
                        iface: &str) -> u64{

    // Get the total bytes sent by every network interface
    let mut snd_tot: Vec<u64> = Vec::new();
    for (interface_name, ntwk) in req_net.list() {
        if interface_name == iface {
            //println!("{:?} rx:{} in {} secs --> {} KBps", interface_name,ntwk.transmitted(),polling_secs,((ntwk.transmitted() as i32 /polling_secs) / 1000) as i32 );
            snd_tot.push(ntwk.transmitted() as u64);
        }
    }

    // Total them and convert the bytes to KB
    let ntwk_tot: u64 = snd_tot.iter().sum();
    //let ntwk_processed: u64 = (((ntwk_tot * 8) / (*polling_secs as u64)) / 1024) as u64;
    let ntwk_processed: u64 = (((ntwk_tot) / (*polling_secs as u64)) / 1000) as u64;
    ntwk_processed
}
// -------------------------------------------------------------------------
// Get average temperature of all temperature sensors
fn get_avg_temp(req_comp: &sysinfo::Components) -> i32{

    // For every component, if it's the CPU, put its temperature in variable to return
    let mut avg_temp: i32 = 0;
    let temp_components_count:i32 = req_comp.list().len() as i32;
    if temp_components_count >0 {
        for comp in req_comp.list() {
            if comp.temperature()>0.0
            {
                avg_temp += comp.temperature() as i32;
            }
        }
        (avg_temp/temp_components_count) as i32
    }
    else {
        0
    }
}
// -------------------------------------------------------------------------
// Get the temperature of an specific temp item
fn get_temp_item(req_comp: &sysinfo::Components, temp_item: &str) -> i32{
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut wanted_temp: f32 = -1.;
    for comp in req_comp.list() {
        //println!("{:?}", comp.label());
        if comp.label() == temp_item {
            wanted_temp = comp.temperature();
        }
    }

    wanted_temp as i32
}