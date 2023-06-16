use reqwest::{header::CONTENT_TYPE, Body, Client, Method, Response, Url};
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
    let foo = cli::parse_args();

    let method: Method = Method::from(&foo.method.to_uppercase().parse().unwrap());
    let url: Url = Url::from(cli::get_url(foo.url).parse().unwrap());
    let body = cli::get_body((&foo.body).clone());

    let resp = send_request(method, url, body).await?;

    println!("{}", resp.text().await?);

    Ok(())
}
