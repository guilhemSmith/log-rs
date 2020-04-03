mod timestamp;

use crate::timestamp::get_timestamp;

pub fn log_info(msg: &str) {
    println!("[INFO-{}]: {}", get_timestamp(), msg);
}

pub fn log_debug(msg: &str) {
    println!("[DEBUG-{}]: {}", get_timestamp(), msg);
}

pub fn log_warning(msg: &str) {
    println!("[WARNING-{}]: {}", get_timestamp(), msg);
}

pub fn log_error(msg: &str) {
    println!("[ERROR-{}]: {}", get_timestamp(), msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info() {
        log_info("this is an info.");
    }

    #[test]
    fn debug() {
        log_debug("this is a debug.");
    }

    #[test]
    fn error() {
        log_error("this is an error.");
    }

    #[test]
    fn warning() {
        log_warning("this is a warning.");
    }
}