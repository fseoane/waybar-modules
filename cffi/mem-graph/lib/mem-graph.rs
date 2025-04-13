use serde::Deserialize;
use std::{thread, time::Duration};
use waybar_cffi::{
    gtk::{ prelude::ContainerExt, traits::{LabelExt, WidgetExt}, Label},
    waybar_module, InitInfo, Module,
};

const MEM_COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const MEM_CHARS: &[&str]= &["b","c","d","e","f","g","h","i","j"];

#[derive(Deserialize)]
struct Config {
    history: Option<i32>,
    interval: Option<i32>,
}

// CMNT stands for Cpu. Memory, Network, Temperature
struct MEMGraph{
    history: Option<i32>,
    interval: Option<i32>,
    stats:Vec<f32>,
}

impl Module for MEMGraph {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();

        let history = config.history.unwrap_or(10);
        let interval = config.interval.unwrap_or(5);

        let sleep_duration: Duration = Duration::from_secs(interval as u64);

        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();

        let label_mem = Label::new(Some(""));

        let mut mem_graph = MEMGraph{
            history: Some(history.clone()),
            interval: Some(interval.clone()),
            stats: Vec::new(),
        };

        //for i in 1..history{
            // Refresh the system metrics
            current_sys.refresh_all();

            let mem_prcnt = get_mem_use(&current_sys);

            if mem_graph.stats.len() == history as usize{
                mem_graph.stats.remove(0);
            }
            mem_graph.stats.push(mem_prcnt);

            thread::sleep(sleep_duration);
        //}

        let mem_chart = get_single_chart(&mem_graph.stats,MEM_CHARS,MEM_COLORS) ;

        label_mem.set_markup(&mem_chart.as_str());
        label_mem.set_tooltip_markup(Some(&mem_chart.as_str()));

        container.add(&label_mem);
        println!("mem_graph init finished");
        mem_graph

    }

    fn update(&mut self) {
        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();

        current_sys.refresh_all();

        if self.stats.len() == history as usize {
            let new_stat = get_mem_use(&current_sys);
            self.stats.remove(0);
            self.stats.push(new_stat);
        }
        thread::sleep(sleep_duration);

    }

    /// Called when the module should be refreshed in response to a signal.
    fn refresh(&mut self, signal: i32) {

        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();

        current_sys.refresh_all();

        if self.stats.len() == history as usize {
            let new_stat = get_mem_use(&current_sys);
            self.stats.remove(0);
            self.stats.push(new_stat);
        }
        thread::sleep(sleep_duration);
    }

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}

}

waybar_module!(MEMGraph);

// -------------------------------------------------------------------------

// Get the CPU chart
fn get_single_chart(stats_set: &Vec<f32>, symbols:&[&str],colors:&[&str] ) -> String {

    let mut return_chart: String = String::from("<span font-family='efe-graph' rise='-4444'>");
    let _chart_avg_percent: f32 = stats_set.iter().copied().sum::<f32>() / stats_set.len() as f32;

    // Put all of the core loads into a vector
    for one_stat in stats_set.iter(){
        let stat_0_to_9: usize = ((*one_stat * (stats_set.len() as f32 - 1.0)) / 100.0) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&colors[stat_0_to_9],&symbols[stat_0_to_9]).as_str());
    }
    //{\"text\":\"$TEXT\",\"alt\":\"Avg.Usage: $averageUsage\",\"tooltip\":\"Avg.Usage:$averageUsage\",\"class\":\"\",\"percentage\":$cpuUsage}

    return_chart.push_str("</span>");
    return_chart
}

// -------------------------------------------------------------------------

// Divide the used RAM by the total RAM
fn get_mem_use(req_sys: &sysinfo::System) -> f32{
    (req_sys.used_memory() as f32) / (req_sys.total_memory() as f32) * 100.
}
