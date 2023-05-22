use reqwest::{Client, Response, Method, Url, Body, header::CONTENT_TYPE};
use std::env;
use std::fs::File;
use std::io::Read;
use tokio;

mod config;

async fn send_request(method: Method, url: Url, body: Option<String>) -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let req_builder = client.request(method, url);
    let req_builder = match body {
        Some(body_content) => req_builder.header(CONTENT_TYPE, "application/json").body(Body::from(body_content)),
        None => req_builder,
    };
    req_builder.send().await
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: endpoint <options> HTTP_VERB API_ENDPOINT <PAYLOAD>");
        std::process::exit(1);
    }

    let method = args[1].to_uppercase();
    let method: Method = method.parse().unwrap();

    let url = if args[2].starts_with("http") {
        args[2].clone()
    } else {
        let config = config::read_config();
        match config.base_url {
            Some(base_url) => format!("{}{}", base_url, args[2]),
            None => args[2].clone(),
        }
    };

    let url: Url = url.parse().unwrap();

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

    let resp = send_request(method, url, body).await?;

    println!("{}", resp.text().await?);

    Ok(())
}
