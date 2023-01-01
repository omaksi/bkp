extern crate chrono;

// use crate::time::chrono::TimeZone;
use chrono::{DateTime, TimeZone, Utc};

pub fn parse_timestamp(timestamp_str: String) -> Option<DateTime<Utc>> {
    // Parse the timestamp (in seconds since the Unix epoch) into a DateTime object
    match timestamp_str.parse::<i64>() {
        Ok(timestamp) => match Utc.timestamp_opt(timestamp, 0) {
            chrono::offset::LocalResult::None => None,
            chrono::offset::LocalResult::Single(dt) => Some(dt),
            chrono::offset::LocalResult::Ambiguous(_, _) => None,
        },
        Err(_) => None,
    }
}

pub fn get_current_timestamp() -> String {
    Utc::now().to_rfc3339()
}
