//! Spinner animations for indeterminate progress.

use crate::style::{Color, Style};
use crate::text::Span;
use std::time::Instant;

/// Spinner animation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerStyle {
    /// Dots animation (default)
    #[default]
    Dots,
    /// Line animation
    Line,
    /// Dots2
    Dots2,
    /// Arc animation
    Arc,
    /// Circle animation
    Circle,
    /// Square animation
    Square,
    /// Star animation
    Star,
    /// Bounce animation
    Bounce,
    /// Box animation
    Box,
    /// Simple animation
    Simple,
}

impl SpinnerStyle {
    /// Get the frames for this spinner style.
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["-", "\\", "|", "/"],
            SpinnerStyle::Dots2 => &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
            SpinnerStyle::Arc => &["◜", "◠", "◝", "◞", "◡", "◟"],
            SpinnerStyle::Circle => &["◐", "◓", "◑", "◒"],
            SpinnerStyle::Square => &["◰", "◳", "◲", "◱"],
            SpinnerStyle::Star => &["✶", "✸", "✹", "✺", "✹", "✷"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⠂"],
            SpinnerStyle::Box => &["▖", "▘", "▝", "▗"],
            SpinnerStyle::Simple => &["◴", "◷", "◶", "◵"],
        }
    }

    /// Get the interval between frames in milliseconds.
    pub fn interval_ms(&self) -> u64 {
        match self {
            SpinnerStyle::Dots | SpinnerStyle::Dots2 => 80,
            SpinnerStyle::Line => 130,
            SpinnerStyle::Arc | SpinnerStyle::Circle => 100,
            SpinnerStyle::Square => 120,
            SpinnerStyle::Star => 70,
            SpinnerStyle::Bounce => 120,
            SpinnerStyle::Box => 100,
            SpinnerStyle::Simple => 100,
        }
    }
}

/// A spinner for indeterminate progress.
#[derive(Debug, Clone)]
pub struct Spinner {
    /// Spinner style
    style: SpinnerStyle,
    /// Start time for animation
    start_time: Instant,
    /// Text to display after the spinner
    text: String,
    /// Style for the spinner character
    spinner_style: Style,
    /// Style for the text
    text_style: Style,
}

impl Spinner {
    /// Create a new spinner with optional text.
    pub fn new(text: &str) -> Self {
        Spinner {
            style: SpinnerStyle::Dots,
            start_time: Instant::now(),
            text: text.to_string(),
            spinner_style: Style::new().foreground(Color::Cyan),
            text_style: Style::new(),
        }
    }

    /// Set the spinner style.
    pub fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the spinner character style.
    pub fn spinner_style(mut self, style: Style) -> Self {
        self.spinner_style = style;
        self
    }

    /// Set the text style.
    pub fn text_style(mut self, style: Style) -> Self {
        self.text_style = style;
        self
    }

    /// Set the text.
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Update the text.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Get the text.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Get the spinner style.
    pub fn get_style(&self) -> SpinnerStyle {
        self.style
    }

    /// Get the current frame index.
    fn current_frame_index(&self) -> usize {
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;
        let interval = self.style.interval_ms();
        let frames = self.style.frames();
        ((elapsed_ms / interval) as usize) % frames.len()
    }

    /// Get the current frame character.
    pub fn current_frame(&self) -> &'static str {
        let frames = self.style.frames();
        let idx = self.current_frame_index();
        frames[idx]
    }

    /// Render the spinner to spans.
    pub fn render(&self) -> Vec<Span> {
        vec![
            Span::styled(self.current_frame().to_string(), self.spinner_style),
            Span::raw(" "),
            Span::styled(self.text.clone(), self.text_style),
        ]
    }

    /// Render to a string (for simple output).
    pub fn to_string_colored(&self) -> String {
        format!("{} {}", self.current_frame(), self.text)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Spinner::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_frames() {
        let style = SpinnerStyle::Dots;
        let frames = style.frames();
        assert!(!frames.is_empty());
        assert_eq!(frames[0], "⠋");
    }

    #[test]
    fn test_spinner_render() {
        let spinner = Spinner::new("Loading...");
        let spans = spinner.render();
        assert_eq!(spans.len(), 3);
    }

    #[test]
    fn test_all_spinner_styles() {
        let styles = [
            SpinnerStyle::Dots,
            SpinnerStyle::Line,
            SpinnerStyle::Dots2,
            SpinnerStyle::Arc,
            SpinnerStyle::Circle,
            SpinnerStyle::Square,
            SpinnerStyle::Star,
            SpinnerStyle::Bounce,
            SpinnerStyle::Box,
            SpinnerStyle::Simple,
        ];

        for style in styles {
            let frames = style.frames();
            assert!(!frames.is_empty(), "{:?} has no frames", style);
        }
    }
}
