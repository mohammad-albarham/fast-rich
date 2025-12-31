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
use crate::text::{Span, Text};

use crossterm::{
    execute,
    style::{Attribute, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use std::io::{self, Write};

/// Escape HTML special characters.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Escape SVG special characters.
fn svg_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Rendering context passed to Renderable objects.
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Available width for rendering.
    pub width: usize,
    /// Available height for rendering (optional).
    pub height: Option<usize>,
}

impl Default for RenderContext {
    fn default() -> Self {
        RenderContext {
            width: 80,
            height: None,
        }
    }
}

/// Color system capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorSystem {
    /// No color support
    NoColor,
    /// Standard 8/16 colors
    #[default]
    Standard,
    /// 256 colors
    EightBit,
    /// True color (16 million colors)
    TrueColor,
    /// Windows legacy console (mapped to Standard for ANSI output)
    Windows,
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
    /// The detected or forced color system
    color_system: ColorSystem,
    /// Whether to use markup parsing
    markup: bool,
    /// Whether to translate emoji shortcodes
    emoji: bool,
    /// Soft wrap text at terminal width
    soft_wrap: bool,
    /// Whether recording is enabled
    record: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// Buffer for recorded segments
    recording: std::sync::Arc<std::sync::Mutex<Vec<Segment>>>,
}

#[derive(Debug, Clone)]
enum ConsoleOutput {
    Stdout,
    Stderr,
    Buffer(std::sync::Arc<std::sync::Mutex<Vec<u8>>>),
}

