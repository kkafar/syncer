mod cli;
mod server;

use clap::Parser;
use md5;
use core::panic;
use std::fs;
use std::path;
use xdg;


const APP_PREFIX: &str = "syncer";


fn main() {
    // let content = include_str!("../data/input01.txt");
    //
    // println!("{content}");
    // let digest = md5::compute(content.as_bytes());
    // println!("{digest:?}");
    //
    // let Ok(binary_content) = fs::read(path::Path::new("data/input01.txt")) else {
    //     return;
    // };
    //
    // println!("{binary_content:?}");
    // let binary_digest = md5::compute(binary_content);
    // println!("{binary_digest:?}");

    let Ok(xdg_dirs) = xdg::BaseDirectories::new() else {
        panic!("Could not create xdg::BaseDirectories, do you have HOME environment variable set?");
    };

    let cli = cli::Cli::parse();

    match cli.command {
        cli::Command::File(subcmd) => {
            match subcmd.command {
                cli::FileCommand::Add { file } => {

                },
                cli::FileCommand::Remove { file } => {

                },
            }
        }
        cli::Command::Server(subcmd) => {
            match subcmd.command {
                cli::ServerCommand::Start => {

                },
                cli::ServerCommand::Stop => {

                },
            }
        }
    }


    let Ok(state_home) = xdg_dirs.create_state_directory(path::Path::new(APP_PREFIX)) else {
        panic!("Could not create state directory. Make sure ... TODO");
    };

    println!("XDG_STATE_HOME={state_home:?}")

}
