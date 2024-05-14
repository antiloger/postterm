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
    pub header: Vec<String>,
    /// body for REST
    #[arg(short = 'b')]
    pub body: Vec<String>,
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

pub fn check_x(x: Restargs) {
    if let Some(get_x) = x.get {
        print!("get >> {}", get_x);
    } else if let Some(post_x) = x.post {
        print!("post >> {}", post_x);
    }
}
