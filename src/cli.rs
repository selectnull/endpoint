use std::fs::File;
use std::io::Read;

use crate::config;

pub struct Cli {
    pub method: String,
    pub url: String,
    pub body: Option<String>
}

pub fn parse(args: Vec<String>) -> Result<Cli, Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err("Not enough arguments".into())
    }

    let method = args[1].to_uppercase();

    let url = if args[2].starts_with("http") {
        args[2].clone()
    } else {
        let config = config::read_config();
        match config.base_url {
            Some(base_url) => format!("{}{}", base_url, args[2]),
            None => args[2].clone(),
        }
    };

    let body = if args.len() > 3 {
        if args[3].starts_with("@") {
            let mut file = File::open(&args[3][1..])?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Some(contents)
        } else {
            Some(args[3].clone())
        }
    } else {
        None
    };

    Ok(Cli{method, url, body})
}
