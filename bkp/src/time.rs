extern crate chrono;

use chrono::{DateTime, Utc};

pub fn parse_timestamp(timestamp_str: String) -> Option<DateTime<Utc>> {
    match DateTime::parse_from_rfc3339(&timestamp_str) {
        Ok(dt) => Some(DateTime::<Utc>::from(dt)),
        Err(err) => {
            println!("Error parsing timestamp: {}", timestamp_str);
            println!("Error: {}", err);
            None
        }
    }
}

// pub fn get_current_timestamp() -> String {
//     Utc::now().to_rfc3339()
// }
