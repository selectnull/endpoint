use std::fs::File;
use std::io::Read;
use clap::Parser;

use crate::config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(name="method")]
    pub method: String,

    #[clap(name="url")]
    pub url: String,

    #[clap(name="body")]
    pub body: Option<String>,

    #[clap(long, short)]
    pub jwt: Option<String>,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    return args;
}


pub fn get_url(url: String) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        let config = config::read_config();
        match config.base_url {
            Some(base_url) => format!("{}{}", base_url, url),
            None => url,
        }
    }
}

pub fn get_body(body: Option<String>) -> Option<String> {
    match body {
        Some(x) => {
            if x.starts_with("@") {
                let filename = &x[1..];
                let file = File::open(filename);
                let mut contents = String::new();
                file.expect("File not found").read_to_string(&mut contents).unwrap();
                Some(contents)
            } else {
                Some(x)
            }
        },
        None => None
    }
}
