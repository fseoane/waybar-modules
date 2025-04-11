use serde::Deserialize;
use std::{thread, time::Duration};
use waybar_cffi::{
    gtk::{glib::ObjectExt, prelude::ContainerExt, traits::{LabelExt, WidgetExt}, Label},
    waybar_module, InitInfo, Module,
};

const CPU_COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const CPU_CHARS: &[&str]= &["b","c","d","e","f","g","h","i","j"];

#[derive(Deserialize)]
struct Config {
    history: Option<i32>,
    interval: Option<i32>,
    interface: Option<String>,
    temperature_item: Option<String>,
}

// CMNT stands for Cpu. Memory, Network, Temperature
struct CMNTGraph{
    history: Option<i32>,
    interval: Option<i32>,
    interface: Option<String>,
    temperature_item: Option<String>,
    cpu:Vec<f32>,
    mem:Vec<f32>,
    net_up:Vec<u64>,
    net_down:Vec<u64>,
    temp:Vec<f32>,
}

impl Module for CMNTGraph {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();

        let interface = config.interface.unwrap_or(String::from("ëth0"));
        let history = config.history.unwrap_or(10);
        let interval = config.interval.unwrap_or(5);
        let temperature_item = config.temperature_item.unwrap_or(String::from(""));

        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();
        let mut current_net = sysinfo::Networks::new_with_refreshed_list();
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        let mut cmnt_graph = CMNTGraph{
            history: Some(history.clone()),
            interval: Some(interval.clone()),
            interface: Some(interface.clone()),
            temperature_item: Some(temperature_item.clone()),
            cpu: Vec::new(),
            mem: Vec::new(),
            net_up: Vec::new(),
            net_down: Vec::new(),
            temp: Vec::new(),
        };

        //for i in 1..history{
            // Refresh the system metrics
            current_sys.refresh_all();
            current_net.refresh(true);
            if temperature_item.len() >0 {
                current_comp.refresh(true);
            }
            // Call each function to get all the values we need
            let cpu_avg = get_cpu_use(&current_sys);
            let mem_prcnt = get_mem_use(&current_sys);
            let mut temperature = 0.0;
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

            if cmnt_graph.cpu.len() == history as usize{
                cmnt_graph.cpu.remove(0);
            }
            cmnt_graph.cpu.push(cpu_avg);

            if cmnt_graph.mem.len() == history as usize{
                cmnt_graph.mem.remove(0);
            }
            cmnt_graph.mem.push(mem_prcnt);

            if cmnt_graph.net_up.len() == history as usize{
                cmnt_graph.net_up.remove(0);
            }
            cmnt_graph.net_up.push(ntwk_up);

            if cmnt_graph.net_down.len() == history as usize{
                cmnt_graph.net_down.remove(0);
            }
            cmnt_graph.net_down.push(ntwk_dwn);

            if cmnt_graph.temp.len() == history as usize{
                cmnt_graph.temp.remove(0);
            }
            cmnt_graph.temp.push(temperature);

            thread::sleep(sleep_duration);
        //}

        let label = Label::new(Some(""));
        label.set_markup(get_cpu_chart(&cmnt_graph.cpu).as_str());

        label.set_tooltip_markup(Some(get_cpu_chart(&cmnt_graph.cpu).as_str()));

        container.add(&label);

