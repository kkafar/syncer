mod cli;

use clap::Parser;
use md5;
use std::fs;
use std::path;


fn main() {
    let content = include_str!("../data/input01.txt");

    println!("{content}");
    let digest = md5::compute(content.as_bytes());
    println!("{digest:?}");

    let Ok(binary_content) = fs::read(path::Path::new("data/input01.txt")) else {
        return;
    };

    println!("{binary_content:?}");
    let binary_digest = md5::compute(binary_content);
    println!("{binary_digest:?}");


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
}
