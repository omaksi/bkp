mod actions;
mod backup;
mod cli;
mod compress;
mod config;
mod globalconfig;
mod logger;
mod scripts;
mod storage;
mod time;

// use std::env;

use crate::{cli::parse_args, logger::create_logger};

fn main() {
    println!("Welcome to bkp");

    // env::set_var("RUST_BACKTRACE", "1");

    create_logger();

    parse_args();
}
