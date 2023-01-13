// #[macro_use]
extern crate log;
extern crate simplelog;

// use log::{debug, error, info};

use simplelog::*;

use std::fs::OpenOptions;

use crate::globalconfig::GLOBAL_CONFIG;

pub fn create_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            // File::create("bkp.log").unwrap(),
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(GLOBAL_CONFIG.log_file_location.to_string())
                .unwrap(),
        ),
    ])
    .unwrap();

    // error!("Bright red error");
    // info!("This only appears in the log file");
    // debug!("This level is currently not enabled for any logger");
}
