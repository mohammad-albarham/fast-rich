//! Panels for displaying content in a box with optional title.
//!
//! A `Panel` draws a box around content with customizable borders,
//! title, and padding.

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Span, Text};

/// Border style for panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// Standard box drawing characters
    #[default]
    Rounded,
    /// Square corners
    Square,
    /// Heavy/bold borders
    Heavy,
    /// Double-line borders
    Double,
    /// ASCII-only borders
    Ascii,
    /// Minimal borders (dashes)
    Minimal,
    /// No visible border (but space is reserved)
    Hidden,
}

impl BorderStyle {
    /// Get the border characters for this style.
    fn chars(&self) -> BorderChars {
        match self {
            BorderStyle::Rounded => BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            },
            BorderStyle::Square => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
            },
            BorderStyle::Heavy => BorderChars {
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                horizontal: '━',
                vertical: '┃',
            },
            BorderStyle::Double => BorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
            },
            BorderStyle::Ascii => BorderChars {
                top_left: '+',
                top_right: '+',
                bottom_left: '+',
                bottom_right: '+',
                horizontal: '-',
                vertical: '|',
            },
            BorderStyle::Minimal => BorderChars {
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                horizontal: '─',
                vertical: ' ',
            },
            BorderStyle::Hidden => BorderChars {
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                horizontal: ' ',
                vertical: ' ',
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BorderChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
}

/// A panel that wraps content in a box.
#[derive(Debug, Clone)]
pub struct Panel {
    /// The content to display
    content: Text,
    /// Optional title at the top
    title: Option<String>,
    /// Optional subtitle at the bottom
    subtitle: Option<String>,
    /// Border style
    border_style: BorderStyle,
    /// Style for the border
    style: Style,
    /// Style for the title
    title_style: Style,
    /// Horizontal padding inside the box
    padding_x: usize,
    /// Vertical padding inside the box
    padding_y: usize,
    /// Expand to full width
    expand: bool,
}

impl Panel {
    /// Create a new panel with content.
    pub fn new<T: Into<Text>>(content: T) -> Self {
        Panel {
            content: content.into(),
            title: None,
            subtitle: None,
            border_style: BorderStyle::Rounded,
            style: Style::new(),
            title_style: Style::new(),
            padding_x: 1,
            padding_y: 0,
            expand: true,
        }
    }

    /// Set the title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the subtitle.
    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    /// Set the border style.
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set the border color/style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the title style.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Set horizontal padding.
    pub fn padding_x(mut self, padding: usize) -> Self {
        self.padding_x = padding;
        self
    }

    /// Set vertical padding.
    pub fn padding_y(mut self, padding: usize) -> Self {
        self.padding_y = padding;
        self
    }

    /// Set both horizontal and vertical padding.
    pub fn padding(self, x: usize, y: usize) -> Self {
        self.padding_x(x).padding_y(y)
    }

    /// Set whether the panel expands to full width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    fn render_top_border(&self, width: usize, chars: &BorderChars) -> Segment {
        let inner_width = width.saturating_sub(2);

