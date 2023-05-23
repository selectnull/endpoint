use reqwest::{header::CONTENT_TYPE, Body, Client, Method, Response, Url};
use std::env;
use tokio;

mod cli;
mod config;

async fn send_request(
    method: Method,
    url: Url,
    body: Option<String>,
) -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let req_builder = client.request(method, url);
    let req_builder = match body {
        Some(body_content) => req_builder
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body_content)),
        None => req_builder,
    };
    req_builder.send().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let parsed = match cli::parse(args) {
        Ok(x) => x,
        Err(_err) => {
            eprintln!("Usage: endpoint <options> HTTP_VERB API_ENDPOINT <PAYLOAD>");
            std::process::exit(1);
        }
    };

    let method: Method = parsed.method.parse().unwrap();
    let url: Url = parsed.url.parse().unwrap();

    let resp = send_request(method, url, parsed.body).await?;

    println!("{}", resp.text().await?);

    Ok(())
}
