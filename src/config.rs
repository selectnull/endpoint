use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub base_url: Option<String>,
}

fn get_config_file_path() -> Option<PathBuf> {
    let mut path = env::current_dir().expect("Failed to get current directory");
    let root = Path::new("/");

    loop {
        let config_path = path.join(".endpoint.toml");
        if config_path.exists() && config_path.is_file() {
            return Some(config_path);
        }
        if path == root {
            return None;
        }
        path.pop();
    }
}

pub fn read_config() -> Config {
    let config_path = match get_config_file_path() {
        Some(path) => path,
        None => return Config { base_url: None },
    };

    let mut config_string = String::new();
    File::open(&config_path)
        .unwrap()
        .read_to_string(&mut config_string)
        .unwrap();
    toml::from_str(&config_string).unwrap()
}
