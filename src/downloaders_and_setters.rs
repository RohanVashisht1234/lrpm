use colored::Colorize;
use reqwest::blocking::Client;

use git2::{self, Repository};
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn download_this_url_at_this_file(dir: String, url: String, file_name: String) -> bool {
    let client = Client::new();
    let response = client.get(url).send().expect("Internet issue");
    if response.status().is_success() {
        let bytes = response.bytes().expect("Internet issue");
        let main_path = Path::new(&dir);
        if let Ok(_) = std::fs::create_dir_all(main_path) {
        } else {
            println!("{}", "Unable to create the dir".bold().red());
        }
        println!("{}", file_name);
        let mut file = File::create(&file_name).expect("msg");
        copy(&mut bytes.as_ref(), &mut file).expect("msg");
        true
    } else {
        println!("a");
        false
    }
}

pub fn git_clone(repo_url: String, clone_path: String) -> bool {
    match Repository::clone(&repo_url, clone_path) {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}
