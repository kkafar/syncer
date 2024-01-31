use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    File(FileArgs),
    Server(ServerArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct FileArgs {
    #[command(subcommand)]
    pub command: FileCommand,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct ServerArgs {
    #[command(subcommand)]
    pub command: ServerCommand,
}


#[derive(Debug, Subcommand)]
pub enum FileCommand {
    Add {
        #[arg(short, long)]
        file: PathBuf,
    },
    Remove {
        #[arg(short, long)]
        file: PathBuf,
    }
}

#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    Start,
    Stop,
}
