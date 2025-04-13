use serde::Deserialize;
use std::{thread, time::Duration};
use waybar_cffi::{
    gtk::{ prelude::ContainerExt, traits::{LabelExt, WidgetExt}, Label},
    waybar_module, InitInfo, Module,
};

const TEMP_COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const TEMP_CHARS: &[&str]= &["b","c","d","e","f","g","h","i","j"];

#[derive(Deserialize)]
struct Config {
    history: Option<i32>,
    interval: Option<i32>,
    temperature_item: Option<String>,
}

// CMNT stands for Cpu. Memory, Network, Temperature
struct TEMPGraph{
    history: Option<i32>,
    interval: Option<i32>,
    temperature_item: Option<String>,
    stats:Vec<f32>,
}

impl Module for TEMPGraph {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();

        let history = config.history.unwrap_or(10);
        let interval = config.interval.unwrap_or(5);
        let temperature_item = config.temperature_item.unwrap_or(String::from(""));

        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        let label_temp = Label::new(Some(""));

        let mut temp_graph = TEMPGraph{
            history: Some(history.clone()),
            interval: Some(interval.clone()),
            temperature_item: Some(temperature_item.clone()),
            stats: Vec::new(),
        };

        //for i in 1..history{
            // Refresh the system metrics
            if temperature_item.len() >0 {
                current_comp.refresh(true);
            }
            // Call each function to get all the values we need

            let mut temperature = 0.0;
            if temperature_item.len() >0 {
                temperature = get_temp_item(&current_comp,&temperature_item);
            }
            else {
                temperature = get_avg_temp(&current_comp);
            }


            if temp_graph.stats.len() == history as usize{
                temp_graph.stats.remove(0);
            }
            temp_graph.stats.push(temperature);

            thread::sleep(sleep_duration);
        //}


        let temp_chart = get_single_chart(&temp_graph.stats,TEMP_CHARS,TEMP_COLORS) ;

        label_temp.set_markup(&temp_chart.as_str());
        label_temp.set_tooltip_markup(Some(&temp_chart.as_str()));

        container.add(&label_temp);
        println!("temp_graph init finished");
        temp_graph

    }

    /// Called when the module should be updated.
    fn update(&mut self) {
        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let temperature_item = self.temperature_item.as_ref().cloned().unwrap_or(String::from(""));
        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        if temperature_item.len() >0 {
            current_comp.refresh(true);
        }

        if self.stats.len() == history as usize {
            let temperature;
            if temperature_item.len() >0 {
                temperature = get_temp_item(&current_comp,&temperature_item);
            }
            else {
                temperature = get_avg_temp(&current_comp);
            }
            self.stats.remove(0);
            self.stats.push(temperature);
        }
        thread::sleep(sleep_duration);

    }

    /// Called when the module should be refreshed in response to a signal.
    fn refresh(&mut self, signal: i32) {

        let history = self.history.unwrap_or(10);
        let interval = self.interval.unwrap_or(5);
        let temperature_item = self.temperature_item.as_ref().cloned().unwrap_or(String::from(""));
        let sleep_duration: Duration = Duration::from_secs(interval as u64);


        // Define a system that we will check
        let mut current_comp: sysinfo::Components=sysinfo::Components::new_with_refreshed_list();

        if temperature_item.len() >0 {
            current_comp.refresh(true);
        }

        if self.stats.len() == history as usize {
            let temperature;
            if temperature_item.len() >0 {
                temperature = get_temp_item(&current_comp,&temperature_item);
            }
            else {
                temperature = get_avg_temp(&current_comp);
            }
            self.stats.remove(0);
            self.stats.push(temperature);
        }
        thread::sleep(sleep_duration);
    }

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}

}

waybar_module!(TEMPGraph);

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