//! Pretty traceback/panic rendering.
//!
//! Provides beautiful panic/error display similar to Rich's traceback feature.

use crate::console::RenderContext;
use crate::panel::{BorderStyle, Panel};
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::Text;
use std::panic::{self, PanicHookInfo};
use std::sync::Once;

/// Configuration for traceback display.
#[derive(Debug, Clone)]
pub struct TracebackConfig {
    /// Show source code context
    pub show_source: bool,
    /// Number of context lines before/after
    pub context_lines: usize,
    /// Show local variables (limited in Rust)
    pub show_locals: bool,
    /// Panel style
    pub border_style: BorderStyle,
    /// Error style
    pub error_style: Style,
}

impl Default for TracebackConfig {
    fn default() -> Self {
        TracebackConfig {
            show_source: true,
            context_lines: 3,
            show_locals: false,
            border_style: BorderStyle::Heavy,
            error_style: Style::new().foreground(Color::Red).bold(),
        }
    }
}

/// A formatted traceback.
pub struct Traceback {
    /// Error message
    message: String,
    /// Location information
    location: Option<String>,
    /// Configuration
    config: TracebackConfig,
}

impl Traceback {
    /// Create a new traceback from a panic info.
    pub fn from_panic(info: &PanicHookInfo<'_>) -> Self {
        let message = match info.payload().downcast_ref::<&str>() {
            Some(s) => s.to_string(),
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => s.clone(),
                None => "Unknown panic".to_string(),
            },
        };

        let location = info
            .location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));

        Traceback {
            message,
            location,
            config: TracebackConfig::default(),
        }
    }

    /// Create from an error message.
    pub fn from_error(message: &str) -> Self {
        Traceback {
            message: message.to_string(),
            location: None,
            config: TracebackConfig::default(),
        }
    }

    /// Set configuration.
    pub fn with_config(mut self, config: TracebackConfig) -> Self {
        self.config = config;
        self
    }

    fn build_content(&self) -> Text {
        let mut text = Text::new();

        // Error header
        text.push_styled("Error: ", Style::new().foreground(Color::Red).bold());
        text.push_styled(
            format!("{}\n", self.message),
            Style::new().foreground(Color::White),
        );

        // Location
        if let Some(ref loc) = self.location {
            text.push_styled("\nLocation: ", Style::new().foreground(Color::Cyan));
            text.push_styled(format!("{}\n", loc), Style::new().foreground(Color::Yellow));
        }

        // Attempt to read source code if available
        if self.config.show_source {
            if let Some(ref loc) = self.location {
                if let Some(source_context) = self.get_source_context(loc) {
                    text.push_styled("\nSource:\n", Style::new().foreground(Color::Cyan));
                    text.push(source_context);
                }
            }
        }

        text
    }

    fn get_source_context(&self, location: &str) -> Option<String> {
        // Parse location (file:line:column)
        let parts: Vec<&str> = location.split(':').collect();
        if parts.len() < 2 {
            return None;
        }

        let file_path = parts[0];
        let line_num: usize = parts[1].parse().ok()?;

        // Try to read the file
        let content = std::fs::read_to_string(file_path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        if line_num == 0 || line_num > lines.len() {
            return None;
        }

        let context = self.config.context_lines;
        let start = line_num.saturating_sub(context + 1);
        let end = (line_num + context).min(lines.len());

        let mut result = String::new();
        for (i, line) in lines.iter().enumerate().take(end).skip(start) {
            let line_number = i + 1;
            let prefix = if line_number == line_num {
                "→ "
            } else {
                "  "
            };
            result.push_str(&format!("{}{:4} │ {}\n", prefix, line_number, line));
        }

        Some(result)
    }
}

impl Renderable for Traceback {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let content = self.build_content();

        let panel = Panel::new(content)
            .title("Traceback")
            .border_style(self.config.border_style)
            .style(Style::new().foreground(Color::Red));

        panel.render(context)
    }
}

static PANIC_HOOK_INSTALLED: Once = Once::new();

/// Install a pretty panic hook.
///
/// This replaces the default panic hook with one that displays
/// nicely formatted tracebacks using rich formatting.
pub fn install_panic_hook() {
    PANIC_HOOK_INSTALLED.call_once(|| {
        let _default_hook = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            // Create and render traceback
            let traceback = Traceback::from_panic(info);
            let console = crate::Console::new();

            // Print a blank line first
            console.newline();
            console.print_renderable(&traceback);
            console.newline();

            // Note: We don't call the default hook here to avoid
            // double-printing. If you want to preserve it:
            // default_hook(info);
        }));
    });
}

/// Format an error for display.
pub fn format_error<E: std::error::Error>(error: &E) -> Traceback {
    let mut message = error.to_string();

    // Include source chain if available
    let mut source = error.source();
    while let Some(s) = source {
        message.push_str(&format!("\n  Caused by: {}", s));
        source = s.source();
    }

    Traceback::from_error(&message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traceback_from_error() {
        let tb = Traceback::from_error("Something went wrong");
        assert_eq!(tb.message, "Something went wrong");
        assert!(tb.location.is_none());
    }

    #[test]
    fn test_traceback_render() {
        let tb = Traceback::from_error("Test error");
        let context = RenderContext { width: 60 };
        let segments = tb.render(&context);

        // Should produce output
        assert!(!segments.is_empty());

        // Should contain error message
        let text: String = segments.iter().map(|s| s.plain_text()).collect();
        assert!(text.contains("Test error"));
    }

    #[test]
    fn test_traceback_config() {
        let config = TracebackConfig {
            show_source: false,
            context_lines: 5,
            ..Default::default()
        };

        let tb = Traceback::from_error("Test").with_config(config);
        assert!(!tb.config.show_source);
        assert_eq!(tb.config.context_lines, 5);
    }
}
