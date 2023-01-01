mod backup;
mod cli;
mod compress;
mod config;
mod fs;
mod s3;
mod time;

use std::path::PathBuf;
use std::time::SystemTime;

// use chrono::{DateTime, Utc};
use time::get_current_timestamp;

use crate::backup::{do_full_backup, do_incremental_backup};
use crate::cli::parse_args;
use crate::config::parse_configs;
use crate::fs::{filter_files_newer_than, filter_files_with_extension, list_files_in_dir};
// use crate::fs::{filter_files_with_extension, list_files_in_dir};
// use crate::fs::list_files_in_dir;

// fn backup_files() {}

// fn parse_timestamp(filename: &str) -> Option<DateTime<Utc>> {
//     let parts: Vec<&str> = filename.split('.').collect();
//     if parts.len() != 2 {
//         return None;
//     }
//     let timestamp_str = parts[0].split('-').nth(1)?;

//     // Parse the timestamp (in seconds since the Unix epoch) into a DateTime object
//     let timestamp = timestamp_str.parse::<i64>().unwrap();
//     Some(Utc.timestamp(timestamp, 0))
// }

fn main() {
    println!("Hello, world!");

    parse_args();

    let configs = parse_configs(PathBuf::from("../testdata/config"));

    // print number of found configs
    println!("Found {} configs", configs.len());
    // println!("{:?}", configs);

    // for each config:
    // find existing backups
    for config in configs {
        println!("Config: {}", config.app_name);
        let files = match list_files_in_dir(config.local_storage_location.clone().into()) {
            Ok(files) => files,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        let files = match filter_files_with_extension(files, "tar") {
            Ok(files) => files,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // print number of found backups
        println!("Found {} backups", files.len());

        // for file in files {
        //     println!("File: {:?}", file);
        //     parse_backup_filename(file.file_name().unwrap().to_str().unwrap());
        // }

        // do_full_backup(&config);
        do_incremental_backup(&config, SystemTime::now());
    }

    // get remote storage address from each config,

    // list_files(
    //     "/Users/ondrej/Documents/GitHub/bkp/bkp/testdata",
    //     fileList,
    //     std::time::UNIX_EPOCH,
    // );
    // println!("{:?}", fileList)
}
