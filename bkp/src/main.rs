use clap::{Parser, Subcommand};
// use std::process::Command;

mod config;

use crate::config::parse_config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Backs apps up according config file
    Backup,
}

fn find_files_to_backup() {}

fn backup_files() {}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    println!("{:?}", args);

    match &args.command {
        Some(Commands::Backup) => {
            println!("Backup command");
            parse_config();
            find_files_to_backup();
            backup_files();
        }
        None => {
            println!("No command");
        }
    }
}
