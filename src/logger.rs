use crate::timestamp::get_timestamp;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};

pub struct Logger {
    info_output: Box<dyn Write>,
    debug_output: Box<dyn Write>,
    warning_output: Box<dyn Write>,
    error_output: Box<dyn Write>,
    file_options: OpenOptions,
}

pub enum OutputKind<'f> {
    STDOUT,
    STDERR,
    FILE(&'f str),
}

impl Logger {
    pub fn new() -> Self {
        let mut log = Logger {
            info_output: Box::new(io::stdout()),
            debug_output: Box::new(io::stdout()),
            warning_output: Box::new(io::stderr()),
            error_output: Box::new(io::stderr()),
            file_options: OpenOptions::new(),
        };
        log.file_options.write(true).create(true);
        return log;
    }

    pub fn set_output_info(&mut self, kind: OutputKind) {
        match get_write_buffer(kind, &self.file_options) {
            Ok(output) => self.info_output = output, 
            Err(err) => self.error(&format!("Failed to set output for info: {}", err)).unwrap(),
        }
    }

    pub fn set_output_debug(&mut self, kind: OutputKind) {
        match get_write_buffer(kind, &self.file_options) {
            Ok(output) => self.debug_output = output, 
            Err(err) => self.error(&format!("Failed to set output for debug: {}", err)).unwrap(),
        }
    }

    pub fn set_output_warning(&mut self, kind: OutputKind) {
        match get_write_buffer(kind, &self.file_options) {
            Ok(output) => self.warning_output = output, 
            Err(err) => self.error(&format!("Failed to set output for warning: {}", err)).unwrap(),
        }
    }

    pub fn set_output_error(&mut self, kind: OutputKind) {
        match get_write_buffer(kind, &self.file_options) {
            Ok(output) => self.error_output = output, 
            Err(err) => self.error(&format!("Failed to set output for error: {}", err)).unwrap(),
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

fn get_write_buffer(kind: OutputKind, option: &OpenOptions) -> io::Result<Box<dyn Write>> {
    Ok(match kind {
        OutputKind::STDOUT => Box::new(io::stdout()),
        OutputKind::STDERR => Box::new(io::stderr()),
        OutputKind::FILE(path) => Box::new(option.open(path)?),
    })
}

fn log(log_msg: String, output: &mut impl Write) -> io::Result<()> {
    output.write(log_msg.as_bytes())?;
    Ok(())
}