        match &self.title {
            None => {
                let line = chars.horizontal.to_string().repeat(inner_width);
                Segment::line(vec![
                    Span::styled(chars.top_left.to_string(), self.style),
                    Span::styled(line, self.style),
                    Span::styled(chars.top_right.to_string(), self.style),
                ])
            }
            Some(title) => {
                let title_with_space = format!(" {} ", title);
                let title_width = unicode_width::UnicodeWidthStr::width(title_with_space.as_str());

                if title_width >= inner_width {
                    let line = chars.horizontal.to_string().repeat(inner_width);
                    return Segment::line(vec![
                        Span::styled(chars.top_left.to_string(), self.style),
                        Span::styled(line, self.style),
                        Span::styled(chars.top_right.to_string(), self.style),
                    ]);
                }

                let remaining = inner_width - title_width;
                let left_len = 2.min(remaining);
                let right_len = remaining - left_len;

                Segment::line(vec![
                    Span::styled(chars.top_left.to_string(), self.style),
                    Span::styled(chars.horizontal.to_string().repeat(left_len), self.style),
                    Span::styled(title_with_space, self.title_style),
                    Span::styled(chars.horizontal.to_string().repeat(right_len), self.style),
                    Span::styled(chars.top_right.to_string(), self.style),
                ])
            }
        }
    }

    fn render_bottom_border(&self, width: usize, chars: &BorderChars) -> Segment {
        let inner_width = width.saturating_sub(2);

        match &self.subtitle {
            None => {
                let line = chars.horizontal.to_string().repeat(inner_width);
                Segment::line(vec![
                    Span::styled(chars.bottom_left.to_string(), self.style),
                    Span::styled(line, self.style),
                    Span::styled(chars.bottom_right.to_string(), self.style),
                ])
            }
            Some(subtitle) => {
                let sub_with_space = format!(" {} ", subtitle);
                let sub_width = unicode_width::UnicodeWidthStr::width(sub_with_space.as_str());

                if sub_width >= inner_width {
                    let line = chars.horizontal.to_string().repeat(inner_width);
                    return Segment::line(vec![
                        Span::styled(chars.bottom_left.to_string(), self.style),
                        Span::styled(line, self.style),
                        Span::styled(chars.bottom_right.to_string(), self.style),
                    ]);
                }

                let remaining = inner_width - sub_width;
                let right_len = 2.min(remaining);
                let left_len = remaining - right_len;

                Segment::line(vec![
                    Span::styled(chars.bottom_left.to_string(), self.style),
                    Span::styled(chars.horizontal.to_string().repeat(left_len), self.style),
                    Span::styled(sub_with_space, self.title_style),
                    Span::styled(chars.horizontal.to_string().repeat(right_len), self.style),
                    Span::styled(chars.bottom_right.to_string(), self.style),
                ])
            }
        }
    }

    fn render_content_line(&self, spans: Vec<Span>, width: usize, chars: &BorderChars) -> Segment {
        let inner_width = width.saturating_sub(2 + self.padding_x * 2);
        let content_width: usize = spans.iter().map(|s| s.width()).sum();
        let padding_right = inner_width.saturating_sub(content_width);

        let mut line_spans = Vec::new();
        line_spans.push(Span::styled(chars.vertical.to_string(), self.style));
        line_spans.push(Span::raw(" ".repeat(self.padding_x)));
        line_spans.extend(spans);
        line_spans.push(Span::raw(" ".repeat(padding_right + self.padding_x)));
        line_spans.push(Span::styled(chars.vertical.to_string(), self.style));

        Segment::line(line_spans)
    }

    fn render_empty_line(&self, width: usize, chars: &BorderChars) -> Segment {
        let inner_width = width.saturating_sub(2);
        Segment::line(vec![
            Span::styled(chars.vertical.to_string(), self.style),
            Span::raw(" ".repeat(inner_width)),
            Span::styled(chars.vertical.to_string(), self.style),
        ])
    }
}

impl<T: Into<Text>> From<T> for Panel {
    fn from(content: T) -> Self {
        Panel::new(content)
    }
}

impl Renderable for Panel {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let chars = self.border_style.chars();
        let width = if self.expand {
            context.width
        } else {
            let content_width = self.content.width();
            let min_width = content_width + 2 + self.padding_x * 2;
            min_width.min(context.width)
        };

        let inner_width = width.saturating_sub(2 + self.padding_x * 2);
        let content_lines = self.content.wrap(inner_width);

        let mut segments = Vec::new();

        // Top border
        segments.push(self.render_top_border(width, &chars));

        // Top padding
        for _ in 0..self.padding_y {
            segments.push(self.render_empty_line(width, &chars));
        }

        // Content lines
        for line_spans in content_lines {
            segments.push(self.render_content_line(line_spans, width, &chars));
        }

        // Bottom padding
        for _ in 0..self.padding_y {
            segments.push(self.render_empty_line(width, &chars));
        }

        // Bottom border
        segments.push(self.render_bottom_border(width, &chars));

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_simple() {
        let panel = Panel::new("Hello");
        let context = RenderContext { width: 20 };
        let segments = panel.render(&context);

        // Should have top border, content, bottom border
        assert!(segments.len() >= 3);

        // Check top border starts with corner
        let top = segments[0].plain_text();
        assert!(top.starts_with('╭'));
        assert!(top.ends_with('╮'));
    }

    #[test]
    fn test_panel_with_title() {
        let panel = Panel::new("Content").title("Title");
        let context = RenderContext { width: 30 };
        let segments = panel.render(&context);

        let top = segments[0].plain_text();
        assert!(top.contains("Title"));
    }

    #[test]
    fn test_panel_border_styles() {
        let panel = Panel::new("Test").border_style(BorderStyle::Double);
        let context = RenderContext { width: 20 };
        let segments = panel.render(&context);

        let top = segments[0].plain_text();
        assert!(top.starts_with('╔'));
    }
}
