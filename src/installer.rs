use crate::downloaders_and_setters::{download_this_url_at_this_file, git_clone};
use crate::{user_dir, JSON_URL};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string};

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
    let manifest_file = manifest_dir.clone() + "/manifest.json";

    if let Ok(_) = create_dir_all(manifest_dir.clone()) {
        println!("Step 1: {}", "Done ✔".bright_green().bold());
    };
    println!("{}", "Updating manifest".bold().bright_green());
    download_this_url_at_this_file(
        manifest_dir.clone(),
        JSON_URL.to_owned(),
        manifest_file.clone(),
    );
    println!("Step 2: {}", "Done ✔".bright_green().bold());
    println!(
        "{}",
        "Updated manifest.json successfully".bright_green().bold()
    );

    let data = read_to_string(manifest_file.clone()).expect("msg");
    let p: JSON = serde_json::from_str(&data).expect("msg");
    let mut correct_plugin = false;
    for i in p.addons.iter() {
        let cur_id = i.id.to_string() + "\n";
        if cur_id == extension_name {
            correct_plugin = true;
            if let Some(current_path) = &i.path {
                if current_path != &String::new() {
                    println!("ok");
                    let url: String =
                        "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/"
                            .to_string()
                            + current_path;
                    if download_this_url_at_this_file(
                        manifest_dir.clone(),
                        url,
                        litexl_dir.clone() + "/" + current_path,
                    ) {
                        println!(
                            "{}",
                            "Successfully installed your plugin".bold().bright_green()
                        );
                    } else {
                        println!("{}", "Was not able to install your plugin".bold().red());
                    }
                }
            } else if let Some(current_remote) = &i.remote {
                let mut splitted_url = current_remote.split(":");
                _ = splitted_url.next();
                let actual_url = "https:".to_string() + splitted_url.next().unwrap();
                // let hash = splitted_url.next().unwrap().to_string();
                // println!("{}", actual_url);
                // println!("{}", hash);
                // println!("{}", litexl_dir.clone() + "/plugins");
                if git_clone(
                    actual_url,
                    litexl_dir.clone() + "/plugins/" + &extension_name,
                ) {
                    println!(
                        "{}",
                        "Successfully installed your plugin".bold().bright_green()
                    );
                };
            } else {
                println!("{}", "Was not able to install your plugin".bold().red());
            }
        }
    }
    if !correct_plugin{
        println!("{}", "Was not able to find your plugin".bold().red());
    }
}
