mod write_buffer;
mod output_kind;

pub use output_kind::OutputKind;
pub use write_buffer::WriteBuffer;

use crate::timestamp::get_timestamp;
use std::io;
use std::fs::OpenOptions;
use std::collections::HashMap;

pub struct Logger<'o> {
    write_buffers: HashMap<OutputKind<'o>, WriteBuffer>,
    info_output: OutputKind<'o>,
    debug_output: OutputKind<'o>,
    warning_output: OutputKind<'o>,
    error_output: OutputKind<'o>,
    file_options: OpenOptions,
}

impl<'o> Logger<'o> {
    pub fn new() -> io::Result<Self> {
        let mut options = OpenOptions::new();
        options.write(true).create(true);

        let mut stdout = WriteBuffer::new(&OutputKind::STDOUT, &options)?;
        let mut stderr = WriteBuffer::new(&OutputKind::STDERR, &options)?;
        stdout.increase_count();
        stderr.increase_count();

        let mut map = HashMap::new();
        map.insert(OutputKind::STDOUT, stdout);
        map.insert(OutputKind::STDERR, stderr);

        let log = Logger {
            write_buffers: map,
            info_output: OutputKind::STDOUT,
            debug_output: OutputKind::STDOUT,
            warning_output: OutputKind::STDERR,
            error_output: OutputKind::STDERR,
            file_options: options,
        };

        return Ok(log);
    }

    pub fn set_output_info(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        match self.write_buffers.get_mut(&kind) {
            None => {
                let buffer = WriteBuffer::new(&kind, &self.file_options)?;
                self.write_buffers.insert(kind.clone(), buffer);
            },
            Some(buffer) => buffer.increase_count()
        };
        if match self.write_buffers.get_mut(&self.info_output) {
            None => false,
            Some(buffer) => buffer.decrease_count()
        } {
            self.write_buffers.remove(&self.info_output);
        }
        self.info_output = kind;
        Ok(())
    }

    pub fn set_output_debug(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        match self.write_buffers.get_mut(&kind) {
            None => {
                let buffer = WriteBuffer::new(&kind, &self.file_options)?;
                self.write_buffers.insert(kind.clone(), buffer);
            },
            Some(buffer) => buffer.increase_count()
        };
        if match self.write_buffers.get_mut(&self.debug_output) {
            None => false,
            Some(buffer) => buffer.decrease_count()
        } {
            self.write_buffers.remove(&self.debug_output);
        }
        self.debug_output = kind;
        Ok(())
    }

    pub fn set_output_warning(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        match self.write_buffers.get_mut(&kind) {
            None => {
                let buffer = WriteBuffer::new(&kind, &self.file_options)?;
                self.write_buffers.insert(kind.clone(), buffer);
            },
            Some(buffer) => buffer.increase_count()
        };
        if match self.write_buffers.get_mut(&self.warning_output) {
            None => false,
            Some(buffer) => buffer.decrease_count()
        } {
            self.write_buffers.remove(&self.warning_output);
        }
        self.warning_output = kind;
        Ok(())
    }

    pub fn set_output_error(&mut self, kind: OutputKind<'o>) -> io::Result<()> {
        match self.write_buffers.get_mut(&kind) {
            None => {
                let buffer = WriteBuffer::new(&kind, &self.file_options)?;
                self.write_buffers.insert(kind.clone(), buffer);
            },
            Some(buffer) => buffer.increase_count()
        };
        if match self.write_buffers.get_mut(&self.error_output) {
            None => false,
            Some(buffer) => buffer.decrease_count()
        } {
            self.write_buffers.remove(&self.error_output);
        }
        self.error_output = kind;
        Ok(())
    }

    pub fn info(&mut self, msg: &str) -> io::Result<()> {
        if let Some(buffer) = self.write_buffers.get_mut(&self.info_output) {
            buffer.log(format!("[INFO-{}]: {}\n", get_timestamp(), msg))?;
        };
        Ok(())
    }

    pub fn debug(&mut self, msg: &str) -> io::Result<()> {
        if let Some(buffer) = self.write_buffers.get_mut(&self.debug_output) {
            buffer.log(format!("[DEBUG-{}]: {}\n", get_timestamp(), msg))?;
        };
        Ok(())
    }

    pub fn warning(&mut self, msg: &str) -> io::Result<()> {
        if let Some(buffer) = self.write_buffers.get_mut(&self.warning_output) {
            buffer.log(format!("[WARNING-{}]: {}\n", get_timestamp(), msg))?;
        };
        Ok(())
    }

    pub fn error(&mut self, msg: &str) -> io::Result<()> {
        if let Some(buffer) = self.write_buffers.get_mut(&self.error_output) {
            buffer.log(format!("[ERROR-{}]: {}\n", get_timestamp(), msg))?;
        };
        Ok(())
    }
}

