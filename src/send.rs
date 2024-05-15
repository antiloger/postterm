use crate::sendinfo::{BodyData, HeaderData, RequestData};
use reqwest::header::HeaderValue;
use reqwest::{header, Client, Method, RequestBuilder};
pub async fn de_send(data: RequestData) {
    let client = reqwest::Client::new();
    let builder = build_request(&client, &data.url, data.method, data.header, data.body).await;
    let response = builder.send().await;
    println!("response: {response:?}")
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
        header_map.insert(
            header::ACCEPT,
            HeaderValue::from_str(&head.accept).expect("Invalid Accept header"),
        );
        header_map.insert(
            header::ACCEPT_ENCODING,
            HeaderValue::from_str(&head.accept_encoding).expect("invalid accept_encoding"),
        );
        // TODO: add for loop to custom headers
        builder = builder.headers(header_map);
    }

    if let Some(data) = body {
        // TODO: complete the convert part
        match data {
            BodyData::Text(text) => {
                println!("body is text: {}", text);
                builder = builder.body(text);
            }
            BodyData::Binary(bin) => {
                println!("body is binary: {}", bin);
            }
            BodyData::Json(json) => {
                println!("body is json: {}", json);
            }
            BodyData::FormData(_) => {
                println!("body is fromdata: ");
            }
        }
    }

    builder
}
