mod timestamp;
mod logger;

pub use logger::{Logger, OutputKind};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    
    #[test]
    fn info() -> io::Result<()> {
        let mut log = Logger::new()?;
        log.info("this is an info.")
    }

    #[test]
    fn debug() -> io::Result<()> {
        let mut log = Logger::new()?;
        log.debug("this is a debug.")
    }

    #[test]
    fn error() -> io::Result<()> {
        let mut log = Logger::new()?;
        log.error("this is an error.")
    }

    #[test]
    fn warning() -> io::Result<()> {
        let mut log = Logger::new()?;
        log.warning("this is a warning.")
    }

    #[test]
    fn all_separate_output() -> io::Result<()> {
        let mut log = Logger::new()?;
        log.config_info(OutputKind::FILE("infolog.txt"))?;
        log.config_debug(OutputKind::FILE("debuglog.txt"))?;
        log.config_error(OutputKind::FILE("errorlog.txt"))?;
        log.config_warning(OutputKind::FILE("warninglog.txt"))?;
        log.info("this is an info.")?;
        log.debug("this is a debug.")?;
        log.error("this is an error.")?;
        log.warning("this is a warning.")?;
        Ok(())
    }

    #[test]
    fn all_same_output() -> io::Result<()> {
        let mut log = Logger::new()?;
        log.config_info(OutputKind::FILE("log.txt"))?;
        log.config_debug(OutputKind::FILE("log.txt"))?;
        log.config_error(OutputKind::FILE("log.txt"))?;
        log.config_warning(OutputKind::FILE("log.txt"))?;
        log.info("this is an info.")?;
        log.debug("this is a debug.")?;
        log.error("this is an error.")?;
        log.warning("this is a warning.")?;
        Ok(())
    }
    
}