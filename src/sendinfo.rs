use crate::error::{PtError, Result};
use core::str;
use regex::Regex;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct HeaderData {
    pub user_agent: String,
    pub header_data: Vec<(String, String)>,
}

impl HeaderData {
    pub fn get_header(data: String) -> Result<HeaderData> {
        let v_data = scrape_form_data(data).unwrap();
        let output = HeaderData {
            user_agent: "postterm/v0.1".into(),
            header_data: v_data,
        };

        Ok(output)
    }
}

#[derive(Debug)]
pub enum BodyData {
    Text(String),
    Json(String),
    FormData(Vec<(String, String)>),
    Binary(String),
}

impl BodyData {
    pub fn get_body(data: String) -> Result<BodyData> {
        let reg = Regex::new(r"\s*:\s*").unwrap();

        let mut pair = reg.splitn(&data, 2);
        if let (Some(typ), Some(value)) = (pair.next(), pair.next()) {
            match typ.trim().to_lowercase().as_str() {
                "json" => Ok(BodyData::Json(value.to_string())),
                "text" => Ok(BodyData::Text(value.trim().to_string())),
                "binary" => Ok(BodyData::Binary(value.trim().to_string())),
                "form-data" => {
                    let form = scrape_form_data(value.to_string()).unwrap();
                    Ok(BodyData::FormData(form))
                }
                _ => Err(PtError::BodyInvalidArg),
            }
        } else {
            Err(PtError::BodyInvalidArg)
        }
    }
}

fn scrape_form_data(data: String) -> Result<Vec<(String, String)>> {
    let mut v_data: Vec<(String, String)> = Vec::new();
    let pair_re = Regex::new(r"\s*,\s*").unwrap();
    let kv_re = Regex::new(r"\s*:\s*").unwrap();

    let pairs: Vec<&str> = pair_re.split(data.as_str()).collect();

    for pair in pairs {
        let mut tup = kv_re.split(pair);
        if let (Some(key), Some(value)) = (tup.next(), tup.next()) {
            v_data.push((key.trim().to_string(), value.trim().to_string()));
        } else {
            return Err(PtError::InvalidArg);
        }
    }

    Ok(v_data)
}
