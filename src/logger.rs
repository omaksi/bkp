// #[macro_use]
extern crate log;
extern crate simplelog;

// use log::{debug, error, info};

use log::{debug, error};
use simplelog::*;

use std::{fs::OpenOptions, process::exit};

use crate::globalconfig::GLOBAL_CONFIG;

pub fn create_logger() {
    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            // File::create("bkp.log").unwrap(),
            match OpenOptions::new()
                .create(true)
                .append(true)
                .open(GLOBAL_CONFIG.log_file_location.to_string())
            {
                Ok(file) => file,
                Err(e) => {
                    panic!("Unable to open log file for write: {}", e);
                }
            },
        ),
    ]) {
        Ok(_) => debug!("Logger initialized"),
        Err(e) => {
            error!("Unable to initialize logger: {}", e);
            exit(1)
        }
    }
}
