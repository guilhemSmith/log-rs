use crate::timestamp::get_timestamp;
use std::io::{self, Write};

pub struct Logger {
    info_output: Box<dyn Write>,
    debug_output: Box<dyn Write>,
    warning_output: Box<dyn Write>,
    error_output: Box<dyn Write>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            info_output: Box::new(io::stdout()),
            debug_output: Box::new(io::stdout()),
            warning_output: Box::new(io::stderr()),
            error_output: Box::new(io::stderr()),
        }
    }

    pub fn info(&mut self, msg: &str) -> io::Result<()> {
        log(format!("[INFO-{}]: {}\n", get_timestamp(), msg), &mut self.info_output)
    }

    pub fn debug(&mut self, msg: &str) -> io::Result<()> {
        log(format!("[DEBUG-{}]: {}\n", get_timestamp(), msg), &mut self.debug_output)
    }

    pub fn warning(&mut self, msg: &str) -> io::Result<()> {
        log(format!("[WARNING-{}]: {}\n", get_timestamp(), msg), &mut self.warning_output)
    }

    pub fn error(&mut self, msg: &str) -> io::Result<()> {
        log(format!("[ERROR-{}]: {}\n", get_timestamp(), msg), &mut self.error_output)
    }
}

fn log(log_msg: String, output: &mut impl Write) -> io::Result<()> {
    output.write(log_msg.as_bytes())?;
    Ok(())
}
