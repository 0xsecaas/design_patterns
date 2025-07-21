use std::sync::LazyLock;

/// A simple singleton logger implementation using `LazyLock`.
///
/// The `Logger` struct provides a basic logging interface with a `log` method
/// that prints messages with a specified log level. The singleton instance
/// `LOGGER` is lazily initialized and can be accessed globally.
///
/// # Example
///
/// ```rust
/// LOGGER.log("Application started", LogLevel::Info);
/// ```
struct Logger {}

pub enum LogLevel {
    Info,
    Error,
    Warn,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            LogLevel::Info => "INFO",
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        };
        write!(f, "{}", level_str)
    }
}

impl Logger {
    /// Logs a message with a specified log level.
    ///
    /// # Parameters
    /// - `msg`: The message to be logged.
    /// - `level`: The log level (`LogLevel` enum, e.g., `LogLevel::Info`, `LogLevel::Error`) to associate with the message.
    fn log(&self, msg: &str, level: LogLevel) {
        println!("{}:{}", msg, level);
    }
}

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger {});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_does_not_panic() {
        LOGGER.log("Hello There", LogLevel::Info);
    }
}
