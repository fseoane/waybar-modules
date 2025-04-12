
use std::env;
use std::{thread, time::Duration};
use sysinfo::CpuRefreshKind;

const CPU_COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
const CPU_CHARS: &[&str]= &["a","b","c","d","e","f","g","h","i","j"];

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

    let mut return_chart: String = String::from("<span font-family='efe-graph' rise='-4444'>");
    let _chart_avg_percent: f32 = stats_set.iter().copied().sum::<f32>() / stats_set.len() as f32;

    // Put all of the core loads into a vector
    for one_stat in stats_set.iter(){
        let stat_0_to_9: usize = ((one_stat * (symbols.len() as f32 - 1.0)) / 100.0) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&colors[stat_0_to_9],&symbols[stat_0_to_9]).as_str());
    }
    return_chart.push_str("</span>");
    return_chart
}

// -------------------------------------------------------------------------

// Get the average core usage
fn get_cpu_use(req_sys: &mut sysinfo::System) -> f32{

    //std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    req_sys.refresh_cpu_usage();
    let cpu_avg: f32 = req_sys.global_cpu_usage();
    return cpu_avg as f32;
}

// -------------------------------------------------------------------------


fn main() {
    let mut history = 15;
    let mut interval: u32 = 2;
    let args: Vec<String> = env::args().collect();
    let mut stats: Vec<f32> = Vec::new();


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
            }
        }
    }
    if (interval == 0) || (history == 0)  {
        panic!("--interval and --history must be greater than 0");
    }

    let sleep_duration: Duration = Duration::from_secs(interval as u64);
    let mut current_sys = sysinfo::System::new_all();
    current_sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    
    let _current_stats_length =  stats.len();

    loop {
        // Call each function to get all the values we need
        let cpu_avg = get_cpu_use(&mut current_sys);

        if stats.len() == history as usize{
            stats.remove(0);
        }
        stats.push(cpu_avg);
        let stats_tot: f32 = stats.iter().sum();
        let stats_avg: i32 = (stats_tot / stats.len() as f32) as i32;
        thread::sleep(sleep_duration);

        let cpu_chart = get_single_chart(&stats,CPU_CHARS,CPU_COLORS) ;
        println!("{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\": \"\",\"alt\":\"Avg.Usage: {}\",\"percentage\":{}}}",&cpu_chart,&cpu_chart,&stats_avg,stats[stats.len()-1] as i32);

    }

}
