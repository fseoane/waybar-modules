
use std::env;
use std::{thread, time::Duration};

const COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const CHARS: &[&str]=  &["a","b","c","d","e","f","g","h","i","j"];

fn display_help() {
    println!("Usage: {} [options]", env::current_exe().unwrap().display());
    println!();
    println!("Options:");
    println!("  --interval <seconds>   Set the interval between updates (default: 2)");
    println!("  --history <number>     Set the number of reading to show in the graph (default: 10)");
    println!("  --item <sensor_name>   Set the name of temperature sensor/item to show in the graph (default: max)");
    println!("
           --item max      - means the return will be the average of temperature from all sensors");
    println!("                            --item avg      - means the return will be the max of temperature from all sensors");
    println!("                            --item '<name>' - means the return will be the temperature from that sensor");
    println!();
}

// -------------------------------------------------------------------------


// Get the  chart
fn get_single_chart(stats_set: &Vec<f32>, symbols:&[&str],colors:&[&str] ) -> String {

    //let mut return_chart: String = String::from("<span font-family='efe-graph' rise='-4444'>");
    let mut return_chart: String = String::from("");
    let _chart_avg_percent: f32 = stats_set.iter().copied().sum::<f32>() / stats_set.len() as f32;

    // Put all of the core loads into a vector
    for one_stat in stats_set.iter(){
        let stat_0_to_9: usize = ((one_stat * (symbols.len() as f32 - 1.0)) / 100.0) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&colors[stat_0_to_9],&symbols[stat_0_to_9]).as_str());
    }
    //return_chart.push_str("</span>");
    return_chart
}

// -------------------------------------------------------------------------

// Get the temperature of the CPU
fn get_avg_temp(req_comp: &sysinfo::Components) -> (f32,Vec<String>){
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut avg_temp: f32 = 0.0;
    let mut temp: f32 = 0.0;
    let mut components_temp:Vec<String> = Vec::new();
    let mut num_non_zero_components: i32 = 0;

    let temp_components_count:i32 = req_comp.list().len() as i32;
    if temp_components_count >0 {
        for comp in req_comp.list() {
            temp += match comp.temperature() {
                Some(t) => {
                    if t > 0.0 {
                        t
                    } else {
                        0.0
                    }
                },
                None => 0.0,
            };
            if temp > 0.0 {
                avg_temp += temp;
                num_non_zero_components +=1;
                // if comp.temperature()>0.0
                // {
                //     avg_temp += comp.temperature();
                // }
                components_temp.push(format!("{}-{}°C",comp.label(),temp as i32 ));
            }
        }
        avg_temp = avg_temp/num_non_zero_components as f32;
    }
    else {
        avg_temp = 0.0;
    }

    return (avg_temp,components_temp);

}


// Get the temperature of the CPU
fn get_max_temp(req_comp: &sysinfo::Components) -> (f32,Vec<String>){
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut max_temp: f32 = 0.0;
    let mut temp: f32;
    let mut components_temp:Vec<String> = Vec::new();

    let temp_components_count:i32 = req_comp.list().len() as i32;
    if temp_components_count > 0 {
        for comp in req_comp.list() {
            temp = match comp.temperature() {
                Some(t) => {
                    if t > 0.0 {
                        t
                    } else {
                        0.0
                    }
                },
                None => 0.0,
            };

            if temp > max_temp {max_temp = temp;}
            if temp > 0.0 { components_temp.push(format!("{}-{}°C",comp.label(),temp as i32 ));}
        }
    }
    else {
        max_temp=0.0;
    }

    return (max_temp,components_temp);

}

// Get the temperature of the CPU
fn get_temp_item(req_comp: &sysinfo::Components, temp_item: &str) ->(f32,Vec<String>){
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut wanted_temp: f32 = -1.;
    for comp in req_comp.list() {
        //println!("{:?}", comp.label());
        if comp.label() == temp_item {
            wanted_temp = match comp.temperature() {
                Some(t) => {
                    if t > 0.0 {
                        t
                    } else {
                        0.0
                    }
                },
                None => 0.0,
            };
        }
    }
    return (wanted_temp,vec![format!("{}-{}",temp_item.to_string(),wanted_temp)]);

}

// -------------------------------------------------------------------------

fn main() {
    let mut history = 10;
    let mut interval: u32 = 2;
    let mut item: String = String::from("max");

    let args: Vec<String> = env::args().collect();


    // gather parameters from command line
    if args.len() > 1 {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--help" {
                display_help();
            } else if arg == "--interval" && i + 1 < args.len() {
                interval = args[i + 1].parse().unwrap_or_else(|_| {
                    panic!("--interval must be greater than 0!")
                });
            } else if arg == "--history" && i + 1 < args.len() {
                history = args[i + 1].parse().unwrap_or_else(|_| {
                    panic!("--history must be greater than 0!")
                });
            } else if arg == "--item" && i + 1 < args.len() {
                item = args[i + 1].to_string()
            };

        }
    }
    if (interval == 0) || (history == 0)  {
        panic!("--interval and --history must be greater than 0");
    }

    let mut stats: Vec<f32> = vec![0.0; history];

    let sleep_duration: Duration = Duration::from_secs(interval as u64);

    //let mut current_comp: sysinfo::Components=sysinfo::Components::new();
    let mut current_comp = sysinfo::Components::new_with_refreshed_list();
    let mut temp_stat_type  = String::from("Avg.");

    loop {
        let temperature: (f32, Vec<String>);

        current_comp.refresh(true);

        if item == "avg" {
            temperature = get_avg_temp(&current_comp);
        } else if item == "max"{
                temperature = get_max_temp(&current_comp);
                temp_stat_type  = String::from("Max.");
            } else {
                temperature = get_temp_item(&current_comp,&item);
                temp_stat_type  = item.to_string();
            }


        if stats.len() == history as usize{
            stats.remove(0);
        }
        stats.push(temperature.0);

        let components_temp = temperature.1;
        let components_temp_item_maxlen: usize = match components_temp
            .iter()
            .map(|f|f.len()).max(){
                Some(m) => m,
                None => 0,
            };
        let mut components_temp_tabulada = String::from("");
        for line in components_temp.iter(){
            let linelen = line.len();
            let tabbedline = line.replace("-"," ".repeat(components_temp_item_maxlen-(linelen-1)).as_str());
            components_temp_tabulada.push_str(format!("{}\\n",&tabbedline).as_str());
        }

        let temp_chart = get_single_chart(&stats,CHARS,COLORS);
        println!("{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\":\"\",\"alt\":\"{}Temp: {}°C\\n---------------\\n{}\",\"percentage\":{}}}",&temp_chart,&temp_chart,&temp_stat_type,stats[stats.len()-1] as i32,&components_temp_tabulada,stats[stats.len()-1] as i32);
        thread::sleep(sleep_duration);

    }
}
