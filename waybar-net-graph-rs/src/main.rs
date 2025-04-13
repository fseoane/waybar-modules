
use std::env;
use std::{thread, time::Duration};

const COLORSUP:&[&str] =  &["#f299b9","#f288a9","#f29988","#f38877","#f37777","#f36677","#f35577","#f35566","#f74433","#f70011"];
const COLORSDOWN:&[&str] =&["#97f0cd","#87f0bd","#77f0ad","#87f0ad","#67f09d","#47f08d","#37f08d","#27f08d","#17f08d","#07f08d"];
const CHARSUP: &[&str]=   &[" ","b","c","d","e","f","g","h","i","j"];             // font efe-graph.ttf
const CHARSDOWN: &[&str]= &["k","l","m","n","o","p","q","r","s","t"];       // font efe-graph.ttf

fn display_help() {
    println!("Usage: {} [options]", env::current_exe().unwrap().display());
    println!();
    println!("Options:");
    println!("  --interval <seconds>        Set the interval between updates (default: 1)");
    println!("  --history <number>          Set the number of reading to show in the graph (default: 15)");
    println!("  --interface <net_interface> Set the network interface to monitor (default: eth0)");
    println!("                              or 'total' to monitor all interfaces.");
    println!();
}

// -------------------------------------------------------------------------


// Get the  double chart with metrics un the upper middle annd metrics on bottom middle
fn get_double_chart(up_stats_set: &Vec<u64>,down_stats_set: &Vec<u64>, max_value: &u64, up_symbols:&[&str], down_symbols:&[&str],up_colors:&[&str] ,down_colors:&[&str] ) -> String {

    let mut return_chart: String = String::from("<span font-family='efe-graph-bold' rise='-4444'>");

    // Put all of the core loads into a vector
    for one_stat in up_stats_set.iter(){
        let stat_0_to_9: usize = ((((one_stat * 100)/max_value) * (up_symbols.len() as u64 - 1 as u64)) / 100 as u64) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&up_colors[stat_0_to_9],&up_symbols[stat_0_to_9]).as_str());
    }

    return_chart.push_str("\\r");

    // Put all of the core loads into a vector
    for one_stat in down_stats_set.iter(){
        let stat_0_to_9: usize = ((((one_stat * 100)/max_value) * (up_symbols.len() as u64 - 1 as u64)) / 100 as u64) as usize;
        return_chart.push_str(format!("<span color='{}'>{}</span>",&down_colors[stat_0_to_9],&down_symbols[stat_0_to_9]).as_str());
    }

    return_chart.push_str("</span>");
    return_chart
}

// -------------------------------------------------------------------------

// Get the total network (down) usage
fn get_tot_stat_dwn(req_net: &sysinfo::Networks,
    polling_secs: &i32) -> u64{
    // Get the total bytes recieved by every network interface
    let mut rcv_tot: Vec<u64> = Vec::new();
    for (_interface_name, stat) in req_net.list() {
        rcv_tot.push(stat.received() as u64);
    }

    // Total them and convert the bytes to KB
    let stat_tot: u64 = rcv_tot.iter().sum();
    //let stat_processed = (((stat_tot*8)/(*polling_secs as u64)) / 1024) as u64;
    let stat_processed = (((stat_tot)/(*polling_secs as u64)) / 1000) as u64;
    stat_processed
}

// -------------------------------------------------------------------------

// Get the total network (up) usage
fn get_tot_stat_up( req_net: &sysinfo::Networks,
    polling_secs: &i32) -> u64{
    // Get the total bytes sent by every network interface
    let mut snd_tot: Vec<u64> = Vec::new();
    for (_interface_name, stat) in req_net.list() {
        snd_tot.push(stat.transmitted() as u64);
    }

    // Total them and convert the bytes to KB
    let stat_tot: u64 = snd_tot.iter().sum();
    //let stat_processed = (((stat_tot*8)/(*polling_secs as u64)) / 1024) as u64;
    let stat_processed = (((stat_tot)/(*polling_secs as u64)) / 1000) as u64;
    stat_processed
}

// -------------------------------------------------------------------------

// Get the network (down)  usage for an interface
fn get_iface_stat_dwn(  req_net: &sysinfo::Networks,
                        polling_secs: &i32,
                        iface: &str) -> u64{

    // Get the total bytes recieved by every network interface
    let mut rcv_tot: Vec<u64> = Vec::new();
    for (interface_name, stat) in req_net.list() {
        if interface_name == iface {
            //println!("{:?} rx:{} in {} secs --> {} KBps", interface_name,stat.received(),polling_secs,((stat.received() as i32 /polling_secs) / 1000) as i32 );
            rcv_tot.push(stat.received() as u64);
        }
    }

    // Total them and convert the bytes to KB
    let stat_tot: u64 = rcv_tot.iter().sum();
    //let stat_processed = (((stat_tot*8)/(*polling_secs as u64)) / 1024) as u64;
    let stat_processed = (((stat_tot)/(*polling_secs as u64)) / 1000) as u64;
    stat_processed
}

