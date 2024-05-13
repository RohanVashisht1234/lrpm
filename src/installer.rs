use crate::downloaders_and_setters::download_this_url_at_this_file;
use crate::user_dir;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize)]

struct Addon {
    id: String,
    description: String,
    path: Option<String>,
    remote: Option<String>,
}
#[derive(Serialize, Deserialize)]
struct JSON {
    addons: Vec<Addon>,
}

pub fn installer(extension_name: String) {
    let litexl_dir = user_dir() + "/.config/lite-xl";
    let lrpm_dir = user_dir() + "/.config/lrpm";
    let manifest_dir = lrpm_dir + "/bundles";
    let manifest_file = manifest_dir + "/manifest.json";

    println!(
        "{}",
        "Manifests downloaded successfully!".bold().bright_green()
    );

    let data = read_to_string(manifest_file).expect("msg");
    let p: JSON = serde_json::from_str(&data).expect("msg");
    for i in p.addons.iter() {
        let cur_id = i.id.to_string() + "\n";
        println!("{}", extension_name);
        if cur_id == extension_name {
            if let Some(current_path) = &i.path {
                if current_path != &String::new() {
                    println!("ok");
                    let url: String =
                        "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/"
                            .to_string()
                            + current_path;
                    if download_this_url_at_this_file(url, litexl_dir.clone() + "/" + current_path)
                    {
                        println!("Successfully installed your plugin");
                    } else {
                        println!("Was not able to install your plugin");
                    }
                }
            } else if let Some(current_remote) = &i.remote {
                
            }
        }
    }
}
