use std::env;
use std::io::Error;
use std::process::Command;
use std::{thread, time::Duration};



fn display_help() {
    println!("Usage: {} [options]", env::current_exe().unwrap().display());
    println!();
    println!("Options:");
    println!("  --interval <seconds>   Set the interval seconds between updates get refreshed from internet (default: 900)");
    println!();
}

// check updates from network
fn sync_database() {
    // checkupdates --nocolor
    Command::new("checkupdates")
        .args(["--nocolor"])
        .output()
        .expect("failed to execute process");
}

// get updates info without network operations
fn get_updates() -> (u16, String) {
    // checkupdates --nosync --nocolor
    let output = Command::new("checkupdates")
        .args(["--nosync", "--nocolor"])
        .output()
        .expect("failed to execute process");
    match output.status.code() {
        Some(_code) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if stdout.is_empty() {
                return (0, "0".to_string());
            }
            ((stdout.split(" -> ").count() as u16) - 1, stdout)
        }
        None => (0, "0".to_string()),
    }
}


// get updates info without network operations
fn get_aur_updates() -> (u16, String) {
    // checkupdates --nosync --nocolor
    let output = Command::new("checkupdates-with-aur")
        .args(["--nosync", "--nocolor"])
        .output()
        .expect("failed to execute process");
    match output.status.code() {
        Some(_code) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if stdout.is_empty() {
                return (0, "0".to_string());
            }
            ((stdout.split(" -> ").count() as u16) - 1, stdout)
        }
        None => (0, "0".to_string()),
    }
}

fn main() -> Result<(), Error> {

    let mut interval: u32 = 300;  // by default every 900 seconds (15 minutes)
    let mut columns:usize = 1;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--help" {
                display_help();
                return Ok(());
            } else if arg == "--interval" && i + 1 < args.len() {
                interval = args[i + 1].parse().unwrap_or_else(|_| {
                    panic!("--interval must be greater than 0!")
                });
            }
        }
    }

    let sleep_duration: Duration = Duration::from_secs(1);

    if interval == 0 {
        panic!("interval must be greater than 0");
    }

    let mut iter: u32 = interval;

    loop {
        if iter % update_on_iter == 0 {
            sync_database();
            iter = 0;
        }
        let mut longest_line:usize = 0;
        let (updates, mut stdout) = get_updates();
        let (mut aur_updates, mut aur_stdout) = get_aur_updates();
        let mut text_aur_stdout:String =  String::from("");
        let mut text_stdout:String =  String::from("");

        if updates > 0 {
            if updates < 10 {
                columns = 1;
            } else if updates < 20{
                columns = 2;
            } else if updates < 80{
                columns = 3;
            } else if updates >= 80{
                columns = 4;
            }

            let mut padding = [0; 4];
            stdout
                .split_whitespace()
                .enumerate()
                .for_each(|(index, word)| {
                    padding[index % 4] = padding[index % 4].max(word.len())
                });



            stdout = format!(
                "{}",
                stdout
                    .split_whitespace()
                    .enumerate()
                    .map(|(index, word)| {
                        word.to_string() + " ".repeat(padding[index % 4] - word.len()).as_str()
                    })
                    .collect::<Vec<String>>()
                    .chunks(4)
                    .map(|line| line.join(" "))
                    //.find_map(|(i,line)|(i % columns < (columns -1)).then_some(line.join("\t"))                     )
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            let mut iterlines=0;
            for line in stdout.lines(){
                if iterlines % columns < (columns-1){
                    text_stdout = text_stdout + line + "\t | ";
                } else {
                    text_stdout = text_stdout + line + "\n";
                }
                if line.len()>longest_line{
                    longest_line=line.len()  + ( 3* (columns-1));
                }
                iterlines+= 1;
            };
        }

        if aur_updates > 0 {
            let mut padding = [0; 4];
            aur_stdout
                .split_whitespace()
                .enumerate()
                .for_each(|(index, word)| {
                    padding[index % 4] = padding[index % 4].max(word.len())
                });

            aur_stdout = format!(
                "{}",
                aur_stdout
                    .split_whitespace()
                    .enumerate()
                    .map(|(index, word)| {
                        word.to_string() + " ".repeat(padding[index % 4] - word.len()).as_str()
                    })
                    .collect::<Vec<String>>()
                    .chunks(4)
                    .map(|line| line.join(" "))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            let mut iterlines=0;
            for line in aur_stdout.lines(){
                if stdout.contains(line) {
                    aur_updates= aur_updates - 1;
                }
                else {
                    if iterlines % columns < (columns-1){
                        text_aur_stdout = text_aur_stdout + line + "\t | ";
                    } else {
                        text_aur_stdout = text_aur_stdout + line + "\n";
                    }
                    if line.len()>longest_line{
                        longest_line=line.len() + ( 3* (columns-1));
                    }
                    iterlines+= 1;
                }
            };
        }

        if updates > 0 || aur_updates > 0 {
            let mut tooltip = String::from("");
            if updates >0 {
                tooltip = format!("PACMAN ({})\\n{} \\n{}\\n",&updates,"¯".repeat(columns * longest_line),text_stdout.trim_end().replace("\"", "\\\"").replace("\n", "\\n").replace("\t", "\\t"));
            }

            if aur_updates >0 {
                tooltip = format!("{}\\nAUR ({}) \\n{}\\n{}\\n",tooltip,&aur_updates,"¯".repeat(columns * longest_line),text_aur_stdout.trim_end().replace("\"", "\\\"").replace("\n", "\\n").replace("\t", "\\t"));
            }

            let alt = tooltip.clone();

            println!("{{\"text\":\"{}({}+{})\",\"tooltip\":\"{}\",\"class\":\"has-updates\",\"alt\":\"{}\"}}", (&updates+&aur_updates),&updates,&aur_updates, &tooltip, &alt);
        } else {
            println!("{{\"text\":\"\",tooltip\":\"\",\"class\":\"\",\"alt\":\"\"}}");
        }
        iter += 1;
        thread::sleep(sleep_duration);
    }
}
