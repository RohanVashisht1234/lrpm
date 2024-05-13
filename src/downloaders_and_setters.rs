use crate::commons_and_constants::JSON_URL;
use crate::user_dir;
use colored::Colorize;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::{self, read_to_string, File};
use std::io::copy;
use std::path::Path;

pub fn download_this_url_at_this_file(url: String, file_name: String) -> bool {
    let client = Client::new();

    let response = client.get(url).send().expect("Internet issue");
    if response.status().is_success() {
        let bytes = response.bytes().expect("Internet issue");
        let parent_dir = Path::new(&file_name).parent().unwrap();
        if !parent_dir.exists() {
            let _ = std::fs::create_dir_all(parent_dir);
        }
        let mut file = File::create(&file_name).expect("msg");
        copy(&mut bytes.as_ref(), &mut file).expect("msg");
        return true;
    } else {
        println!("a");
        return false;
    }
}
