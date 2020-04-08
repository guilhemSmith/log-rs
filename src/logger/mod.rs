mod enums;
mod write_buffer;

pub use enums::{Level, OutputKind};
pub use write_buffer::WriteBuffer;

use crate::timestamp::get_timestamp;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;

/// A logger that can log messages on different level to different output.  
/// The output of each level can be configured to eventually be same for all, or unique.
pub struct Logger<'o> {
    file_options: OpenOptions,
    level_outputs: HashMap<Level, OutputKind<'o>>,
    write_buffers: HashMap<OutputKind<'o>, WriteBuffer>,
    format: String,
}

impl<'o> Logger<'o> {
    /// Create a new logger.
    /// By default, `INFO` and `DEBUG` are writting to `stdout`,
    /// and `WARNING` and `ERROR` write to `stderr`.
    ///
    /// # Return Value
    ///
    /// The new logger if the function succeeded, `None` otherwise.
    pub fn new() -> Option<Self> {
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);

        let mut map_level = HashMap::new();
        map_level.insert(Level::INFO, OutputKind::STDOUT);
        map_level.insert(Level::DEBUG, OutputKind::STDOUT);
        map_level.insert(Level::WARNING, OutputKind::STDERR);
        map_level.insert(Level::ERROR, OutputKind::STDERR);

        let mut stdout = if let Ok(buff) = WriteBuffer::new(&OutputKind::STDOUT, &options) {
            buff
        } else {
            return None;
        };
        let mut stderr = if let Ok(buff) = WriteBuffer::new(&OutputKind::STDERR, &options) {
            buff
        } else {
            return None;
        };
        stdout.increase_count();
        stderr.increase_count();

        let mut map_buffer = HashMap::new();
        map_buffer.insert(OutputKind::STDOUT, stdout);
        map_buffer.insert(OutputKind::STDERR, stderr);

        let log = Logger {
            file_options: options,
            level_outputs: map_level,
            write_buffers: map_buffer,
            format: String::from("[%l](%t) - %m"),
        };

        return Some(log);
    }

    /// Configure the format of the log.
    /// 
    /// # Arguments
    /// 
    /// * `new_format` - The format to be used for logging message.  
    /// Any occurrence of `%l` will be replaced by the level, `%t` by the timestamp, `%m` by the message.  
    pub fn config_format<S: ToString + ?Sized>(&mut self, new_format: &S) {
        self.format = new_format.to_string();

    }

    /// Configure the output of the given level.  
    ///
    /// # Arguments
    ///
    /// * `kind` - The new output kind to be used by the level.
    /// * `level` - The level to configure.
    ///
    /// # Return Value
    ///
    /// The io error if something went wrong and the configuration was not possible.
    fn config_output(&mut self, kind: OutputKind<'o>, level: Level) -> io::Result<()> {
        match self.write_buffers.get_mut(&kind) {
            None => {
                let buffer = WriteBuffer::new(&kind, &self.file_options)?;
                self.write_buffers.insert(kind.clone(), buffer);
            }
            Some(buffer) => buffer.increase_count(),
        };
        let output = self.level_outputs.get_mut(&level).unwrap();
        if match self.write_buffers.get_mut(output) {
            None => false,
            Some(buffer) => buffer.decrease_count(),
        } {
            self.write_buffers.remove(output);
        }
        *output = kind;
        Ok(())
    }

    /// Configure the output for `INFO`.  
    ///
    /// # Arguments
    ///
    /// * `kind` - The new output kind to be used by the info.
    pub fn config_info(&mut self, kind: OutputKind<'o>) {
        if let Err(err) = self.config_output(kind, Level::INFO) {
            self.error(&format!("Failed to configure INFO: {}", err));
        }
    }

    /// Configure the output for `DEBUG`.  
    ///
    /// # Arguments
    ///
    /// * `kind` - The new output kind to be used by the debug.
    pub fn config_debug(&mut self, kind: OutputKind<'o>) {
        if let Err(err) = self.config_output(kind, Level::DEBUG) {
            self.error(&format!("Failed to configure DEBUG: {}", err));
        }
    }

    /// Configure the output for `WARNING`.  
    ///
    /// # Arguments
    ///
    /// * `kind` - The new output kind to be used by the warning.
    pub fn config_warning(&mut self, kind: OutputKind<'o>) {
        if let Err(err) = self.config_output(kind, Level::WARNING) {
            self.error(&format!("Failed to configure WARNING: {}", err));
        }
    }

    /// Configure the output for `ERROR`.  
    ///
    /// # Arguments
    ///
    /// * `kind` - The new output kind to be used by the error.
    pub fn config_error(&mut self, kind: OutputKind<'o>) {
        if let Err(err) = self.config_output(kind, Level::ERROR) {
            self.error(&format!("Failed to configure ERROR: {}", err));
        }
    }

    /// Send a message on the given level.  
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write.
    /// * `level` - The level to write on.
    ///
    /// # Return Value
    ///
    /// The io error if something went wrong and the logging was not possible.
    fn write_log(&mut self, msg: &str, level: Level) {
        if let Some(buffer) = self
            .write_buffers
            .get_mut(self.level_outputs.get(&level).unwrap())
        {
            let mut log_msg = self
                .format
                .replace("%l", &level.to_string())
                .replace("%t", &get_timestamp().to_string())
                .replace("%m", msg);
            log_msg.push('\n');
            if let Err(err) = buffer.log(log_msg) {
                match level {
                    Level::ERROR => {}
                    _ => self.error(&format!("Failed to log to `{}`: {}", level, err)),
                }
            };
        };
    }

    /// Send a message on `INFO`.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write on `INFO`.
    pub fn info(&mut self, msg: &str) {
        self.write_log(msg, Level::INFO);
    }

    /// Send a message on `DEBUG`.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write on `DEBUG`.
    pub fn debug(&mut self, msg: &str) {
        self.write_log(msg, Level::DEBUG);
    }

    /// Send a message on `WARNING`.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write on `WARNING`.
    pub fn warning(&mut self, msg: &str) {
        self.write_log(msg, Level::WARNING);
    }

    /// Send a message on `ERROR`.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to write on `ERROR`.
    pub fn error(&mut self, msg: &str) {
        self.write_log(msg, Level::ERROR);
    }
}
