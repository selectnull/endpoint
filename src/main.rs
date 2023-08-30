use reqwest::{header::CONTENT_TYPE, header::AUTHORIZATION, Body, Client, Method, Response, Url};
use tokio;

mod cli;
mod config;

async fn send_request(
    method: Method,
    url: Url,
    body: Option<String>,
    jwt: Option<String>,
) -> Result<Response, reqwest::Error> {

    let client = Client::new();
    let req_builder = client.request(method, url);

    let req_builder = match body {
        Some(body_content) => req_builder
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body_content)),
        None => req_builder,
    };

    let req_builder = if let Some(token) = jwt {
        if !token.is_empty() {
            req_builder.header(AUTHORIZATION, format!("Bearer {}", token))
        } else {
            req_builder
        }
    } else {
        req_builder
    };

    req_builder.send().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();

    let method: Method = Method::from(&args.method.to_uppercase().parse().unwrap());
    let url: Url = Url::from(cli::get_url(args.url).parse().unwrap());
    let body = cli::get_body((&args.body).clone());
    let jwt = args.jwt;

    let resp = send_request(method, url, body, jwt).await?;

    if args.status {
        println!("{}", resp.status());
    }

    println!("\n{}", resp.text().await?);

    Ok(())
}
