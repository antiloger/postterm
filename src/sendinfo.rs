pub struct RequestData {
    pub method: reqwest::Method,
    pub url: String,
    pub header: Option<HeaderData>,
    pub body: Option<BodyData>,
}

impl RequestData {
    pub fn new(
        mtd: reqwest::Method,
        u: String,
        bd: Option<BodyData>,
        hd: Option<HeaderData>,
    ) -> RequestData {
        RequestData {
            method: mtd,
            url: u,
            header: hd,
            body: bd,
        }
    }
}

pub struct HeaderData {
    pub user_agent: String,
    pub accept: String,
    pub accept_encoding: String,
    pub header_data: Vec<(String, String)>,
}

impl HeaderData {
    pub fn default() -> Self {
        HeaderData {
            user_agent: "postterm/v0.1".into(),
            accept: "*/*".into(),
            accept_encoding: "gzip, deflate".into(),
            header_data: Vec::new(),
        }
    }
}

pub enum BodyData {
    Text(String),
    Json(String),
    FormData(Vec<(String, String)>),
    Binary(String),
}
