//! Console abstraction for terminal output.
//!
//! The `Console` type is the main entry point for rich terminal output.
//! It handles styled printing, word wrapping, and terminal capabilities.
//!
//! # Examples
//!
//! ```no_run
//! use rich_rust::Console;
//!
//! let console = Console::new();
//! console.print("Hello, [bold magenta]World[/]!");
//! ```

use crate::markup;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Span, Text};

use crossterm::{
    execute,
    style::{Attribute, Color as CrosstermColor, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io::{self, Write};

/// Rendering context passed to Renderable objects.
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Available width for rendering.
    pub width: usize,
}

impl Default for RenderContext {
    fn default() -> Self {
        RenderContext { width: 80 }
    }
}

/// The main console type for rich terminal output.
#[derive(Debug)]
pub struct Console {
    /// Output stream (stdout or stderr)
    output: ConsoleOutput,
    /// Terminal width (cached or forced)
    width: Option<usize>,
    /// Whether to force color output
    force_color: bool,
    /// Whether color is enabled
    color_enabled: bool,
    /// Whether to use markup parsing
    markup: bool,
    /// Whether to translate emoji shortcodes
    emoji: bool,
    /// Soft wrap text at terminal width
    soft_wrap: bool,
}

#[derive(Debug)]
enum ConsoleOutput {
    Stdout,
    Stderr,
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    /// Create a new Console writing to stdout.
    pub fn new() -> Self {
        Console {
            output: ConsoleOutput::Stdout,
            width: None,
            force_color: false,
            color_enabled: Self::detect_color_support(),
            markup: true,
            emoji: true,
            soft_wrap: true,
        }
    }

    /// Create a new Console writing to stderr.
    pub fn stderr() -> Self {
        Console {
            output: ConsoleOutput::Stderr,
            ..Self::new()
        }
    }

    /// Set a fixed terminal width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Force color output even when not detected.
    pub fn force_color(mut self, force: bool) -> Self {
        self.force_color = force;
        if force {
            self.color_enabled = true;
        }
        self
    }

    /// Enable or disable markup parsing.
    pub fn markup(mut self, enabled: bool) -> Self {
        self.markup = enabled;
        self
    }

    /// Enable or disable emoji shortcode translation.
    pub fn emoji(mut self, enabled: bool) -> Self {
        self.emoji = enabled;
        self
    }

    /// Enable or disable soft word wrapping.
    pub fn soft_wrap(mut self, enabled: bool) -> Self {
        self.soft_wrap = enabled;
        self
    }

    /// Get the current terminal width.
    pub fn get_width(&self) -> usize {
        self.width.unwrap_or_else(|| {
            terminal::size()
                .map(|(w, _)| w as usize)
                .unwrap_or(80)
        })
    }

    /// Detect if color output is supported.
    fn detect_color_support() -> bool {
        // Check common environment variables
        if std::env::var("NO_COLOR").is_ok() {
            return false;
        }

        if std::env::var("FORCE_COLOR").is_ok() {
            return true;
        }

        // Check if output is a TTY
        // For simplicity, assume color is supported
        // A more complete implementation would use atty crate
        true
    }

    /// Print a string with markup support.
    pub fn print(&self, content: &str) {
        let text = if self.markup {
            markup::parse(content)
        } else {
            Text::plain(content.to_string())
        };

        self.print_renderable(&text);
    }

    /// Print any renderable object.
    pub fn print_renderable(&self, renderable: &dyn Renderable) {
        let context = RenderContext {
            width: self.get_width(),
        };

        let segments = renderable.render(&context);
        self.write_segments(&segments);
    }

    /// Print a line (with newline at the end).
    pub fn println(&self, content: &str) {
        self.print(content);
        self.newline();
    }

    /// Print an empty line.
    pub fn newline(&self) {
        let _ = self.write_raw("\n");
    }

    /// Write segments to the output.
    fn write_segments(&self, segments: &[Segment]) {
        for segment in segments {
            for span in &segment.spans {
                self.write_span(span);
            }
            if segment.newline {
                let _ = self.write_raw("\n");
            }
        }
        let _ = self.flush();
    }

    /// Write a single span with styling.
    fn write_span(&self, span: &Span) {
        if !self.color_enabled || span.style.is_empty() {
            let _ = self.write_raw(&span.text);
            return;
        }

        let mut writer = self.get_writer();

        // Set foreground color
        if let Some(color) = span.style.foreground {
            let _ = execute!(writer, SetForegroundColor(color.to_crossterm()));
        }

        // Set background color
        if let Some(color) = span.style.background {
            let _ = execute!(writer, SetBackgroundColor(color.to_crossterm()));
        }

        // Set attributes
        if span.style.bold {
            let _ = execute!(writer, SetAttribute(Attribute::Bold));
        }
        if span.style.dim {
            let _ = execute!(writer, SetAttribute(Attribute::Dim));
        }
        if span.style.italic {
            let _ = execute!(writer, SetAttribute(Attribute::Italic));
        }
        if span.style.underline {
            let _ = execute!(writer, SetAttribute(Attribute::Underlined));
        }
        if span.style.blink {
            let _ = execute!(writer, SetAttribute(Attribute::SlowBlink));
        }
        if span.style.reverse {
            let _ = execute!(writer, SetAttribute(Attribute::Reverse));
        }
        if span.style.hidden {
            let _ = execute!(writer, SetAttribute(Attribute::Hidden));
        }
        if span.style.strikethrough {
            let _ = execute!(writer, SetAttribute(Attribute::CrossedOut));
        }

        // Write the text
        let _ = execute!(writer, Print(&span.text));

        // Reset
        let _ = execute!(writer, ResetColor);
        let _ = execute!(writer, SetAttribute(Attribute::Reset));
    }

    /// Get the writer for this console.
    fn get_writer(&self) -> Box<dyn Write> {
        match self.output {
            ConsoleOutput::Stdout => Box::new(io::stdout()),
            ConsoleOutput::Stderr => Box::new(io::stderr()),
        }
    }

    /// Write raw string to output.
    fn write_raw(&self, s: &str) -> io::Result<()> {
        match self.output {
            ConsoleOutput::Stdout => {
                let mut stdout = io::stdout();
                stdout.write_all(s.as_bytes())
            }
            ConsoleOutput::Stderr => {
                let mut stderr = io::stderr();
                stderr.write_all(s.as_bytes())
            }
        }
    }

    /// Flush the output.
    fn flush(&self) -> io::Result<()> {
        match self.output {
            ConsoleOutput::Stdout => io::stdout().flush(),
            ConsoleOutput::Stderr => io::stderr().flush(),
        }
    }

    /// Clear the screen.
    pub fn clear(&self) {
        let mut writer = self.get_writer();
        let _ = execute!(
            writer,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
    }

    /// Show a rule (horizontal line).
    pub fn rule(&self, title: &str) {
        let width = self.get_width();
        let rule = crate::rule::Rule::new(title);
        self.print_renderable(&rule);
        self.newline();
    }
}

/// A guard that captures output for testing.
#[derive(Debug)]
pub struct CapturedOutput {
    segments: Vec<Segment>,
}

impl CapturedOutput {
    /// Create a new capture.
    pub fn new() -> Self {
        CapturedOutput {
            segments: Vec::new(),
        }
    }

    /// Get the plain text output.
    pub fn plain_text(&self) -> String {
        let mut result = String::new();
        for segment in &self.segments {
            result.push_str(&segment.plain_text());
            if segment.newline {
                result.push('\n');
            }
        }
        result
    }
}

impl Default for CapturedOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_default_width() {
        let console = Console::new().width(80);
        assert_eq!(console.get_width(), 80);
    }

    #[test]
    fn test_render_context_default() {
        let context = RenderContext::default();
        assert_eq!(context.width, 80);
    }
}