        cmnt_graph
    }

    // fn init(info: &InitInfo, config: Config) -> Self {
    //     let container = info.get_root_widget();

    //     let interface = config.interface.unwrap_or(String::from("ëth0"));
    //     let history = config.history.unwrap_or(10);
    //     let interval = config.interval.unwrap_or(5);
    //     let temperature_item = config.temperature_item.unwrap_or(String::from(""));

    //     let sleep_duration: Duration = Duration::from_secs(interval as u64);


    //     // Define a system that we will check
    //     let mut current_sys = sysinfo::System::new_all();
    //     let mut current_net = sysinfo::Networks::new_with_refreshed_list();
    //     let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

    //     let mut cmnt_graph = CMNTGraph{
    //         history: Some(history.clone()),
    //         interval: Some(interval.clone()),
    //         interface: Some(interface.clone()),
    //         temperature_item: Some(temperature_item.clone()),
    //         cpu: Vec::new(),
    //         mem: Vec::new(),
    //         net_up: Vec::new(),
    //         net_down: Vec::new(),
    //         temp: Vec::new(),
    //     };

    //     loop {
    //         // Refresh the system metrics
    //         current_sys.refresh_all();
    //         current_net.refresh(true);
    //         if temperature_item.len() >0 {
    //             current_comp.refresh(true);
    //         }
    //         // Call each function to get all the values we need
    //         let cpu_avg = get_cpu_use(&current_sys);
    //         let mem_prcnt = get_mem_use(&current_sys);
    //         let mut temperature = 0.0;
    //         if temperature_item.len() >0 {
    //             temperature = get_temp_item(&current_comp,&temperature_item);
    //         }
    //         else {
    //             temperature = get_avg_temp(&current_comp);
    //         }

    //         let ntwk_dwn ;
    //         let ntwk_up ;
    //         if interface == "total" {
    //             ntwk_dwn = get_tot_ntwk_dwn(&current_net,&interval);
    //             ntwk_up = get_tot_ntwk_up(&current_net,&interval);
    //         }
    //         else{
    //             ntwk_dwn = get_iface_ntwk_dwn(&current_net,&interval,&interface);
    //             ntwk_up = get_iface_ntwk_up(&current_net,&interval,&interface);
    //         }

    //         if cmnt_graph.cpu.len() == history as usize{
    //             cmnt_graph.cpu.remove(0);
    //         }
    //         cmnt_graph.cpu.push(cpu_avg);

    //         if cmnt_graph.mem.len() == history as usize{
    //             cmnt_graph.mem.remove(0);
    //         }
    //         cmnt_graph.mem.push(mem_prcnt);

    //         if cmnt_graph.net_up.len() == history as usize{
    //             cmnt_graph.net_up.remove(0);
    //         }
    //         cmnt_graph.net_up.push(ntwk_up);

    //         if cmnt_graph.net_down.len() == history as usize{
    //             cmnt_graph.net_down.remove(0);
    //         }
    //         cmnt_graph.net_down.push(ntwk_dwn);

    //         if cmnt_graph.temp.len() == history as usize{
    //             cmnt_graph.temp.remove(0);
    //         }
    //         cmnt_graph.temp.push(temperature);

    //         thread::sleep(sleep_duration);
    //     }

    //     let label = Label::new(Some(""));
    //     label.set_markup(get_cpu_chart(&cmnt_graph.cpu).as_str());

    //     label.set_tooltip_markup(Some(get_cpu_chart(&cmnt_graph.cpu).as_str()));

    //     container.add(&label);
    //     cmnt_graph

    // }
    /// Called when the module should be updated.
    fn update(&mut self) {
        let interface = self.interface.as_ref().cloned().unwrap_or(String::from("ëth0"));
        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let temperature_item = self.temperature_item.as_ref().cloned().unwrap_or(String::from(""));

        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();
        let mut current_net = sysinfo::Networks::new_with_refreshed_list();
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        current_sys.refresh_all();
        current_net.refresh(true);
        if temperature_item.len() >0 {
            current_comp.refresh(true);
        }

        if self.cpu.len() == history as usize {
            let cpu_avg = get_cpu_use(&current_sys);
            self.cpu.remove(0);
            self.cpu.push(cpu_avg);
        }

        if self.mem.len() == history as usize {
            let mem_prcnt = get_mem_use(&current_sys);
            self.mem.remove(0);
            self.mem.push(mem_prcnt);
        }

        if self.temp.len() == history as usize {
            let temperature;
            if temperature_item.len() >0 {
                temperature = get_temp_item(&current_comp,&temperature_item);
            }
            else {
                temperature = get_avg_temp(&current_comp);
            }
            self.temp.remove(0);
            self.temp.push(temperature);
        }

        if self.net_up.len() == history as usize {
            let ntwk_up ;
            if interface == "total" {
                ntwk_up = get_tot_ntwk_up(&current_net,&interval);
            }
            else{
                ntwk_up = get_iface_ntwk_up(&current_net,&interval,&interface);
            }
            self.net_up.remove(0);
            self.net_up.push(ntwk_up);
        }

        if self.net_down.len() == history as usize {
            let ntwk_dwn ;
            if interface == "total" {
                ntwk_dwn = get_tot_ntwk_dwn(&current_net,&interval);
            }
            else{
                ntwk_dwn = get_iface_ntwk_dwn(&current_net,&interval,&interface);
            }
            self.net_down.remove(0);
            self.net_down.push(ntwk_dwn);
        }



    }

    /// Called when the module should be refreshed in response to a signal.
    fn refresh(&mut self, signal: i32) {

        let interface = self.interface.as_ref().cloned().unwrap_or(String::from("ëth0"));
        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let temperature_item = self.temperature_item.as_ref().cloned().unwrap_or(String::from(""));

        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();
        let mut current_net = sysinfo::Networks::new_with_refreshed_list();
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        current_sys.refresh_all();
        current_net.refresh(true);
        if temperature_item.len() >0 {
            current_comp.refresh(true);
        }

        if self.cpu.len() == history as usize {
            let cpu_avg = get_cpu_use(&current_sys);
            self.cpu.remove(0);
            self.cpu.push(cpu_avg);
        }

        if self.mem.len() == history as usize {
            let mem_prcnt = get_mem_use(&current_sys);
            self.mem.remove(0);
            self.mem.push(mem_prcnt);
        }

        if self.temp.len() == history as usize {
            let temperature;
            if temperature_item.len() >0 {
                temperature = get_temp_item(&current_comp,&temperature_item);
            }
            else {
                temperature = get_avg_temp(&current_comp);
            }
            self.temp.remove(0);
            self.temp.push(temperature);
        }

        if self.net_up.len() == history as usize {
            let ntwk_up ;
            if interface == "total" {
                ntwk_up = get_tot_ntwk_up(&current_net,&interval);
            }
            else{
                ntwk_up = get_iface_ntwk_up(&current_net,&interval,&interface);
            }
            self.net_up.remove(0);
            self.net_up.push(ntwk_up);
        }

        if self.net_down.len() == history as usize {
            let ntwk_dwn ;
            if interface == "total" {
                ntwk_dwn = get_tot_ntwk_dwn(&current_net,&interval);
            }
            else{
                ntwk_dwn = get_iface_ntwk_dwn(&current_net,&interval,&interface);
            }
            self.net_down.remove(0);
            self.net_down.push(ntwk_dwn);
        }
    }

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}

}