// -------------------------------------------------------------------------

// Get the network (up) usage for an interface
fn get_iface_stat_up(   req_net: &sysinfo::Networks,
                        polling_secs: &i32,
                        iface: &str) -> u64{

    // Get the total bytes sent by every network interface
    let mut snd_tot: Vec<u64> = Vec::new();
    for (interface_name, stat) in req_net.list() {
        if interface_name == iface {
            //println!("{:?} rx:{} in {} secs --> {} KBps", interface_name,stat.transmitted(),polling_secs,((stat.transmitted() as i32 /polling_secs) / 1000) as i32 );
            snd_tot.push(stat.transmitted() as u64);
        }
    }

    // Total them and convert the bytes to KB
    let stat_tot: u64 = snd_tot.iter().sum();
    //let stat_processed: u64 = (((stat_tot * 8) / (*polling_secs as u64)) / 1024) as u64;
    let stat_processed: u64 = (((stat_tot) / (*polling_secs as u64)) / 1000) as u64;
    stat_processed
}

// -------------------------------------------------------------------------


fn main() {
    let mut history = 15;
    let mut interval: i32 = 2;
    let mut interface = "total";
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
            } else if arg == "--interface" && i + 1 < args.len() {
                interface = args[i + 1].as_str()
            };

        }
    }
    if (interval == 0) || (history == 0)  {
        panic!("--interval and --history must be greater than 0");
    }

    let mut up_stats: Vec<u64> =vec![0; history];
    let mut down_stats: Vec<u64> =vec![0; history];

    let sleep_duration: Duration = Duration::from_secs(interval as u64);
    let mut current_net = sysinfo::Networks::new_with_refreshed_list();

    loop {
        current_net.refresh(true);

        let stat_dwn ;
        let stat_up ;
        let mut highest: u64 = 1;

        if interface == "total" {
            stat_dwn = get_tot_stat_dwn(&current_net,&interval);
            stat_up = get_tot_stat_up(&current_net,&interval);
        }
        else{
            stat_dwn = get_iface_stat_dwn(&current_net,&interval,&interface);
            stat_up = get_iface_stat_up(&current_net,&interval,&interface);
        }

        if up_stats.len() == history as usize{
            up_stats.remove(0);
        }
        up_stats.push(stat_up);

        if down_stats.len() == history as usize{
            down_stats.remove(0);
        }
        down_stats.push(stat_dwn);

        let max_down_stats = match down_stats.iter().max(){
            Some(v) => *v,
            None => 0 as u64,
        };
        let max_up_stats = match up_stats.iter().max(){
            Some(v) => *v,
            None => 0 as u64,
        };
        if max_down_stats > highest{
            highest = max_down_stats;
        }
        if max_up_stats > highest{
            highest = max_up_stats;
        }
        let limits = vec![10,20,30,50,75,100,200,300,400,500,750,1000];
        let mut max = 0;
        for limit in limits{
            if highest % limit == highest {
                max = limit;
                break;
            }
        }

        let up_stats_tot: u64 = up_stats.iter().sum();
        let up_stats_avg: i32 = (up_stats_tot / up_stats.len() as u64) as i32;
        let down_stats_tot: u64 = down_stats.iter().sum();
        let down_stats_avg: i32 = (down_stats_tot / down_stats.len() as u64) as i32;
        let sum_stats_avg: i32 = (up_stats_avg + down_stats_avg) / 2  ;

        let net_chart = get_double_chart(&up_stats,&down_stats,&max,CHARSUP,CHARSDOWN,COLORSUP,COLORSDOWN);
        println!("{{\"text\":\"{}\",\"tooltip\":\"{}\",\"class\":\"\",\"alt\":\"Interface : {}\\rUp        : {} KBps\\rDown      : {} KBps\\rRange     : 0-{} KBps\\rAvg.Up    : {} KBps\\rAvg.Down  : {} KBps\",\"percentage\":{}}}",&net_chart,&net_chart,&interface,up_stats[up_stats.len()-1] as i32,down_stats[down_stats.len()-1] as i32,&max,&up_stats_avg,&down_stats_avg,&sum_stats_avg);
        thread::sleep(sleep_duration);                                                                                               
    }

}
