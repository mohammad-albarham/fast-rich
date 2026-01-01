//! Panels for displaying content in a box with optional title.
//!
//! A `Panel` draws a box around content with customizable borders,
//! title, and padding.

use crate::console::RenderContext;
use crate::bidi::TextDirection;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Span, Text};

use crate::box_drawing::{self, Box};

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
    /// ASCII double-head
    AsciiDoubleHead,
    /// Minimal borders (dashes)
    Minimal,
    /// Minimal heavy head
    MinimalHeavyHead,
    /// Minimal double head
    MinimalDoubleHead,
    /// Horizontals only
    Horizontals,
    /// Square double head
    SquareDoubleHead,
    /// Heavy edge
    HeavyEdge,
    /// Heavy head
    HeavyHead,
    /// Double edge
    DoubleEdge,
    /// No visible border (but space is reserved)
    Hidden,
}

impl BorderStyle {
    /// Get the box characters for this style.
    pub fn to_box(&self) -> Box {
        match self {
            BorderStyle::Rounded => box_drawing::ROUNDED,
            BorderStyle::Square => box_drawing::SQUARE,
            BorderStyle::Heavy => box_drawing::HEAVY,
            BorderStyle::Double => box_drawing::DOUBLE,
            BorderStyle::Ascii => box_drawing::ASCII,
            BorderStyle::AsciiDoubleHead => box_drawing::ASCII_DOUBLE_HEAD,
            BorderStyle::Minimal => box_drawing::MINIMAL,
            BorderStyle::MinimalHeavyHead => box_drawing::MINIMAL_HEAVY_HEAD,
            BorderStyle::MinimalDoubleHead => box_drawing::MINIMAL_DOUBLE_HEAD,
            BorderStyle::Horizontals => box_drawing::HORIZONTALS,
            BorderStyle::SquareDoubleHead => box_drawing::SQUARE_DOUBLE_HEAD,
            BorderStyle::HeavyEdge => box_drawing::HEAVY_EDGE,
            BorderStyle::HeavyHead => box_drawing::HEAVY_HEAD,
            BorderStyle::DoubleEdge => box_drawing::DOUBLE_EDGE,
            BorderStyle::Hidden => Box {
                top: box_drawing::Line::new(' ', ' ', ' ', ' '),
                head: box_drawing::Line::new(' ', ' ', ' ', ' '),
                mid: box_drawing::Line::new(' ', ' ', ' ', ' '),
                bottom: box_drawing::Line::new(' ', ' ', ' ', ' '),
                header: box_drawing::Line::new(' ', ' ', ' ', ' '),
                cell: box_drawing::Line::new(' ', ' ', ' ', ' '),
            },
        }
    }
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

    fn render_top_border(&self, width: usize, box_chars: &Box, is_rtl: bool) -> Segment {
        let inner_width = width.saturating_sub(2);
        let chars = box_chars.top;

        match &self.title {
            None => {
                let line = chars.mid.to_string().repeat(inner_width);
                Segment::line(vec![
                    Span::styled(chars.left.to_string(), self.style),
                    Span::styled(line, self.style),
                    Span::styled(chars.right.to_string(), self.style),
                ])
            }
            Some(title) => {
                let title_with_space = format!(" {} ", title);
                let title_width = unicode_width::UnicodeWidthStr::width(title_with_space.as_str());

                if title_width >= inner_width {
                    let line = chars.mid.to_string().repeat(inner_width);
                    return Segment::line(vec![
                        Span::styled(chars.left.to_string(), self.style),
                        Span::styled(line, self.style),
                        Span::styled(chars.right.to_string(), self.style),
                    ]);
                }

                let remaining = inner_width - title_width;
                // In LTR, title is on Left (small padding left). In RTL, Title on Right (small padding right).
                let (left_len, right_len) = if is_rtl {
                    let r = 2.min(remaining);
                    (remaining - r, r)
                } else {
                    let l = 2.min(remaining);
                    (l, remaining - l)
                };

                Segment::line(vec![
                    Span::styled(chars.left.to_string(), self.style),
                    Span::styled(chars.mid.to_string().repeat(left_len), self.style),
                    Span::styled(title_with_space, self.title_style),
                    Span::styled(chars.mid.to_string().repeat(right_len), self.style),
                    Span::styled(chars.right.to_string(), self.style),
                ])
            }
        }
    }

