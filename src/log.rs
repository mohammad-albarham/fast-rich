//! Logging utilities similar to Rich's console.log().
//!
//! Provides timestamped logging with file/line information and pretty-printing.

use crate::console::{Console, RenderContext};
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
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
    /// Whether to show the timestamp
    pub show_time: bool,
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
    /// Rich's default style for this level ([`style_name`](Self::style_name)
    /// in the default theme): debug green, info blue, warning yellow,
    /// error bold red. Rendering uses the console theme instead.
    ///
    /// ```
    /// use fast_rich::log::LogLevel;
    /// assert!(LogLevel::Error.style().bold);
    /// ```
    pub fn style(&self) -> Style {
        crate::theme::builtin_style(self.style_name()).unwrap_or_default()
    }

    /// The theme style name for this level, like Rich's `RichHandler`:
    /// `logging.level.debug`, `logging.level.info`,
    /// `logging.level.warning` or `logging.level.error`.
    ///
    /// ```
    /// use fast_rich::log::LogLevel;
    /// assert_eq!(LogLevel::Warning.style_name(), "logging.level.warning");
    /// ```
    pub fn style_name(&self) -> &'static str {
        match self {
            LogLevel::Debug => "logging.level.debug",
            LogLevel::Info => "logging.level.info",
            LogLevel::Warning => "logging.level.warning",
            LogLevel::Error => "logging.level.error",
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
            show_time: true,
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

    /// Set whether to show the timestamp.
    pub fn show_time(mut self, show: bool) -> Self {
        self.show_time = show;
        self
    }

    /// Format the timestamp.
    fn format_time(&self) -> String {
        use std::time::UNIX_EPOCH;

        let duration = self.time.duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = duration.as_secs();

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
    /// Styles come from the theme, by Rich's names: `log.time`,
    /// `log.level` + `logging.level.<level>`, `log.message` and `log.path`.
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let mut spans = Vec::new();

        // Timestamp
        if self.show_time {
            spans.push(Span::styled(
                format!("[{}]", self.format_time()),
                context.style("log.time"),
            ));
            spans.push(Span::raw(" "));
        }

        // Level
        let level_style = context
            .style("log.level")
            .combine(&context.style(self.level.style_name()));
        spans.push(Span::styled(
            format!("{:5}", self.level.label()),
            level_style,
        ));

        spans.push(Span::raw(" "));

        // Message
        spans.push(Span::styled(
            self.message.clone(),
            context.style("log.message"),
        ));

        // Location
        if let Some(location) = self.format_location() {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(location, context.style("log.path")));
        }

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

#[cfg(feature = "logging")]
mod log_integration {
    //! Integration with the `log` crate.
    use super::*;
    use log::{Level, Log, Metadata, Record, SetLoggerError};
    use std::sync::OnceLock;

    static CONSOLE: OnceLock<Console> = OnceLock::new();

    /// Configuration for the RichLogger.
    #[derive(Clone, Debug)]
    pub struct RichLoggerConfig {
        /// Whether to show timestamps.
        pub enable_time: bool,
        /// Whether to show the file path/location.
        pub enable_path: bool,
    }

    impl Default for RichLoggerConfig {
        fn default() -> Self {
            Self {
                enable_time: true,
                enable_path: true,
            }
        }
    }

    /// A log handler that outputs to a rich Console.
    pub struct RichLogger {
        config: RichLoggerConfig,
    }

    impl RichLogger {
        /// Create a new builder for RichLogger.
        pub fn builder() -> RichLoggerBuilder {
            RichLoggerBuilder::default()
        }

        /// Initialize the logger with default settings.
        pub fn init() -> Result<(), SetLoggerError> {
            Self::builder().init()
        }
    }

    /// Builder for RichLogger.
    #[derive(Default)]
    pub struct RichLoggerBuilder {
        config: RichLoggerConfig,
        level: Option<log::LevelFilter>,
    }

    impl RichLoggerBuilder {
        /// Enable or disable timestamps.
        pub fn enable_time(mut self, enable: bool) -> Self {
            self.config.enable_time = enable;
            self
        }

        /// Enable or disable file paths.
        pub fn enable_path(mut self, enable: bool) -> Self {
            self.config.enable_path = enable;
            self
        }

        /// Set the max log level.
        pub fn filter_level(mut self, level: log::LevelFilter) -> Self {
            self.level = Some(level);
            self
        }

        /// Initialize the logger.
        pub fn init(self) -> Result<(), SetLoggerError> {
            // Initialize global console if not already
            CONSOLE.get_or_init(Console::new);

            let logger = Box::new(RichLogger {
                config: self.config,
            });

            // We need to leak the logger to satisfy 'static requirement of set_logger
            let static_logger = Box::leak(logger);

            log::set_logger(static_logger)?;
            log::set_max_level(self.level.unwrap_or(log::LevelFilter::Trace));
            Ok(())
        }
    }

    impl Log for RichLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            if !self.enabled(record.metadata()) {
                return;
            }

            let console = CONSOLE.get_or_init(Console::new);

            let level = match record.level() {
                Level::Error => LogLevel::Error,
                Level::Warn => LogLevel::Warning,
                Level::Info => LogLevel::Info,
                Level::Debug | Level::Trace => LogLevel::Debug,
            };

            let mut log_msg = LogMessage::new(&format!("{}", record.args()))
                .level(level)
                .show_time(self.config.enable_time);

            if self.config.enable_path {
                if let Some(file) = record.file_static() {
                    if let Some(line) = record.line() {
                        log_msg = log_msg.location(file, line);
                    }
                }
            }

            // Note: Timestamp is handled by LogMessage itself based on creation time,
            // but we could suppress it in render if we passed config down.
            // For now, let's just use what LogMessage does, but maybe we should refactor LogMessage
            // to just hold data and let the renderer decide?
            // Or simpler: We can't easily change LogMessage::render without changing trait signature
            // or adding fields.
            // Let's assume LogMessage::render always renders time if it has it,
            // but we want to control it.
            // Hack fix: If enable_time is false, we could modify how we construct LogMessage or
            // implementation of Renderable for LogMessage needs to know about config.
            // Since LogMessage is a public struct separate from RichLogger,
            // we should probably just make LogMessage configurable or specific to this usage.
            //
            // For this iteration, let's keep LogMessage implementation simple and maybe update it
            // to have public fields we can manipulate or rendering options.
            // But LogMessage implements Renderable directly.

            console.print_renderable(&log_msg);
        }

        fn flush(&self) {}
    }
}

#[cfg(feature = "logging")]
pub use log_integration::{RichLogger, RichLoggerBuilder};

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
        let context = RenderContext::new(80);
        let segments = msg.render(&context);

        assert_eq!(segments.len(), 1);
        let text = segments[0].plain_text();
        assert!(text.contains("INFO"));
        assert!(text.contains("Hello"));
    }
}
