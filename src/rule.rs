//! Horizontal rules for visual separation.
//!
//! A `Rule` draws a horizontal line across the terminal, optionally with
//! a centered title.

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::Span;

/// A horizontal rule/line.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Optional title in the center
    title: Option<String>,
    /// Character to use for the line
    character: char,
    /// Style for the rule
    style: Style,
    /// Style for the title
    title_style: Style,
    /// Alignment of the title
    align: RuleAlign,
}

/// Alignment for rule title.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuleAlign {
    /// Left-aligned title
    Left,
    /// Center-aligned title (default)
    #[default]
    Center,
    /// Right-aligned title
    Right,
}

impl Rule {
    /// Create a new rule with optional title.
    pub fn new(title: &str) -> Self {
        Rule {
            title: if title.is_empty() {
                None
            } else {
                Some(title.to_string())
            },
            character: '─',
            style: Style::new(),
            title_style: Style::new(),
            align: RuleAlign::Center,
        }
    }

    /// Create a rule without a title.
    pub fn line() -> Self {
        Rule {
            title: None,
            character: '─',
            style: Style::new(),
            title_style: Style::new(),
            align: RuleAlign::Center,
        }
    }

    /// Set the character used for the line.
    pub fn character(mut self, c: char) -> Self {
        self.character = c;
        self
    }

    /// Set the style for the rule line.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the style for the title.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Set the title alignment.
    pub fn align(mut self, align: RuleAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule::line()
    }
}

impl Renderable for Rule {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let width = context.width;

        match &self.title {
            None => {
                // Simple line
                let line = self.character.to_string().repeat(width);
                vec![Segment::new(vec![Span::styled(line, self.style)])]
            }
            Some(title) => {
                let title_with_spacing = format!(" {} ", title);
                let title_width =
                    unicode_width::UnicodeWidthStr::width(title_with_spacing.as_str());

                if title_width >= width {
                    // Title is too long, just show title
                    return vec![Segment::new(vec![Span::styled(
                        title.clone(),
                        self.title_style,
                    )])];
                }

                let remaining = width - title_width;

                let (left_len, right_len) = match self.align {
                    RuleAlign::Left => (4.min(remaining), remaining.saturating_sub(4)),
                    RuleAlign::Center => {
                        let left = remaining / 2;
                        (left, remaining - left)
                    }
                    RuleAlign::Right => (remaining.saturating_sub(4), 4.min(remaining)),
                };

                let left_line = self.character.to_string().repeat(left_len);
                let right_line = self.character.to_string().repeat(right_len);

                vec![Segment::new(vec![
                    Span::styled(left_line, self.style),
                    Span::styled(title_with_spacing, self.title_style),
                    Span::styled(right_line, self.style),
                ])]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_simple() {
        let rule = Rule::line();
        let context = RenderContext { width: 10 };
        let segments = rule.render(&context);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].plain_text(), "──────────");
    }

    #[test]
    fn test_rule_with_title() {
        let rule = Rule::new("Title");
        let context = RenderContext { width: 20 };
        let segments = rule.render(&context);
        let text = segments[0].plain_text();
        assert!(text.contains("Title"));
        assert!(text.starts_with("─"));
        assert!(text.ends_with("─"));
    }

    #[test]
    fn test_rule_custom_char() {
        let rule = Rule::line().character('=');
        let context = RenderContext { width: 5 };
        let segments = rule.render(&context);
        assert_eq!(segments[0].plain_text(), "=====");
    }
}