    fn render_bottom_border(&self, width: usize, box_chars: &Box, is_rtl: bool) -> Segment {
        let inner_width = width.saturating_sub(2);
        let chars = box_chars.bottom;

        match &self.subtitle {
            None => {
                let line = chars.mid.to_string().repeat(inner_width);
                Segment::line(vec![
                    Span::styled(chars.left.to_string(), self.style),
                    Span::styled(line, self.style),
                    Span::styled(chars.right.to_string(), self.style),
                ])
            }
            Some(subtitle) => {
                let sub_with_space = format!(" {} ", subtitle);
                let sub_width = unicode_width::UnicodeWidthStr::width(sub_with_space.as_str());

                if sub_width >= inner_width {
                    let line = chars.mid.to_string().repeat(inner_width);
                    return Segment::line(vec![
                        Span::styled(chars.left.to_string(), self.style),
                        Span::styled(line, self.style),
                        Span::styled(chars.right.to_string(), self.style),
                    ]);
                }

                let remaining = inner_width - sub_width;
                // In LTR, subtitle on Right (small padding right). In RTL, Subtitle on Left (small padding left).
                let (left_len, right_len) = if is_rtl {
                    let l = 2.min(remaining);
                    (l, remaining - l)
                } else {
                    let r = 2.min(remaining);
                    (remaining - r, r)
                };

                Segment::line(vec![
                    Span::styled(chars.left.to_string(), self.style),
                    Span::styled(chars.mid.to_string().repeat(left_len), self.style),
                    Span::styled(sub_with_space, self.title_style),
                    Span::styled(chars.mid.to_string().repeat(right_len), self.style),
                    Span::styled(chars.right.to_string(), self.style),
                ])
            }
        }
    }

    fn render_content_line(&self, spans: Vec<Span>, width: usize, box_chars: &Box, is_rtl: bool) -> Segment {
        let inner_width = width.saturating_sub(2 + self.padding_x * 2);
        let content_width: usize = spans.iter().map(|s| s.width()).sum();
        let padding_right = inner_width.saturating_sub(content_width);
        let chars = box_chars.cell;

        let mut line_spans = Vec::new();
        line_spans.push(Span::styled(chars.left.to_string(), self.style));
        
        let (left_pad, right_pad) = if is_rtl {
            (padding_right + self.padding_x, self.padding_x)
        } else {
            (self.padding_x, padding_right + self.padding_x)
        };

        line_spans.push(Span::styled(" ".repeat(left_pad), self.style));
        line_spans.extend(spans);
        line_spans.push(Span::styled(
            " ".repeat(right_pad),
            self.style,
        ));
        line_spans.push(Span::styled(chars.right.to_string(), self.style));

        Segment::line(line_spans)
    }

    fn render_empty_line(&self, width: usize, box_chars: &Box) -> Segment {
        let inner_width = width.saturating_sub(2);
        let chars = box_chars.cell;
        Segment::line(vec![
            Span::styled(chars.left.to_string(), self.style),
            Span::styled(" ".repeat(inner_width), self.style),
            Span::styled(chars.right.to_string(), self.style),
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
        let is_rtl = matches!(context.direction, TextDirection::Rtl);

        let mut box_chars = self.border_style.to_box();
        // If RTL, swap left/right border characters
        if is_rtl {
             use crate::box_drawing::Line;
             let swap_line = |mut line: Line| {
                 std::mem::swap(&mut line.left, &mut line.right);
                 line
             };
             box_chars.top = swap_line(box_chars.top);
             box_chars.head = swap_line(box_chars.head);
             box_chars.mid = swap_line(box_chars.mid);
             box_chars.bottom = swap_line(box_chars.bottom);
             box_chars.header = swap_line(box_chars.header);
             box_chars.cell = swap_line(box_chars.cell);
        }

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
        segments.push(self.render_top_border(width, &box_chars, is_rtl));

        // Top padding
        for _ in 0..self.padding_y {
            segments.push(self.render_empty_line(width, &box_chars));
        }

        // Content lines
        for line_spans in content_lines {
            segments.push(self.render_content_line(line_spans, width, &box_chars, is_rtl));
        }

        // Bottom padding
        for _ in 0..self.padding_y {
            segments.push(self.render_empty_line(width, &box_chars));
        }

        // Bottom border
        segments.push(self.render_bottom_border(width, &box_chars, is_rtl));

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_simple() {
        let panel = Panel::new("Hello");
        let context = RenderContext {
            width: 20,
            height: None, direction: Default::default(),
        };
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
        let context = RenderContext {
            width: 30,
            height: None, direction: Default::default(),
        };
        let segments = panel.render(&context);

        let top = segments[0].plain_text();
        assert!(top.contains("Title"));
    }

    #[test]
    fn test_panel_border_styles() {
        let panel = Panel::new("Test").border_style(BorderStyle::Double);
        let context = RenderContext {
            width: 20,
            height: None, direction: Default::default(),
        };
        let segments = panel.render(&context);

        let top = segments[0].plain_text();
        assert!(top.starts_with('╔'));
    }
}
