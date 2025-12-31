//! Highlighters for pattern-based text styling.
//!
//! Provides regex-based highlighting and built-in highlighters for common patterns.

use crate::style::Style;
use crate::text::{Span, Text};
use regex::Regex;

/// Trait for text highlighters.
pub trait Highlighter {
    /// Highlight text and return styled spans.
    fn highlight(&self, text: &str) -> Vec<Span>;
}

/// A regex-based highlighter that applies styles to matched patterns.
#[derive(Debug, Clone)]
pub struct RegexHighlighter {
    patterns: Vec<(Regex, Style)>,
}

impl RegexHighlighter {
    /// Create a new empty regex highlighter.
    pub fn new() -> Self {
        RegexHighlighter {
            patterns: Vec::new(),
        }
    }

    /// Add a pattern with associated style.
    pub fn add_pattern(&mut self, pattern: &str, style: Style) -> Result<(), regex::Error> {
        let regex = Regex::new(pattern)?;
        self.patterns.push((regex, style));
        Ok(())
    }

    /// Builder method to add a pattern.
    pub fn with_pattern(mut self, pattern: &str, style: Style) -> Result<Self, regex::Error> {
        self.add_pattern(pattern, style)?;
        Ok(self)
    }

    /// Create a highlighter for URLs.
    pub fn url_highlighter(style: Style) -> Self {
        let mut hl = RegexHighlighter::new();
        // Simple URL pattern
        let _ = hl.add_pattern(r"https?://[^\s]+", style);
        hl
    }

    /// Create a highlighter for numbers.
    pub fn number_highlighter(style: Style) -> Self {
        let mut hl = RegexHighlighter::new();
        let _ = hl.add_pattern(r"\b\d+\.?\d*\b", style);
        hl
    }

    /// Create a highlighter for email addresses.
    pub fn email_highlighter(style: Style) -> Self {
        let mut hl = RegexHighlighter::new();
        let _ = hl.add_pattern(
            r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b",
            style,
        );
        hl
    }
}

impl Default for RegexHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for RegexHighlighter {
    fn highlight(&self, text: &str) -> Vec<Span> {
        if self.patterns.is_empty() {
            return vec![Span::raw(text.to_string())];
        }

        // Find all matches across all patterns
        let mut matches: Vec<(usize, usize, Style)> = Vec::new();

        for (regex, style) in &self.patterns {
            for m in regex.find_iter(text) {
                matches.push((m.start(), m.end(), *style));
            }
        }

        // Sort matches by start position
        matches.sort_by_key(|m| m.0);

        // Build spans, handling overlaps (first match wins)
        let mut spans = Vec::new();
        let mut last_end = 0;

        for (start, end, style) in matches {
            // Skip if this match overlaps with previous
            if start < last_end {
                continue;
            }

            // Add unstyled text before match
            if start > last_end {
                spans.push(Span::raw(text[last_end..start].to_string()));
            }

            // Add styled match
            spans.push(Span::styled(text[start..end].to_string(), style));
            last_end = end;
        }

        // Add remaining unstyled text
        if last_end < text.len() {
            spans.push(Span::raw(text[last_end..].to_string()));
        }

        if spans.is_empty() {
            vec![Span::raw(text.to_string())]
        } else {
            spans
        }
    }
}

/// Apply a highlighter to text and return a styled Text object.
pub fn highlight_text(text: &str, highlighter: &impl Highlighter) -> Text {
    let spans = highlighter.highlight(text);
    Text::from_spans(spans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_regex_highlighter() {
        let mut hl = RegexHighlighter::new();
        hl.add_pattern(r"\d+", Style::new().foreground(Color::Cyan))
            .unwrap();

        let spans = hl.highlight("Port 8080 is open");
        assert_eq!(spans.len(), 3); // "Port ", "8080", " is open"
    }

    #[test]
    fn test_url_highlighter() {
        let hl = RegexHighlighter::url_highlighter(Style::new().foreground(Color::Blue));
        let spans = hl.highlight("Visit https://example.com for info");
        assert!(spans.len() > 1);
    }
}
