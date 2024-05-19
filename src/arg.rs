use crate::{
    error::{PtError, Result},
    sendinfo::{BodyData, HeaderData, RequestData},
};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// seclet mode
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// to work with terminal UI
    Open,
    /// Info about
    Project(InfoArgs),
    /// for Rest test
    X(Restargs),
}

#[derive(Args)]
pub struct Restargs {
    /// GET method <URL>
    #[arg(long)]
    pub get: Option<String>,
    /// POST method <URL>
    #[arg(long)]
    pub post: Option<String>,
    /// PUT method <URL>
    #[arg(long)]
    pub put: Option<String>,
    /// DELETE method <URL>
    #[arg(long)]
    pub delete: Option<String>,
    /// header for REST
    #[arg(short = 't')]
    pub header: Option<String>,
    /// body for REST
    #[arg(short = 'b')]
    pub body: Option<String>,
}

#[derive(Args)]
pub struct InfoArgs {
    /// create new project name
    #[arg(short = 'n')]
    pub projectnew: Option<String>,
    /// open a project <project_name>
    #[arg(short = 'o')]
    pub projectopen: Option<String>,
    /// model [Rest, Tcp]
    #[arg(short = 'm')]
    pub model: Option<String>,
}

pub fn check_x(x: Restargs) -> Result<RequestData> {
    if let Some(get_x) = x.get {
        println!("gett {} b {:?} h {:?}", get_x, x.body, x.header);
        let mut hd: Option<HeaderData> = None;
        let mut bd: Option<BodyData> = None;

        if let Some(head) = x.header {
            hd = Some(HeaderData::get_header(head)?);
        }

        if let Some(body) = x.body {
            bd = Some(BodyData::get_body(body)?);
        }

        Ok(RequestData::new(reqwest::Method::GET, get_x, bd, hd))
    } else if let Some(post_x) = x.post {
        println!("{}", post_x);
        Ok(RequestData::new(reqwest::Method::POST, post_x, None, None))
    } else {
        Err(PtError::ArgNotProvide)
    }
}
