//! Logging utilities similar to Rich's console.log().
//!
//! Provides timestamped logging with file/line information and pretty-printing.

use crate::console::{Console, RenderContext};
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::Span;
use std::time::SystemTime;

/// A log message with metadata.
#[derive(Debug)]
pub struct LogMessage {
    /// The message content
    pub message: String,
    /// File where the log was called
    pub file: Option<&'static str>,
    /// Line number
    pub line: Option<u32>,
    /// Timestamp
    pub time: SystemTime,
    /// Log level
    pub level: LogLevel,
}

/// Log level for messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    /// Debug level
    Debug,
    /// Info level (default)
    #[default]
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
}

impl LogLevel {
    /// Get the style for this log level.
    pub fn style(&self) -> Style {
        match self {
            LogLevel::Debug => Style::new().foreground(Color::BrightBlack),
            LogLevel::Info => Style::new().foreground(Color::Blue),
            LogLevel::Warning => Style::new().foreground(Color::Yellow),
            LogLevel::Error => Style::new().foreground(Color::Red).bold(),
        }
    }

    /// Get the label for this log level.
    pub fn label(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

impl LogMessage {
    /// Create a new log message.
    pub fn new(message: &str) -> Self {
        LogMessage {
            message: message.to_string(),
            file: None,
            line: None,
            time: SystemTime::now(),
            level: LogLevel::Info,
        }
    }

    /// Set the file and line.
    pub fn location(mut self, file: &'static str, line: u32) -> Self {
        self.file = Some(file);
        self.line = Some(line);
        self
    }

    /// Set the log level.
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Format the timestamp.
    fn format_time(&self) -> String {
        use std::time::UNIX_EPOCH;

        let duration = self.time.duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = duration.as_secs();

        // Simple time formatting (HH:MM:SS)
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        let millis = duration.subsec_millis();

        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    }

    /// Format the location.
    fn format_location(&self) -> Option<String> {
        match (self.file, self.line) {
            (Some(file), Some(line)) => {
                // Get just the filename
                let filename = file.rsplit('/').next().unwrap_or(file);
                Some(format!("{}:{}", filename, line))
            }
            _ => None,
        }
    }
}

impl Renderable for LogMessage {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        let mut spans = Vec::new();

        // Timestamp
        spans.push(Span::styled(
            format!("[{}]", self.format_time()),
            Style::new().foreground(Color::BrightBlack),
        ));

        spans.push(Span::raw(" "));

        // Level
        spans.push(Span::styled(
            format!("{:5}", self.level.label()),
            self.level.style(),
        ));

        spans.push(Span::raw(" "));

        // Location
        if let Some(location) = self.format_location() {
            spans.push(Span::styled(
                format!("[{}]", location),
                Style::new().foreground(Color::Cyan),
            ));
            spans.push(Span::raw(" "));
        }

        // Message
        spans.push(Span::raw(self.message.clone()));

        vec![Segment::line(spans)]
    }
}

/// Extension trait for Console to add logging methods.
pub trait ConsoleLog {
    /// Log a message with timestamp and location.
    fn log(&self, message: &str);

    /// Log a debug message.
    fn debug(&self, message: &str);

    /// Log a warning message.
    fn warn(&self, message: &str);

    /// Log an error message.
    fn error(&self, message: &str);
}

impl ConsoleLog for Console {
    fn log(&self, message: &str) {
        let log_msg = LogMessage::new(message);
        self.print_renderable(&log_msg);
    }

    fn debug(&self, message: &str) {
        let log_msg = LogMessage::new(message).level(LogLevel::Debug);
        self.print_renderable(&log_msg);
    }

    fn warn(&self, message: &str) {
        let log_msg = LogMessage::new(message).level(LogLevel::Warning);
        self.print_renderable(&log_msg);
    }

    fn error(&self, message: &str) {
        let log_msg = LogMessage::new(message).level(LogLevel::Error);
        self.print_renderable(&log_msg);
    }
}

/// Macro for logging with file/line information.
#[macro_export]
macro_rules! log {
    ($console:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let log_msg = $crate::log::LogMessage::new(&message)
            .location(file!(), line!());
        $console.print_renderable(&log_msg);
    }};
}

/// Macro for debug logging.
#[macro_export]
macro_rules! log_debug {
    ($console:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let log_msg = $crate::log::LogMessage::new(&message)
            .location(file!(), line!())
            .level($crate::log::LogLevel::Debug);
        $console.print_renderable(&log_msg);
    }};
}

/// Macro for warning logging.
#[macro_export]
macro_rules! log_warn {
    ($console:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let log_msg = $crate::log::LogMessage::new(&message)
            .location(file!(), line!())
            .level($crate::log::LogLevel::Warning);
        $console.print_renderable(&log_msg);
    }};
}

/// Macro for error logging.
#[macro_export]
macro_rules! log_error {
    ($console:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        let log_msg = $crate::log::LogMessage::new(&message)
            .location(file!(), line!())
            .level($crate::log::LogLevel::Error);
        $console.print_renderable(&log_msg);
    }};
}

#[cfg(feature = "logging")]
mod log_integration {
    //! Integration with the `log` crate.

    use super::*;
    use log::{Level, Log, Metadata, Record};
    use std::sync::OnceLock;

    static CONSOLE: OnceLock<Console> = OnceLock::new();

    /// A log handler that outputs to a rich Console.
    pub struct RichLogger;

    impl Log for RichLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            let console = CONSOLE.get_or_init(Console::new);

            let level = match record.level() {
                Level::Error => LogLevel::Error,
                Level::Warn => LogLevel::Warning,
                Level::Info => LogLevel::Info,
                Level::Debug | Level::Trace => LogLevel::Debug,
            };

            let mut log_msg = LogMessage::new(&format!("{}", record.args())).level(level);

            if let Some(file) = record.file_static() {
                if let Some(line) = record.line() {
                    log_msg = log_msg.location(file, line);
                }
            }

            console.print_renderable(&log_msg);
        }

        fn flush(&self) {}
    }

    /// Install the rich logger as the global logger.
    pub fn install() -> Result<(), log::SetLoggerError> {
        static LOGGER: RichLogger = RichLogger;
        log::set_logger(&LOGGER)?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(())
    }
}

#[cfg(feature = "logging")]
pub use log_integration::{install as install_logger, RichLogger};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_message_format_time() {
        let msg = LogMessage::new("test");
        let time = msg.format_time();
        // Should be in HH:MM:SS.mmm format
        assert!(time.contains(':'));
        assert!(time.contains('.'));
    }

    #[test]
    fn test_log_message_render() {
        let msg = LogMessage::new("Hello").level(LogLevel::Info);
        let context = RenderContext { width: 80 };
        let segments = msg.render(&context);

        assert_eq!(segments.len(), 1);
        let text = segments[0].plain_text();
        assert!(text.contains("INFO"));
        assert!(text.contains("Hello"));
    }

    #[test]
    fn test_log_levels() {
        assert_eq!(LogLevel::Debug.label(), "DEBUG");
        assert_eq!(LogLevel::Info.label(), "INFO");
        assert_eq!(LogLevel::Warning.label(), "WARN");
        assert_eq!(LogLevel::Error.label(), "ERROR");
    }
}
