use crate::error::Result;
use crate::response::Response;
use crate::sendinfo::{BodyData, HeaderData, RequestData};
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{header, Client, Method, RequestBuilder};
use serde_json::Value;

pub async fn de_send(data: RequestData) -> Result<Response> {
    let client = reqwest::Client::new();
    let builder = build_request(&client, &data.url, data.method, data.header, data.body).await;
    let response = builder.send().await;
    let head = response.unwrap().headers().clone();
    println!("{:#?}", head);
}

pub async fn build_request(
    client: &Client,
    url: &str,
    method: Method,
    header: Option<HeaderData>,
    body: Option<BodyData>,
) -> RequestBuilder {
    let mut builder = client.request(method, url);

    if let Some(head) = header {
        let mut header_map = header::HeaderMap::new();
        header_map.insert(
            header::USER_AGENT,
            HeaderValue::from_str(&head.user_agent).expect("Invalid user_agent header"),
        );

        for (key, value) in head.header_data {
            header_map.insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                HeaderValue::from_bytes(value.as_bytes()).unwrap(),
            );
        }
        builder = builder.headers(header_map);
    }

    if let Some(data) = body {
        // TODO: complete the convert part, add error handling
        match data {
            BodyData::Text(text) => {
                println!("body is text: {}", text);
                builder = builder.body(text);
            }
            BodyData::Binary(bin) => {
                println!("body is binary: {}", bin);
            }
            BodyData::Json(json) => {
                let json_body: Value = serde_json::from_str(&json).unwrap();
                builder = builder.json(&json_body);
                println!("body is json: {}", json_body);
            }
            BodyData::FormData(_) => {
                println!("body is fromdata: ");
            }
        }
    }

    builder
}
