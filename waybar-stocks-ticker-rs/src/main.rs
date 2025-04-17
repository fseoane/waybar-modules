use log::{error, info, warn};
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use regex::Regex;

fn get_html(url: &str) -> String {
    let resp = reqwest::blocking::get(url);
	match resp {
        Ok(response) => match response.text() {
            Ok(r) => r,
            Err(e) => {
                error!("[X] Error '{}' getting the public IP from https://ipecho.net/plain", e);
                String::from("")
            }
        },
        Err(e) => {
            error!("[X] Error '{}' getting the public IP from https://ipecho.net/plain", e);
            String::from("")
        }
    }
}

fn get_html_filtered(url: &str, reg_expression: &str) -> Vec<String> {
	let response = match reqwest::blocking::get(url) {
        Ok(resp) => match resp.text() {
            Ok(r) => r,
            Err(e) => {
                error!("[X] Error '{}' getting the public IP from https://ipecho.net/plain", e);
                String::from("")
            }
        },
        Err(e) => {
            error!("[X] Error '{}' getting the public IP from https://ipecho.net/plain", e);
            String::from("")
        }
    }
	let re = Regex::new(&reg_expression).unwrap();
    let result = re.find_iter(&response)
        .map(|m| String::from(m.as_str()))
        .collect::<Vec<String>>();
    return result;
}



fn main(){
	let url: String = String::from("https://t.me/s/DonTorrent?before=250"); //https://ipecho.net/plain

	let reg_expr= "Dominio Oficial <i class=\"emoji\" style=\"background-image:url('//telegram.org/img/emoji/40/E29C85.png')\"><b>✅</b></i> (29/03/2025) <a href="https://dontorrent.website/" target="_blank" rel="noopener">https://dontorrent.website</a> (Disponible)"
	let response = get_html_filtered(&url,);






	println!("response:\n{:?}", response);



	//"Dominio Oficial ✅"
	//<div class="tgme_widget_message_text js-message_text" dir="auto">Dominio Oficial <i class="emoji" style="background-image:url('//telegram.org/img/emoji/40/E29C85.png')"><b>✅</b></i> (29/03/2025) <a href="https://dontorrent.website/" target="_blank" rel="noopener">https://dontorrent.website</a> (Disponible)</div>
}