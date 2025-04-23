
use std::env;
use std::{thread, time::Duration};
use sysinfo::Disks;

const COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const CHARS: &[&str]=  &["0","b","c","d","e","f","g","h","i","j"];
//const CHARS: &[&str]=  &[" ","▁","▂","▃","▄","▅","▆","▇","█","█"];   // with mono nerd fonts

fn display_help() {
    println!("Usage: {} [options]", env::current_exe().unwrap().display());
    println!();
    println!("Options:");
    println!("  --interval <seconds>   Set the interval between updates (default: 1)");
    println!("  --history <number>     Set the number of reading to show in the graph (default: 15)");
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
        //let stat_0_to_9: usize = (one_stat  / symbols.len() as f32).round() as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&colors[stat_0_to_9],&symbols[stat_0_to_9]).as_str());
    }
    //return_chart.push_str("</span>");
    return_chart
}

// -------------------------------------------------------------------------


// Get the average core usage
fn get_disks_available_space(req_disks: &mut sysinfo::Disks) -> (f32,Vec<String>){

    let mut disk_space_usage:Vec<String> = Vec::new();
    let mut disk_tot: u64 = 0;
    for disk in req_disks.list() {
        disk_space_usage.push(format!("{}-{}-{}",disk.name().to_string_lossy(),disk.available_space() as u64,disk.total_space() as u64));
        disk_tot += disk.available_space()
    }

    let disk_avg: f32 = (disk_tot / req_disks.list().len() as u64) as f32;
    // println!("--------------");
    // //println!("cpu cores {}",cpus.len());
    // println!("cpu cores {}",num_cpus::get());
    // println!("cpu phi   {}",num_cpus::get_physical());
    // println!("cpu avg   {}",cpu_avg);

    return (disk_avg,disk_space_usage);

}

// -------------------------------------------------------------------------


fn main() {
    let mut interval: u32 = 2;
    let mut history = 10;
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
            } else if arg == "--disk" && i + 1 < args.len() {
                history = args[i + 1].parse().unwrap_or_else(|_| {
                    panic!("--disk must contain a mount point!")
                });
            }
        }
    }
    if (interval == 0) || (history == 0) {
        panic!("--interval must be greater than 0 and --disk must containt a mount point");
    }


    let mut stats: Vec<f32> = vec![0.0; history];
    // let mut i:f32 =100.0;
    // for ref mut itm in stats.iter(){
    //     *itm = &i.clone();
    //     i = i - stats.len() as f32;
    // }

    let sleep_duration: Duration = Duration::from_secs(interval as u64);

    let mut current_disks = Disks::new_with_refreshed_list();

    loop {
        // Call each function to get all the values we need
        let disks_avg_space = get_disks_available_space(&mut current_disks);

        if stats.len() == history{
            stats.remove(0);
        }
        stats.push(disks_avg_space.0);
        let disk_avg = disks_avg_space.0;

        let disks_usage = disks_avg_space.1;
        let disk_usage_item_maxlen: usize = match disks_usage
            .iter()
            .map(|f|f.len()).max(){
                Some(m) => m,
                None => 0,
            };
        let mut disks_usage_tabulada = String::from("");
        for line in disks_usage.iter(){
            let linelen = line.len();
            let tabbedline = line.replace("-"," ".repeat(disk_usage_item_maxlen-(linelen-1)).as_str());
            disks_usage_tabulada.push_str(format!("{}%\\n",&tabbedline).as_str());
        }

        let disks_chart = get_single_chart(&stats,CHARS,COLORS) ;
        println!("{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\": \"\",\"alt\":\"Avg.Usage: {}%\\n--------------\\n{}\",\"percentage\":{}}}",&disks_chart,&disks_chart,&disk_avg,&disks_usage_tabulada,stats[stats.len()-1] as i32);
        thread::sleep(sleep_duration);
        current_disks.refresh(true);
        //current_sys.refresh_all();

    }

}