struct BufferWriter {
    buffer: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut lock = self
            .buffer
            .lock()
            .map_err(|e| io::Error::other(e.to_string()))?;
        lock.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    /// Create a new Console writing to stdout.
    pub fn new() -> Self {
        let (color_enabled, color_system) = Self::detect_color_system();
        Console {
            output: ConsoleOutput::Stdout,
            width: None,
            force_color: false,
            color_enabled,
            color_system,
            markup: true,
            emoji: true,
            soft_wrap: true,
            record: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            recording: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Create a new Console writing to stderr.
    pub fn stderr() -> Self {
        let (color_enabled, color_system) = Self::detect_color_system();
        Console {
            output: ConsoleOutput::Stderr,
            width: None,
            force_color: false,
            color_enabled,
            color_system,
            markup: true,
            emoji: true,
            soft_wrap: true,
            record: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            recording: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Create a new Console that captures output to memory.
    ///
    /// Useful for testing output verification.
    pub fn capture() -> Self {
        Console {
            output: ConsoleOutput::Buffer(std::sync::Arc::new(std::sync::Mutex::new(Vec::new()))),
            width: Some(80),   // Default width for tests
            force_color: true, // Force color for tests
            color_enabled: true,
            color_system: ColorSystem::TrueColor, // Capture assumes good capabilities
            markup: true,
            emoji: true,
            soft_wrap: true,
            record: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            recording: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Get the captured output as a string (if using capture mode).
    pub fn get_captured_output(&self) -> String {
        match &self.output {
            ConsoleOutput::Buffer(buf) => {
                let lock = buf.lock().unwrap();
                String::from_utf8(lock.clone()).unwrap_or_default()
            }
            _ => String::new(),
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
            // If forcing color and we were previously NoColor, assume Standard
            if self.color_system == ColorSystem::NoColor {
                self.color_system = ColorSystem::Standard;
            }
        }
        self
    }

    /// Set the color system explicitly.
    pub fn color_system(mut self, system: ColorSystem) -> Self {
        self.color_system = system;
        // If explicitly setting a color system (other than NoColor), enable color
        self.color_enabled = system != ColorSystem::NoColor;
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

    /// Enable or disable recording of output.
    pub fn record(self, enabled: bool) -> Self {
        self.record
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
        self
    }

    /// Start recording output.
    pub fn start_recording(&self) {
        self.record
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Ok(mut lock) = self.recording.lock() {
            lock.clear();
        }
    }

    /// Stop recording output.
    pub fn stop_recording(&self) {
        self.record
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get the current terminal width.
    pub fn get_width(&self) -> usize {
        self.width
            .unwrap_or_else(|| terminal::size().map(|(w, _)| w as usize).unwrap_or(80))
    }

    /// Detect color support and system.
    fn detect_color_system() -> (bool, ColorSystem) {
        // Check common environment variables
        if std::env::var("NO_COLOR").is_ok() {
            return (false, ColorSystem::NoColor);
        }

        if std::env::var("FORCE_COLOR").is_ok() {
            // Default to Standard if forced, can be upgraded by other checks if we were smarter,
            // but for now FORCE_COLOR just ensures we have *some* color.
            return (true, ColorSystem::Standard);
        }

        // Check COLORTERM for truecolor
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm.contains("truecolor") || colorterm.contains("24bit") {
                return (true, ColorSystem::TrueColor);
            }
        }

        // Check TERM for 256 colors
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") {
                return (true, ColorSystem::EightBit);
            }
        }

        // Fallback to Standard color if TTY (simplified)
        // In a real app we'd check is_tty
        (true, ColorSystem::Standard)
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
            height: None,
        };

        let segments = renderable.render(&context);
        self.write_segments(&segments);
    }

    /// Print a line (with newline at the end).
    pub fn println(&self, content: &str) {
        self.print(content);
        self.newline();
    }

    /// Print a string without markup parsing.
    ///
    /// Use this when printing content that may contain brackets `[...]`
    /// that should NOT be interpreted as markup (e.g., debug output).
    pub fn print_raw(&self, content: &str) {
        let text = Text::plain(content.to_string());
        self.print_renderable(&text);
    }

    /// Print a line without markup parsing (with newline at the end).
    ///
    /// Use this when printing content that may contain brackets `[...]`
    /// that should NOT be interpreted as markup (e.g., debug output).
    pub fn println_raw(&self, content: &str) {
        self.print_raw(content);
        self.newline();
    }

    /// Print an empty line.
    pub fn newline(&self) {
        let _ = self.write_raw("\n");
        // Record newline segment if recording
        if self.record.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(mut lock) = self.recording.lock() {
                lock.push(Segment::empty_line());
            }
        }
    }

    /// Write segments to the output.
    fn write_segments(&self, segments: &[Segment]) {
        if self.record.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(mut lock) = self.recording.lock() {
                lock.extend_from_slice(segments);
            }
        }

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
        if !self.color_enabled || self.color_system == ColorSystem::NoColor || span.style.is_empty()
        {
            let _ = self.write_raw(&span.text);
            return;
        }

        let mut writer = self.get_writer();

        // Helper to downsample colors based on system
        let process_color = |color: crate::style::Color| -> crossterm::style::Color {
            match self.color_system {
                ColorSystem::Standard | ColorSystem::Windows => color.to_standard().to_crossterm(),
                ColorSystem::EightBit => color.to_ansi256().to_crossterm(),
                ColorSystem::TrueColor => color.to_crossterm(),
                ColorSystem::NoColor => crossterm::style::Color::Reset, // Should be handled by early return
            }
        };

        // Set foreground color
        if let Some(color) = span.style.foreground {
            if matches!(
                self.color_system,
                ColorSystem::Standard | ColorSystem::Windows
            ) {
                let std_color = color.to_standard();
                let sgr = std_color.to_sgr_fg();
                if !sgr.is_empty() {
                    let _ = self.write_raw(&sgr);
                } else {
                    let _ = execute!(writer, SetForegroundColor(std_color.to_crossterm()));
                }
            } else {
                let _ = execute!(writer, SetForegroundColor(process_color(color)));
            }
        }

        // Set background color
        if let Some(color) = span.style.background {
            if matches!(
                self.color_system,
                ColorSystem::Standard | ColorSystem::Windows
            ) {
                let std_color = color.to_standard();
                let sgr = std_color.to_sgr_bg();
                if !sgr.is_empty() {
                    let _ = self.write_raw(&sgr);
                } else {
                    let _ = execute!(writer, SetBackgroundColor(std_color.to_crossterm()));
                }
            } else {
                let _ = execute!(writer, SetBackgroundColor(process_color(color)));
            }
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

        // Reset all attributes (SGR 0 includes color reset)
        let _ = execute!(writer, SetAttribute(Attribute::Reset));
    }

    /// Get the writer for this console.
    fn get_writer(&self) -> Box<dyn Write> {
        match &self.output {
            ConsoleOutput::Stdout => Box::new(io::stdout()),
            ConsoleOutput::Stderr => Box::new(io::stderr()),
            ConsoleOutput::Buffer(buf) => Box::new(BufferWriter {
                buffer: buf.clone(),
            }),
        }
    }

    /// Write raw string to output.
    fn write_raw(&self, s: &str) -> io::Result<()> {
        match &self.output {
            ConsoleOutput::Stdout => {
                let mut stdout = io::stdout();
                stdout.write_all(s.as_bytes())
            }
            ConsoleOutput::Stderr => {
                let mut stderr = io::stderr();
                stderr.write_all(s.as_bytes())
            }
            ConsoleOutput::Buffer(buf) => {
                let mut lock = buf.lock().map_err(|e| io::Error::other(e.to_string()))?;
                lock.extend_from_slice(s.as_bytes());
                Ok(())
            }
        }
    }

    /// Flush the output.
    fn flush(&self) -> io::Result<()> {
        match &self.output {
            ConsoleOutput::Stdout => io::stdout().flush(),
            ConsoleOutput::Stderr => io::stderr().flush(),
            ConsoleOutput::Buffer(_) => Ok(()),
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
        let _width = self.get_width();
        let rule = crate::rule::Rule::new(title);
        self.print_renderable(&rule);
        self.newline();
    }

    /// Print JSON with syntax highlighting.
    ///
    /// This method prints a JSON string with automatic syntax highlighting.
    /// The input should be a valid JSON string.
    #[cfg(feature = "syntax")]
    pub fn print_json(&self, json_str: &str) {
        let syntax = crate::syntax::Syntax::new(json_str, "json");
        self.print_renderable(&syntax);
        self.newline();
    }

    /// Pretty print a debug-printable object.
    ///
    /// Uses syntax highlighting if the `syntax` feature is enabled.
    pub fn print_debug<T: std::fmt::Debug>(&self, obj: &T) {
        let content = format!("{:#?}", obj);

        #[cfg(feature = "syntax")]
        {
            let syntax = crate::syntax::Syntax::new(&content, "rust");
            self.print_renderable(&syntax);
        }

        #[cfg(not(feature = "syntax"))]
        {
            // Use Text::plain to avoid parsing brackets as markup
            let text = Text::plain(content);
            self.print_renderable(&text);
        }

        self.newline();
    }

    /// Export a renderable as plain text.
    ///
    /// Returns the plain text representation without any ANSI codes.
    pub fn export_text(&self, renderable: &dyn Renderable) -> String {
        let context = RenderContext {
            width: self.get_width(),
            height: None,
        };
        let segments = renderable.render(&context);
        self.segments_to_text(&segments)
    }

    fn segments_to_text(&self, segments: &[Segment]) -> String {
        let mut result = String::new();
        for segment in segments {
            result.push_str(&segment.plain_text());
            if segment.newline {
                result.push('\n');
            }
        }
        result
    }

    /// Export a renderable as HTML with inline styles.
    ///
    /// Returns an HTML string with styled `<span>` elements.
    pub fn export_html(&self, renderable: &dyn Renderable) -> String {
        let context = RenderContext {
            width: self.get_width(),
            height: None,
        };
        let segments = renderable.render(&context);
        self.segments_to_html(&segments)
    }

    /// Save the recorded output as HTML.
    pub fn save_html(&self, path: &str) -> io::Result<()> {
        let segments = self.recording.lock().unwrap();
        let html = self.segments_to_html(&segments);
        std::fs::write(path, html)
    }

    fn segments_to_html(&self, segments: &[Segment]) -> String {
        let mut html = String::from("<pre style=\"font-family: monospace; background: #1e1e1e; color: #d4d4d4; padding: 1em;\">\n");

        for segment in segments {
            for span in &segment.spans {
                let style_css = span.style.to_css();
                if style_css.is_empty() {
                    html.push_str(&html_escape(&span.text));
                } else {
                    html.push_str(&format!(
                        "<span style=\"{}\">{}</span>",
                        style_css,
                        html_escape(&span.text)
                    ));
                }
            }
            if segment.newline {
                html.push('\n');
            }
        }

        html.push_str("</pre>");
        html
    }

    /// Export a renderable as SVG.
    ///
    /// Returns an SVG string with text elements.
    pub fn export_svg(&self, renderable: &dyn Renderable) -> String {
        let context = RenderContext {
            width: self.get_width(),
            height: None,
        };
        let segments = renderable.render(&context);
        self.segments_to_svg(&segments)
    }

    /// Save the recorded output as SVG.
    pub fn save_svg(&self, path: &str) -> io::Result<()> {
        let segments = self.recording.lock().unwrap();
        let svg = self.segments_to_svg(&segments);
        std::fs::write(path, svg)
    }

    fn segments_to_svg(&self, segments: &[Segment]) -> String {
        let char_width = 9.6; // Approximate monospace character width
        let line_height = 20.0;
        let padding = 10.0;

        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();

        for segment in segments {
            for span in &segment.spans {
                current_line.push_str(&span.text);
            }
            if segment.newline {
                lines.push(std::mem::take(&mut current_line));
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let max_chars = lines.iter().map(|l| l.len()).max().unwrap_or(80);
        let width = (max_chars as f64 * char_width) + padding * 2.0;
        let height = (lines.len() as f64 * line_height) + padding * 2.0;

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.0} {:.0}\">\n",
            width, height
        );
        svg.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#1e1e1e\"/>\n");
        svg.push_str("  <text font-family=\"monospace\" font-size=\"14\" fill=\"#d4d4d4\">\n");

        for (i, line) in lines.iter().enumerate() {
            let y = padding + (i as f64 + 1.0) * line_height;
            svg.push_str(&format!(
                "    <tspan x=\"{}\" y=\"{:.1}\">{}</tspan>\n",
                padding,
                y,
                svg_escape(line)
            ));
        }

        svg.push_str("  </text>\n</svg>");
        svg
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

    #[test]
    fn test_force_color() {
        let console = Console::new().force_color(true);
        assert!(console.force_color);
        assert!(console.color_enabled);
    }
}
