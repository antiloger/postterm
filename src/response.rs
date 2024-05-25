use std::collections::HashMap;

pub struct Response {
    pub status: reqwest::StatusCode,
    pub httpVer: reqwest::Version,
    pub header: HashMap<String, String>,
    pub body: Option<String>,
    pub cookies: bool,
}

pub enum ResponseBody {
    Binary(u8),
    Text(String),
}

impl Response {
    pub async fn create_response(reqres: &reqwest::Response) -> Response {
        let set_header: HashMap<String, String> = reqres
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_string(),
                    v.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect();
        let set_cookie = set_header.contains_key("Set-Cookie");
        Response {
            status: reqres.status(),
            httpVer: reqres.version(),
            header: set_header,
            body: None,
            cookies: set_cookie,
        }
    }

    pub async fn set_body(&mut self, reqres: &reqwest::Response) {}
}
