mod timestamp;
mod logger;

pub use logger::Logger;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn info() -> io::Result<()> {
        let mut log = Logger::new();
        log.info("this is an info.")
    }

    #[test]
    fn debug() -> io::Result<()> {
        let mut log = Logger::new();
        log.debug("this is a debug.")
    }

    #[test]
    fn error() -> io::Result<()> {
        let mut log = Logger::new();
        log.error("this is an error.")
    }

    #[test]
    fn warning() -> io::Result<()> {
        let mut log = Logger::new();
        log.warning("this is a warning.")
    }
}