use super::OutputKind;
use std::fs::OpenOptions;
use std::io::{self, Write};

/// Can log messages on its output and memorize the count of logger which rely on it.
pub struct WriteBuffer {
    count: usize,
    output: Box<dyn Write>,
}

impl WriteBuffer {
    /// Construct a new write buffer.
    ///
    /// # Arguments
    ///
    /// * `output` - the kind of output to be created.
    /// * `options` - a reference to an OpenOptions used to open the file if necessary.
    ///
    /// # Return Value
    ///
    /// The new write buffer if the function succeeded, the io error otherwise.
    pub fn new(output: &OutputKind, options: &OpenOptions) -> io::Result<Self> {
        Ok(WriteBuffer {
            count: 1,
            output: get_write_buffer(&output, options)?,
        })
    }

    /// Write a message on the struct output.
    ///
    /// # Arguments
    ///
    /// * `log_msg` - the message to write.
    pub fn log(&mut self, log_msg: String) -> io::Result<()> {
        self.output.write(log_msg.as_bytes())?;
        Ok(())
    }

    /// Increase the count of loggers which rely on this buffer by one.
    pub fn increase_count(&mut self) {
        self.count += 1;
    }

    /// Decrease the count of loggers which rely on this buffer by one.
    ///
    /// # Return Value
    ///
    /// `true` if the count has reached zero, indicating that the write buffer could be removed, `false` otherwise.
    pub fn decrease_count(&mut self) -> bool {
        if self.count > 1 {
            self.count -= 1;
            return false;
        } else {
            self.count = 0;
            return true;
        }
    }
}

/// Create an output to write on depending on the kind of output asked.
///
/// # Arguments
///
/// * `kind` - the kind of output to be created.
/// * `option` - a reference to an OpenOptions used to open the file if necessary.
///
/// # Return Value
///
/// The output that can be write on if the function succeeded a success, the io error otherwise.
fn get_write_buffer(kind: &OutputKind, option: &OpenOptions) -> io::Result<Box<dyn Write>> {
    Ok(match kind {
        OutputKind::STDOUT => Box::new(io::stdout()),
        OutputKind::STDERR => Box::new(io::stderr()),
        OutputKind::FILE(path) => Box::new(option.open(path)?),
    })
}
