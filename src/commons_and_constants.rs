use dirs;

pub fn user_dir() -> String {
    if let Some(path) = dirs::home_dir(){
        let path_string: String = path.to_string_lossy().into_owned();
        path_string
    } else{
        String::new()
    }
}

pub const JSON_URL:&str = "https://raw.githubusercontent.com/lite-xl/lite-xl-plugins/master/manifest.json";