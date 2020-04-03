extern crate chrono;
use chrono::Utc;
use std::string::ToString;

pub fn get_timestamp() -> String {
    let now = Utc::now();
    return now.format("%Y-%m-%dT%H:%M:%S%z").to_string();
}