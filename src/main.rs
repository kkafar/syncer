mod cli;
mod env;
mod logging;

use clap::Parser;
use log::info;
use log::trace;
use md5;
use core::panic;
use std::fs;
use std::path;
use xdg;
use rusqlite;



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
    //

    let cli = cli::Cli::parse();

    let _handle = match logging::init() {
        Ok(handle) => handle,
        Err(err) => panic!("{err:?}")
    };

    let xdg_dirs = env::ensure_file_structure_exists();


    match cli.command {
        cli::Command::File(subcmd) => {
            match subcmd.command {
                cli::FileCommand::Add { file } => {
                    trace!("Running FileAdd action");

                },
                cli::FileCommand::Remove { file } => {
                    trace!("Running FileRemove action");

                },
                cli::FileCommand::List => {
                    trace!("Running FileList action")
                },
            }
        }
        cli::Command::Server(subcmd) => {
            match subcmd.command {
                cli::ServerCommand::Start => {
                    trace!("Running ServerStart action");
                },
                cli::ServerCommand::Stop => {
                    trace!("Running ServerStop action");
                },
            }
        }
    }
}
