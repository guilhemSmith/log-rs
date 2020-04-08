extern crate chrono;
use chrono::Local;
use std::string::ToString;

/// Get the current timestamp formated to apply to the ISO 8601.
/// # Return Value
///
/// A String containing the current timestamp.
pub fn get_timestamp() -> String {
    let now = Local::now();
    return now.format("%Y-%m-%dT%H:%M:%S%z").to_string();
}
