use std::fmt;

/// Identify the three kinds of output supported.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum OutputKind<'f> {
    STDOUT,
    STDERR,
    FILE(&'f str),
}

/// Identify the level of the log.
#[derive(Eq, PartialEq, Hash)]
pub enum Level {
    INFO,
    DEBUG,
    WARNING,
    ERROR,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::INFO => write!(f, "INFO"),
            Level::DEBUG => write!(f, "DEBUG"),
            Level::WARNING => write!(f, "WARNING"),
            Level::ERROR => write!(f, "ERROR"),
        }
    }
}
