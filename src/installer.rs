use crate::user_dir;
use crate::commons_and_constants::JSON_URL;
use std::fs::{self, read_to_string, File};
use std::io::copy;
use std::path::Path;
use reqwest::blocking::Client;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]

struct Addon {
    id: String,
    description: String
}
#[derive(Serialize, Deserialize)]
struct JSON {
    addons: Vec<Addon>,
}

pub fn installer(extension_name:String){
    let lrpm_dir = user_dir() + "/.config/lrpm";
    let manifest_dir = lrpm_dir + "/bundles";
    let manifest_file = manifest_dir + "/manifest.json";
    let client = Client::new();

    // Send a GET request to the URL and get the response
    let response = client.get(JSON_URL).send().expect("Internet issue");
    if response.status().is_success() {
        // Get the file data as bytes
        let bytes = response.bytes().expect("Internet issue");

        // Create directories if they don't exist
        let parent_dir = Path::new(&manifest_file).parent().unwrap();
        if !parent_dir.exists() {
            let _ = std::fs::create_dir_all(parent_dir);
        }

        // Create a file at the specified path
        let mut file = File::create(&manifest_file).expect("msg");

        // Write the downloaded file data to the file
        copy(&mut bytes.as_ref(), &mut file).expect("msg");

        println!("{}", "Manifests downloaded successfully!".bold().bright_green());

        let data = read_to_string(&manifest_file).expect("msg");
        let p:JSON = serde_json::from_str(&data).expect("msg");
        for i in p.addons.iter(){
            println!("{}             {}", i.id, termimad::inline(&i.description));
        };

    } else {
        println!("Failed to download file: {}", response.status());
    }
}
