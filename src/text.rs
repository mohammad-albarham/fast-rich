//! Text and Span types for styled text with wrapping and alignment.
//!
//! This module provides `Text` - a container for styled text that supports
//! word wrapping, alignment, and combining multiple styled spans.

use crate::style::Style;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A styled region of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    /// The text content
    pub text: Cow<'static, str>,
    /// The style applied to this span
    pub style: Style,
}

impl Span {
    /// Create a new span with no style.
    pub fn raw<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Span {
            text: text.into(),
            style: Style::new(),
        }
    }

    /// Create a new span with a style.
    pub fn styled<S: Into<Cow<'static, str>>>(text: S, style: Style) -> Self {
        Span {
            text: text.into(),
            style,
        }
    }

    /// Get the display width of this span.
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self.text.as_ref())
    }

    /// Check if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Span {
    fn from(text: S) -> Self {
        Span::raw(text)
    }
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Left-aligned (default)
    #[default]
    Left,
    /// Center-aligned
    Center,
    /// Right-aligned
    Right,
}

/// Overflow behavior when text exceeds available width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Wrap to next line (default)
    #[default]
    Wrap,
    /// Truncate with ellipsis
    Ellipsis,
    /// Hard truncate without indicator
    Truncate,
    /// Allow overflow
    Visible,
}

/// A text container with multiple styled spans.
#[derive(Debug, Clone, Default)]
pub struct Text {
    /// The spans that make up this text
    pub spans: Vec<Span>,
    /// Text alignment
    pub alignment: Alignment,
    /// Overflow behavior
    pub overflow: Overflow,
    /// Optional style applied to the whole text
    pub style: Style,
}

impl Text {
    /// Create a new empty text.
    pub fn new() -> Self {
        Text::default()
    }

    /// Create text from a plain string.
    pub fn plain<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Text {
            spans: vec![Span::raw(text)],
            ..Default::default()
        }
    }

    /// Create text from a styled string.
    pub fn styled<S: Into<Cow<'static, str>>>(text: S, style: Style) -> Self {
        Text {
            spans: vec![Span::styled(text, style)],
            style,
            ..Default::default()
        }
    }

    /// Create text from multiple spans.
    pub fn from_spans<I: IntoIterator<Item = Span>>(spans: I) -> Self {
        Text {
            spans: spans.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Add a span to the text.
    pub fn push_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Append plain text.
    pub fn push<S: Into<Cow<'static, str>>>(&mut self, text: S) {
        self.spans.push(Span::raw(text));
    }

    /// Append styled text.
    pub fn push_styled<S: Into<Cow<'static, str>>>(&mut self, text: S, style: Style) {
        self.spans.push(Span::styled(text, style));
    }

    /// Set the alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the overflow behavior.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Set the overall style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get the total display width (without line breaks).
    pub fn width(&self) -> usize {
        self.spans.iter().map(|s| s.width()).sum()
    }

    /// Get the plain text content without styling.
    pub fn plain_text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_ref()).collect()
    }

    /// Check if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|s| s.is_empty())
    }

    /// Split the text into lines, wrapping at the given width.
    pub fn wrap(&self, width: usize) -> Vec<Vec<Span>> {
        if width == 0 {
            return vec![];
        }

        match self.overflow {
            Overflow::Visible => vec![self.spans.clone()],
            Overflow::Truncate | Overflow::Ellipsis => {
                vec![self.truncate_spans(width, self.overflow == Overflow::Ellipsis)]
            }
            Overflow::Wrap => self.wrap_spans(width),
        }
    }

    fn truncate_spans(&self, width: usize, ellipsis: bool) -> Vec<Span> {
        let mut result = Vec::new();
        let mut remaining_width = if ellipsis {
            width.saturating_sub(1)
        } else {
            width
        };

        for span in &self.spans {
            if remaining_width == 0 {
                break;
            }

            let span_width = span.width();
            if span_width <= remaining_width {
                result.push(span.clone());
                remaining_width -= span_width;
            } else {
                // Truncate this span
                let truncated = truncate_str(&span.text, remaining_width);
                result.push(Span::styled(truncated.to_string(), span.style));
                remaining_width = 0;
            }
        }

        if ellipsis && self.width() > width {
            result.push(Span::raw("…"));
        }

        result
    }

    fn wrap_spans(&self, max_width: usize) -> Vec<Vec<Span>> {
        let mut lines: Vec<Vec<Span>> = Vec::new();
        let mut current_line: Vec<Span> = Vec::new();
        let mut current_width = 0;

        for span in &self.spans {
            let words = split_into_words(&span.text);

            for (word, trailing_space) in words {
                let word_width = UnicodeWidthStr::width(word);
                let space_width = if trailing_space { 1 } else { 0 };
                let total_width = word_width + space_width;

                // If word fits on current line
                if current_width + word_width <= max_width {
                    let text = if trailing_space {
                        format!("{word} ")
                    } else {
                        word.to_string()
                    };
                    current_line.push(Span::styled(text, span.style));
                    current_width += total_width;
                } else if word_width > max_width {
                    // Word is too long, need to break it
                    if !current_line.is_empty() {
                        lines.push(std::mem::take(&mut current_line));
                        current_width = 0;
                    }

                    // Break the word across lines
                    let broken = break_word(word, max_width);
                    for (i, part) in broken.iter().enumerate() {
                        if i > 0 {
                            lines.push(std::mem::take(&mut current_line));
                        }
                        current_line.push(Span::styled(part.to_string(), span.style));
                        current_width = UnicodeWidthStr::width(part.as_str());
                    }

                    if trailing_space && current_width < max_width {
                        current_line.push(Span::styled(" ", span.style));
                        current_width += 1;
                    }
                } else {
                    // Start new line
                    if !current_line.is_empty() {
                        lines.push(std::mem::take(&mut current_line));
                    }
                    let text = if trailing_space {
                        format!("{word} ")
                    } else {
                        word.to_string()
                    };
                    current_line.push(Span::styled(text, span.style));
                    current_width = total_width;
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(Vec::new());
        }

        lines
    }

    /// Apply alignment to a line, returning padded spans.
    pub fn align_line(&self, line: Vec<Span>, width: usize) -> Vec<Span> {
        let line_width: usize = line.iter().map(|s| s.width()).sum();

        if line_width >= width {
            return line;
        }

        let padding = width - line_width;

        match self.alignment {
            Alignment::Left => {
                let mut result = line;
                result.push(Span::raw(" ".repeat(padding)));
                result
            }
            Alignment::Right => {
                let mut result = vec![Span::raw(" ".repeat(padding))];
                result.extend(line);
                result
            }
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                let mut result = vec![Span::raw(" ".repeat(left_pad))];
                result.extend(line);
                result.push(Span::raw(" ".repeat(right_pad)));
                result
            }
        }
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Text {
    fn from(text: S) -> Self {
        Text::plain(text)
    }
}

/// Truncate a string to a given display width.
fn truncate_str(s: &str, max_width: usize) -> &str {
    let mut width = 0;
    let mut end = 0;

    for grapheme in s.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if width + grapheme_width > max_width {
            break;
        }
        width += grapheme_width;
        end += grapheme.len();
    }

    &s[..end]
}

/// Split text into words, preserving trailing spaces.
fn split_into_words(s: &str) -> Vec<(&str, bool)> {
    let mut words = Vec::new();
    let mut word_start = None;

    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            if let Some(start) = word_start {
                words.push((&s[start..i], true));
                word_start = None;
            }
        } else if word_start.is_none() {
            word_start = Some(i);
        }
    }

    if let Some(start) = word_start {
        words.push((&s[start..], false));
    }

    words
}

