mod actions;
mod backup;
mod cli;
mod compress;
mod config;
mod globalconfig;
mod scripts;
mod storage;
mod time;

// use std::env;

use crate::cli::parse_args;

fn main() {
    println!("Welcome to bkp");

    // env::set_var("RUST_BACKTRACE", "1");

    parse_args();
}
