mod write_buffer;
mod enums;

pub use enums::{OutputKind, Level};
pub use write_buffer::WriteBuffer;

use crate::timestamp::get_timestamp;
use std::io;
use std::fs::OpenOptions;
use std::collections::HashMap;

pub struct Logger<'o> {
    file_options: OpenOptions,
    level_outputs: HashMap<Level, OutputKind<'o>>,
    write_buffers: HashMap<OutputKind<'o>, WriteBuffer>,
}

impl<'o> Logger<'o> {
    pub fn new() -> io::Result<Self> {
        let mut options = OpenOptions::new();
        options.write(true).create(true);

        let mut map_level = HashMap::new();
        map_level.insert(Level::INFO, OutputKind::STDOUT);
        map_level.insert(Level::DEBUG, OutputKind::STDOUT);
        map_level.insert(Level::WARNING, OutputKind::STDERR);
        map_level.insert(Level::ERROR, OutputKind::STDERR);

        let mut stdout = WriteBuffer::new(&OutputKind::STDOUT, &options)?;
        let mut stderr = WriteBuffer::new(&OutputKind::STDERR, &options)?;
        stdout.increase_count();
        stderr.increase_count();

        let mut map_buffer = HashMap::new();
        map_buffer.insert(OutputKind::STDOUT, stdout);
        map_buffer.insert(OutputKind::STDERR, stderr);

        let log = Logger {
            file_options: options,
            level_outputs: map_level,
            write_buffers: map_buffer,
        };

        return Ok(log);
    }

    fn config_output(&mut self, kind: OutputKind<'o>, level: Level) -> io::Result<()> {
        match self.write_buffers.get_mut(&kind) {
            None => {
                let buffer = WriteBuffer::new(&kind, &self.file_options)?;
                self.write_buffers.insert(kind.clone(), buffer);
            },
            Some(buffer) => buffer.increase_count()
        };
        let output = self.level_outputs.get_mut(&level).unwrap();
        if match self.write_buffers.get_mut(output) {
            None => false,
            Some(buffer) => buffer.decrease_count()
        } {
            self.write_buffers.remove(output);
        }
        *output = kind;
        Ok(())
    }

    pub fn config_info(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        self.config_output(kind, Level::INFO)
    }

    pub fn config_debug(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        self.config_output(kind, Level::DEBUG)
    }

    pub fn config_warning(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        self.config_output(kind, Level::WARNING)
    }

    pub fn config_error(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        self.config_output(kind, Level::ERROR)
    }

    fn write_log(&mut self, msg: &str, level: Level) -> io::Result<()> {
        if let Some(buffer) = self.write_buffers.get_mut(self.level_outputs.get(&level).unwrap()) {
            buffer.log(format!("[{}-{}]: {}\n", level, get_timestamp(), msg))?;
        };
        Ok(())
    }

    pub fn info(&mut self, msg: &str) -> io::Result<()> {
        self.write_log(msg, Level::INFO)
    }

    pub fn debug(&mut self, msg: &str) -> io::Result<()> {
        self.write_log(msg, Level::DEBUG)
    }

    pub fn warning(&mut self, msg: &str) -> io::Result<()> {
        self.write_log(msg, Level::WARNING)
    }

    pub fn error(&mut self, msg: &str) -> io::Result<()> {
        self.write_log(msg, Level::ERROR)
    }
}