/// Break a long word into parts that fit within max_width.
fn break_word(word: &str, max_width: usize) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;

    for grapheme in word.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);

        if current_width + grapheme_width > max_width && !current.is_empty() {
            parts.push(std::mem::take(&mut current));
            current_width = 0;
        }

        current.push_str(grapheme);
        current_width += grapheme_width;
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_width() {
        assert_eq!(Span::raw("hello").width(), 5);
        assert_eq!(Span::raw("你好").width(), 4); // Chinese characters are double-width
        assert_eq!(Span::raw("").width(), 0);
    }

    #[test]
    fn test_text_plain() {
        let text = Text::plain("Hello, World!");
        assert_eq!(text.plain_text(), "Hello, World!");
        assert_eq!(text.width(), 13);
    }

    #[test]
    fn test_text_wrap_simple() {
        let text = Text::plain("hello world");
        let lines = text.wrap(6);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0][0].text, "hello ");
        assert_eq!(lines[1][0].text, "world");
    }

    #[test]
    fn test_text_wrap_long_word() {
        let text = Text::plain("supercalifragilistic");
        let lines = text.wrap(10);
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_truncate_ellipsis() {
        let text = Text::plain("Hello, World!").overflow(Overflow::Ellipsis);
        let lines = text.wrap(8);
        let plain: String = lines[0].iter().map(|s| s.text.as_ref()).collect();
        assert!(plain.ends_with('…'));
        // Check display width, not byte length (ellipsis is 3 bytes but 1 char width)
        assert!(UnicodeWidthStr::width(plain.as_str()) <= 8);
    }

    #[test]
    fn test_alignment_left() {
        let text = Text::plain("hi").alignment(Alignment::Left);
        let lines = text.wrap(10);
        let aligned = text.align_line(lines[0].clone(), 10);
        let plain: String = aligned.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(plain, "hi        ");
    }

    #[test]
    fn test_alignment_right() {
        let text = Text::plain("hi").alignment(Alignment::Right);
        let lines = text.wrap(10);
        let aligned = text.align_line(lines[0].clone(), 10);
        let plain: String = aligned.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(plain, "        hi");
    }

    #[test]
    fn test_alignment_center() {
        let text = Text::plain("hi").alignment(Alignment::Center);
        let lines = text.wrap(10);
        let aligned = text.align_line(lines[0].clone(), 10);
        let plain: String = aligned.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(plain, "    hi    ");
    }
}
