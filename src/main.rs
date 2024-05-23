use arg::{check_x, Cli, Mode};
use clap::Parser;
use send::de_send;
use sendinfo::{BodyData, RequestData};

mod arg;
mod error;
mod projectinfo;
mod response;
mod send;
mod sendinfo;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Open => {
            println!("open");
        }
        Mode::Project(args) => {
            if let Some(cmd) = args.projectnew {
                println!("{}", cmd);
            }

            if let Some(cmd1) = args.projectopen {
                println!("opne {}", cmd1);
            }
        }
        Mode::X(args) => {
            let sendreq = check_x(args).unwrap();
            println!("{:#?}", sendreq);
            de_send(sendreq).await;
        }
    }
}
