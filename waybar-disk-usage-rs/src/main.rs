
use std::env;
use std::{thread, time::Duration};
use sysinfo::{ProcessesToUpdate,ProcessRefreshKind};

//const COLORSUP:&[&str] =  &["#f299b9","#f288a9","#f29988","#f38877","#f37777","#f36677","#f35577","#f35566","#f74433","#f70011"];
//const COLORSDOWN:&[&str] =&["#97f0cd","#87f0bd","#77f0ad","#87f0ad","#67f09d","#47f08d","#37f08d","#27f08d","#17f08d","#07f08d"];
const COLORSUP:&[&str] =&["##53EDE8","#53E3ED","#53CCED","#53BAED","#53A8ED","#539BED","#538BED","#536FED","#535BED","#6553ED"];
const COLORSDOWN:&[&str] =&["#E7BBF0","#F0B6F2","#F099ED","#F07FD9","#EB71BE","#EB609E","#ED5C92","#ED5379","#ED405A","##ED0C10"];
const CHARSUP: &[&str]=   &["0","b","c","d","e","f","g","h","i","j"];       // font efe-graph.ttf
const CHARSDOWN: &[&str]= &["k","l","m","n","o","p","q","r","s","t"];       // font efe-graph.ttf

fn display_help() {
    println!("Usage: {} [options]", env::current_exe().unwrap().display());
    println!();
    println!("Options:");
    println!("  --interval <seconds>        Set the interval between updates (default: 1)");
    println!("  --history <number>          Set the number of reading to show in the graph (default: 15)");
    println!();
}

// -------------------------------------------------------------------------


// Get the  double chart with metrics un the upper middle annd metrics on bottom middle
fn get_double_chart(up_stats_set: &Vec<u64>,down_stats_set: &Vec<u64>, max_value: &u64, up_symbols:&[&str], down_symbols:&[&str],up_colors:&[&str] ,down_colors:&[&str] ) -> String {

    //let mut return_chart: String = String::from("<span font-family='efe-graph-bold' rise='-4444'>");
    let mut return_chart: String = String::from("");

    // Put all of the core loads into a vector
    for one_stat_up in up_stats_set.iter(){
        let stat_0_to_9: usize = ((((one_stat_up * 100)/max_value) * (up_symbols.len() as u64 - 1 as u64)) / 100 as u64) as usize;
        //let stat_0_to_9: usize = (((one_stat_up * 100)/max_value)  / (up_symbols.len() as u64)) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&up_colors[stat_0_to_9],&up_symbols[stat_0_to_9]).as_str());
    }

    return_chart.push_str("\\r");

    // Put all of the core loads into a vector
    for one_stat_down in down_stats_set.iter(){
        let stat_0_to_9: usize = ((((one_stat_down * 100)/max_value) * (down_symbols.len() as u64 - 1 as u64)) / 100 as u64) as usize;
        //let stat_0_to_9: usize = (((one_stat_down * 100)/max_value)  / (down_symbols.len() as u64)) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&down_colors[stat_0_to_9],&down_symbols[stat_0_to_9]).as_str());
    }

    return_chart
}

// -------------------------------------------------------------------------

// Get the total network (down) usage
fn get_disks_read_and_writen_bytes( req_sys: &sysinfo::System,
                                    polling_secs: &i32) -> (u64,u64){


    let mut read_bytes: u64 = 0;
    let mut written_bytes: u64 = 0;

    for (_pid, process) in req_sys.processes() {
        let disk_usage = process.disk_usage();
        read_bytes += disk_usage.read_bytes;
        written_bytes += disk_usage.written_bytes;
        // println!("read bytes    {}",&read_bytes);
        // println!("written bytes {}",&written_bytes);
    }
    read_bytes = (read_bytes / *polling_secs as u64) as u64;
    written_bytes = (written_bytes / *polling_secs as u64) as u64;


    return ((read_bytes/1000000),(written_bytes/1000000));
}


// -------------------------------------------------------------------------


fn main() {
    let mut history = 15;
    let mut interval: i32 = 2;
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
            };
        }
    }
    if (interval == 0) || (history == 0)  {
        panic!("--interval and --history must be greater than 0");
    }

    let mut read_stats: Vec<u64> =vec![0; history];
    let mut write_stats: Vec<u64> =vec![0; history];

    let sleep_duration: Duration = Duration::from_secs(interval as u64);
    let mut current_sys = sysinfo::System::new_all();

    loop {
        let stats = get_disks_read_and_writen_bytes(&current_sys,&interval);
        let mut highest: u64 = 1;
        println!("read    MBps {}",&stats.0);
        println!("written MBps {}",&stats.1);

        if read_stats.len() == history as usize{
            read_stats.remove(0);
        }
        read_stats.push(stats.0);

        if write_stats.len() == history as usize{
            write_stats.remove(0);
        }
        write_stats.push(stats.1);

        let max_read_stats = match read_stats.iter().max(){
            Some(v) => *v,
            None => 0 as u64,
        };
        let max_write_stats = match write_stats.iter().max(){
            Some(v) => *v,
            None => 0 as u64,
        };
        if max_read_stats > highest{
            highest = max_read_stats;
        }
        if max_write_stats > highest{
            highest = max_write_stats;
        }
        let limits = vec![5,15,30,50,75,100,150,1000,2000,3000,6000,10000,20000];
        let mut max = 0;
        for limit in limits{
            if highest % limit == highest {
                max = limit;
                break;
            } else {
                max =  highest;
            }
        }
        println!("max  MBps {}",&max);

        let read_stats_tot: u64 = read_stats.iter().sum();
        let read_stats_avg: u64 = (read_stats_tot / read_stats.len() as u64) as u64;
        let write_stats_tot: u64 = write_stats.iter().sum();
        let write_stats_avg: u64 = (write_stats_tot / write_stats.len() as u64) as u64;
        let sum_stats_avg: u64 = (read_stats_avg + write_stats_avg) / 2  ;

        let disk_usage_chart = get_double_chart(&read_stats,&write_stats,&max,CHARSUP,CHARSDOWN,COLORSUP,COLORSDOWN);
        println!("{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\":\"\",\"alt\":\"Read      : {} MBps\\rWrite     : {} MBps\\rRange     : 0-{} MBps\\rAvg.Read  : {} MBps\\rAvg.Write : {} MBps\",\"percentage\":{}}}",&disk_usage_chart,&disk_usage_chart,read_stats[read_stats.len()-1] as u64,write_stats[write_stats.len()-1] as u64,&max,&read_stats_avg,&write_stats_avg,&sum_stats_avg);

        current_sys.refresh_processes_specifics(ProcessesToUpdate::All,true,ProcessRefreshKind::nothing().with_disk_usage()); //.refresh_all();
        thread::sleep(sleep_duration);

    }

}
