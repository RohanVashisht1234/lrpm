use reqwest::blocking::Client;

use git2::{self, Repository};
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn download_this_url_at_this_file(dir:String, url: String, file_name: String) -> bool {
    let client = Client::new();

    let response = client.get(url).send().expect("Internet issue");
    if response.status().is_success() {
        let bytes = response.bytes().expect("Internet issue");
        let parent_dir = Path::new(&dir).parent().unwrap();
        if !parent_dir.exists() {
            if let Ok(res) = std::fs::create_dir_all(parent_dir){
                println!("Created {:#?}", res);
            }
            else{
                println!("Unable to create the dir");
            }
        }
        let mut file = File::create(&file_name).expect("msg");
        copy(&mut bytes.as_ref(), &mut file).expect("msg");
        true
    } else {
        println!("a");
        false
    }
}

pub fn git_clone(repo_url: String, clone_path: String, commit_hash: String) {
    match Repository::clone(&repo_url, clone_path) {
        Ok(repo) => {
            println!("Repository cloned successfully to {:?}", repo.path());
            // Do further opermessageations with the cloned repository if needed
        }
        Err(e) => {
            eprintln!("Failed to clone repository: {}", e);
        }
    }
}