waybar_module!(CMNTGraph);

// -------------------------------------------------------------------------

// Get the CPU chart
// Get the CPU chart
fn get_cpu_chart(cpu_stats: &Vec<f32>) -> String {

    let mut cpu_chart: String = String::from("<span font-family='efe-graph' rise='-4444'>");
    let _cpu_avg_percent: f32 = cpu_stats.iter().copied().sum::<f32>() / cpu_stats.len() as f32;

    // Put all of the core loads into a vector
    for cpu_stat in cpu_stats.iter(){
        let cpu_stat_0_to_9: usize = ((*cpu_stat * (cpu_stats.len() as f32 - 1.0)) / 100.0) as usize;
        cpu_chart.push_str(format!("<span color='{}'>{}</span>",CPU_COLORS[cpu_stat_0_to_9],CPU_CHARS[cpu_stat_0_to_9]).as_str());
    }
    //{\"text\":\"$TEXT\",\"alt\":\"Avg.Usage: $averageUsage\",\"tooltip\":\"Avg.Usage:$averageUsage\",\"class\":\"\",\"percentage\":$cpuUsage}

    cpu_chart.push_str("</span>");
    cpu_chart
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

// Get the total network (down) usage
fn get_tot_ntwk_dwn(req_net: &sysinfo::Networks,
    polling_secs: &i32) -> u64{
    // Get the total bytes recieved by every network interface
    let mut rcv_tot: Vec<u64> = Vec::new();
    for (_interface_name, ntwk) in req_net.list() {
        rcv_tot.push(ntwk.received() as u64);
    }

    // Total them and convert the bytes to KB
    let ntwk_tot: u64 = rcv_tot.iter().sum();
    //let ntwk_processed = (((ntwk_tot*8)/(*polling_secs as u64)) / 1024) as u64;
    let ntwk_processed = (((ntwk_tot)/(*polling_secs as u64)) / 1000) as u64;
    ntwk_processed
}

// -------------------------------------------------------------------------

// Get the total network (up) usage
fn get_tot_ntwk_up( req_net: &sysinfo::Networks,
    polling_secs: &i32) -> u64{
    // Get the total bytes sent by every network interface
    let mut snd_tot: Vec<u64> = Vec::new();
    for (_interface_name, ntwk) in req_net.list() {
        snd_tot.push(ntwk.transmitted() as u64);
    }

    // Total them and convert the bytes to KB
    let ntwk_tot: u64 = snd_tot.iter().sum();
    //let ntwk_processed = (((ntwk_tot*8)/(*polling_secs as u64)) / 1024) as u64;
    let ntwk_processed = (((ntwk_tot)/(*polling_secs as u64)) / 1000) as u64;
    ntwk_processed
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
fn get_avg_temp(req_comp: &sysinfo::Components) -> f32{

    // For every component, if it's the CPU, put its temperature in variable to return
    let mut avg_temp: f32 = 0.0;
    let temp_components_count:i32 = req_comp.list().len() as i32;
    let mut count_real_components = 0;
    if temp_components_count > 0 {
        for comp in req_comp.list() {
            match comp.temperature() {
                Some(t) => {
                    count_real_components += 1;
                    avg_temp += t
                }
                None => (),
            }
        }
        avg_temp/count_real_components as f32
    }
    else {
        0.0
    }
}


// -------------------------------------------------------------------------

// Get the temperature of an specific temp item
fn get_temp_item(req_comp: &sysinfo::Components, temp_item: &str) -> f32{
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut wanted_temp: f32 = -1.;
    for comp in req_comp.list() {
        //println!("{:?}", comp.label());
        if comp.label() == temp_item {
           match comp.temperature() {
                Some(t) =>  wanted_temp = t,
                None => (),
            }
        }
    }

    wanted_temp
}