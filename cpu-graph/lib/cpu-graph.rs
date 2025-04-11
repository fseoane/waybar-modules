use serde::Deserialize;
use std::{thread, time::Duration};
use waybar_cffi::{
    gtk::{ prelude::ContainerExt, traits::{LabelExt, WidgetExt}, Label},
    waybar_module, InitInfo, Module,
};

const CPU_COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const CPU_CHARS: &[&str]= &["b","c","d","e","f","g","h","i","j"];

#[derive(Deserialize)]
struct Config {
    history: Option<i32>,
    interval: Option<i32>,
}

// CMNT stands for Cpu. Memory, Network, Temperature
struct CPUGraph{
    history: Option<i32>,
    interval: Option<i32>,
    stats:Vec<f32>,
}

impl Module for CPUGraph {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();

        let history = config.history.unwrap_or(10);
        let interval = config.interval.unwrap_or(5);

        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();


        let label_cpu = Label::new(Some(""));


        let mut cpu_graph = CPUGraph{
            history: Some(history.clone()),
            interval: Some(interval.clone()),
            stats: Vec::new(),
        };

        //for i in 1..history{
            // Refresh the system metrics
            current_sys.refresh_all();

            // Call each function to get all the values we need
            let cpu_avg = get_cpu_use(&current_sys);

            if cpu_graph.stats.len() == history as usize{
                cpu_graph.stats.remove(0);
            }
            cpu_graph.stats.push(cpu_avg);

            thread::sleep(sleep_duration);

        //}

        let cpu_chart = get_single_chart(&cpu_graph.stats,CPU_CHARS,CPU_COLORS) ;

        label_cpu.set_markup(&cpu_chart.as_str());
        label_cpu.set_tooltip_markup(Some(&cpu_chart.as_str()));

        container.add(&label_cpu);

        cpu_graph

    }

    fn update(&mut self) {

        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let sleep_duration: Duration = Duration::from_secs(interval as u64);

        // Define a system that we will check
        let mut current_sys = sysinfo::System::new_all();


        current_sys.refresh_all();

        if self.stats.len() == history as usize {
            let new_stat = get_cpu_use(&current_sys);
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
            let new_stat = get_cpu_use(&current_sys);
            self.stats.remove(0);
            self.stats.push(new_stat);
        }

        thread::sleep(sleep_duration);
    }

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}

}

waybar_module!(CPUGraph);

// -------------------------------------------------------------------------

// Get the  chart
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
