mod actions;
mod backup;
mod cli;
mod compress;
mod config;
mod scripts;
mod storage;
mod time;

use crate::cli::parse_args;

fn main() {
    println!("Welcome to bkp");

    parse_args();
}
