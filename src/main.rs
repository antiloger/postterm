use arg::{check_x, Cli, Mode};
use clap::Parser;

mod arg;
mod projectinfo;
mod sendinfo;

fn main() {
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
            check_x(args);
        }
    }
}
