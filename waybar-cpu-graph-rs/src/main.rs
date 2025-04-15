
use std::env;
use std::{thread, time::Duration};
use std::process::Command;
use sysinfo::CpuRefreshKind;
extern crate num_cpus;

const COLORS:&[&str] = &["#96faf7","#66f1d7","#67f08d","#85f066","#f0ea66","#f0b166","#f09466","#f28888","#f37777","#f85555"];
//const CHARS: &[&str]=  &["a","b","c","d","e","f","g","h","i","j"];
const CHARS: &[&str]=  &[" ","▁","▂","▃","▄","▅","▆","▇","█","█"];   // with mono nerd fonts

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
//  cpuUsage=$({ head -n1 /proc/stat;sleep 0.2;head -n1 /proc/stat; } | awk '/^cpu /{u=$2-u;s=$4-s;i=$5-i;w=$6-w}END{print int(0.5+100*(u+s+w)/(u+s+i+w))}')
// get updates info without network operations
fn get_cpu_usage_from_proc() -> f32 {
    // checkupdates --nosync --nocolor
    let output1 = Command::new("head")
        .args(["-n1", "/proc/stat"])
        .output()
        .expect("failed to execute process");
    let stdout1 = String::from_utf8_lossy(&output1.stdout).to_string();
    let stdout1_vec: Vec<String>=stdout1.split_whitespace().map(|s| s.to_string()).collect();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    let output2 = Command::new("head")
        .args(["-n1", "/proc/stat"])
        .output()
        .expect("failed to execute process");
    let stdout2 = String::from_utf8_lossy(&output2.stdout).to_string();
    let stdout2_vec: Vec<String> = stdout2.split_whitespace().map(|s| s.to_string()).collect();
    
    // println!("stdout1\n{} ", &stdout1);
    // println!("stdout2\n{} ", &stdout2);

    // > cat /proc/stat
    //      cpu  2255 34 2290 22625563 6290 127 456
    // where the meanings of the columns are as follows, from left to right:
    //      user: normal processes executing in user mode
    //      nice: niced processes executing in user mode
    //      system: processes executing in kernel mode
    //      idle: twiddling thumbs
    //      iowait: waiting for I/O to complete
    //      irq: servicing interrupts
    //      softirq: servicing softirqs


    let user1: i32 = stdout1_vec[1].parse().unwrap();
    let user2: i32 = stdout2_vec[1].parse().unwrap();
    //let niced1: i32 = stdout1_vec[3].parse().unwrap();
    //let niced2: i32 = stdout2_vec[3].parse().unwrap();
    let system1: i32 = stdout1_vec[3].parse().unwrap();
    let system2: i32 = stdout2_vec[3].parse().unwrap();
    let idle1: i32 = stdout1_vec[4].parse().unwrap();
    let idle2: i32 = stdout2_vec[4].parse().unwrap();
    let iowait1: i32 = stdout1_vec[5].parse().unwrap();
    let iowait2: i32 = stdout2_vec[5].parse().unwrap();

    // println!("user1 {} user2 {}", &user1,&user2);
    // println!("system1 {} system2 {}", &system1,&system2);
    // println!("idle1 {} idle2 {}", &idle1,&idle2);
    // println!("wait1 {} wait2 {}", &iowait1,&iowait2);

    let user = user2-user1;
    //let niced = niced2-niced1;
    let system = system2-system1;
    let idle = idle2-idle1;
    let iowait = iowait2-iowait1;

    //let cpu_usage: f32 = 0.5 + 100.0 * ((user + system + niced + iowait) as f32) / ((user + system + idle + niced + iowait) as f32);
    let cpu_usage: f32 = 0.5 + 100.0 * ((user + system + iowait) as f32) / ((user + system + idle + iowait) as f32);
    return cpu_usage;

}

// Get the average core usage
fn get_cpu_use(req_sys: &mut sysinfo::System) -> (f32,Vec<String>){

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    let mut cores_usage:Vec<String> = Vec::new();

    for core in req_sys.cpus() {
        cores_usage.push(format!("{}-{}",core.name(),core.cpu_usage() as i32));
    }
    //let cpu_avg1: f32 = req_sys.global_cpu_usage();

    let cpu_avg: f32 = get_cpu_usage_from_proc();


    // let mut cpus: Vec<f32> = Vec::new();
    // for core in req_sys.cpus() {
    //     cores_usage.push(format!("{}-{}",core.name(),core.cpu_usage() as i32));
    //     cpus.push(core.cpu_usage());
    // }

    // // Get the average load
    // let cpu_tot: f32 = cpus.iter().sum();
    // let cpu_avg: f32 = cpu_tot / cpus.len() as f32;
    // println!("--------------");
    // //println!("cpu cores {}",cpus.len());
    // println!("cpu cores {}",num_cpus::get());
    // println!("cpu phi   {}",num_cpus::get_physical());
    // println!("cpu avg   {}",cpu_avg);

    return (cpu_avg,cores_usage);

}

// -------------------------------------------------------------------------


fn main() {
    let mut history = 15;
    let mut interval: u32 = 2;
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
            }
        }
    }
    if (interval == 0) || (history == 0) {
        panic!("--interval and --history must be greater than 0");
    }


    let mut stats: Vec<f32> = vec![99.0; history];
    // let mut i:f32 =100.0;
    // for ref mut itm in stats.iter(){
    //     *itm = &i.clone();
    //     i = i - stats.len() as f32;
    // }

    let sleep_duration: Duration = Duration::from_secs(interval as u64);
    let mut current_sys = sysinfo::System::new_all();
    current_sys.refresh_cpu_specifics(CpuRefreshKind::nothing().with_cpu_usage());
    //current_sys.refresh_all();
    loop {
        // Call each function to get all the values we need
        let cpu_avg = get_cpu_use(&mut current_sys);

        if stats.len() == history{
            stats.remove(0);
        }
        stats.push(cpu_avg.0);

        let stats_tot: f32 = stats.iter().sum();
        let stats_avg: i32 = (stats_tot / stats.len() as f32) as i32;

        let cores_usage = cpu_avg.1;
        let cores_usage_item_maxlen: usize = match cores_usage
            .iter()
            .map(|f|f.len()).max(){
                Some(m) => m,
                None => 0,
            };
        let mut cores_usage_tabulada = String::from("");
        for line in cores_usage.iter(){
            let linelen = line.len();
            let tabbedline = line.replace("-"," ".repeat(cores_usage_item_maxlen-(linelen-1)).as_str());
            cores_usage_tabulada.push_str(format!("{}%\\n",&tabbedline).as_str());
        }

        let cpu_chart = get_single_chart(&stats,CHARS,COLORS) ;
        println!("{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\": \"\",\"alt\":\"Avg.Usage: {}%\\n--------------\\n{}\",\"percentage\":{}}}",&cpu_chart,&cpu_chart,&stats_avg,&cores_usage_tabulada,stats[stats.len()-1] as i32);
        thread::sleep(sleep_duration);
        current_sys.refresh_cpu_usage();
        //current_sys.refresh_all();

    }

}
