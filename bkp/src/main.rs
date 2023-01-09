mod actions;
mod cli;
mod compress;
mod config;
mod storage;
mod time;

use crate::cli::parse_args;

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
}
