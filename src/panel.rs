//! Panels for displaying content in a box with optional title.
//!
//! A `Panel` draws a box around content with customizable borders,
//! title, and padding.

use crate::console::RenderContext;
use crate::padding::PaddingSpec;
use crate::renderable::{render_lines, Renderable, Segment, SharedRenderable};
use crate::style::Style;
use crate::text::{Alignment, Span, Text};
use std::sync::Arc;

use crate::box_drawing::{self, BoxStyle};

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
    /// Custom box characters (e.g. built from a Rich box definition)
    Custom(BoxStyle),
}

impl BorderStyle {
    /// Get the box characters for this style.
    pub fn to_box(&self) -> BoxStyle {
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
            BorderStyle::Custom(b) => *b,
            BorderStyle::Hidden => BoxStyle {
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
///
/// The content can be any [`Renderable`] (see [`Panel::renderable`]), so
/// tables, trees or other panels can be nested, as in Rich.
#[derive(Clone)]
pub struct Panel {
    /// The content to display
    content: SharedRenderable,
    /// Optional title at the top
    title: Option<Text>,
    /// Title alignment within the top border
    title_align: Alignment,
    /// Optional subtitle at the bottom
    subtitle: Option<Text>,
    /// Subtitle alignment within the bottom border
    subtitle_align: Alignment,
    /// Style for the subtitle
    subtitle_style: Style,
    /// Border style
    border_style: BorderStyle,
    /// Style for the border
    style: Style,
    /// Base style for the content area (Rich's `Panel(style=...)`)
    content_style: Style,
    /// Style for the title
    title_style: Style,
    /// Padding around the content
    padding: PaddingSpec,
    /// Expand to full width
    expand: bool,
    /// Fixed total width (including borders)
    width: Option<usize>,
}

impl std::fmt::Debug for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Panel")
            .field("title", &self.title.as_ref().map(Text::plain_text))
            .field("subtitle", &self.subtitle.as_ref().map(Text::plain_text))
            .field("border_style", &self.border_style)
            .field("padding", &self.padding)
            .field("expand", &self.expand)
            .field("width", &self.width)
            .finish_non_exhaustive()
    }
}

impl Panel {
    /// Create a new panel with text content.
    pub fn new<T: Into<Text>>(content: T) -> Self {
        Self::renderable(content.into())
    }

    /// Create a panel around any renderable (a table, tree, another panel…).
    ///
    /// ```
    /// use fast_rich::panel::Panel;
    /// use fast_rich::renderable::render_lines;
    /// use fast_rich::style::Style;
    /// let inner = Panel::new("in");
    /// let outer = Panel::renderable(inner);
    /// let lines = render_lines(&outer, 20, Style::new());
    /// assert_eq!(lines.len(), 5);
    /// ```
    pub fn renderable(content: impl Renderable + Send + Sync + 'static) -> Self {
        Panel {
            content: Arc::new(content),
            title: None,
            title_align: Alignment::Center,
            subtitle: None,
            subtitle_align: Alignment::Center,
            border_style: BorderStyle::Rounded,
            style: Style::new(),
            content_style: Style::new(),
            title_style: Style::new(),
            subtitle_style: Style::new(),
            padding: PaddingSpec::symmetric(0, 1),
            // Fit the content by default; call `expand(true)` for Rich's
            // full-width default.
            expand: false,
            width: None,
        }
    }

    /// Set the title.
    pub fn title(self, title: &str) -> Self {
        self.title_text(Text::plain(title.to_string()))
    }

    /// Set the title from styled text.
    pub fn title_text(mut self, title: Text) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the title alignment (default: center, like Rich).
    pub fn title_align(mut self, align: Alignment) -> Self {
        self.title_align = align;
        self
    }

    /// Set the subtitle.
    pub fn subtitle(self, subtitle: &str) -> Self {
        self.subtitle_text(Text::plain(subtitle.to_string()))
    }

    /// Set the subtitle from styled text.
    pub fn subtitle_text(mut self, subtitle: Text) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    /// Set the subtitle alignment (default: center, like Rich).
    pub fn subtitle_align(mut self, align: Alignment) -> Self {
        self.subtitle_align = align;
        self
    }

    /// Set the border style.
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set the style (border color).
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the base style of the whole panel (Rich's `style`): it applies
    /// under the border style and under the content's own styles.
    pub fn content_style(mut self, style: Style) -> Self {
        self.content_style = style;
        self
    }

    /// Set the title style.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Set the subtitle style.
    pub fn subtitle_style(mut self, style: Style) -> Self {
        self.subtitle_style = style;
        self
    }

    /// Set horizontal padding.
    pub fn padding_x(mut self, padding: usize) -> Self {
        self.padding.left = padding;
        self.padding.right = padding;
        self
    }

    /// Set vertical padding.
    pub fn padding_y(mut self, padding: usize) -> Self {
        self.padding.top = padding;
        self.padding.bottom = padding;
        self
    }

    /// Set both horizontal and vertical padding.
    pub fn padding(self, x: usize, y: usize) -> Self {
        self.padding_x(x).padding_y(y)
    }

    /// Set the padding on each side (top, right, bottom, left).
    pub fn padding_spec(mut self, padding: PaddingSpec) -> Self {
        self.padding = padding;
        self
    }

    /// Set whether to expand to the full available width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Set a fixed total width (borders included). It is still capped by
    /// the available width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    fn title_width(&self) -> usize {
        self.title
            .as_ref()
            .map(|t| one_line(t).iter().map(Span::width).sum::<usize>() + 2)
            .unwrap_or(0)
    }

    /// Natural width: content + padding + borders (or the fixed width).
    fn natural_width(&self) -> usize {
        if let Some(w) = self.width {
            return w;
        }
        let pad = self.padding.left + self.padding.right;
        self.content
            .max_width()
            .max(self.title_width())
            .saturating_add(pad + 2)
    }

    /// Build a top or bottom border line with an optional aligned label.
    fn border_line(
        &self,
        line: &box_drawing::Line,
        label: Option<&Text>,
        align: Alignment,
        label_style: Style,
        width: usize,
    ) -> Segment {
        let border = self.content_style.combine(&self.style);
        let label = match label {
            Some(l) if width > 4 => l,
            _ => {
                return Segment::line(vec![Span::styled(
                    format!(
                        "{}{}{}",
                        line.left,
                        line.mid.to_string().repeat(width.saturating_sub(2)),
                        line.right
                    ),
                    border,
                )]);
            }
        };
        // Like Rich: " label " is truncated/aligned within width - 4 and
        // framed by corner + one border char on each side.
        let avail = width - 4;
        let mut spans = vec![Span::raw(" ")];
        spans.extend(one_line(label));
        spans.push(Span::raw(" "));
        // " label " cut to the space between the corners.
        let text_width = crate::renderable::line_width(&spans).min(avail);
        let spans = crate::renderable::fit_line(spans, text_width, Style::new());
        let excess = avail - text_width;
        let (left, right) = match align {
            Alignment::Left => (0, excess),
            Alignment::Center => (excess / 2, excess - excess / 2),
            Alignment::Right => (excess, 0),
        };
        let fill = |n: usize| line.mid.to_string().repeat(n);
        let mut out = vec![Span::styled(
            format!("{}{}{}", line.left, line.mid, fill(left)),
            border,
        )];
        out.extend(spans.into_iter().map(|mut s| {
            s.style = border.combine(&label_style).combine(&s.style);
            s
        }));
        out.push(Span::styled(
            format!("{}{}{}", fill(right), line.mid, line.right),
            border,
        ));
        Segment::line(out)
    }
}

/// A title's spans on a single line (newlines become spaces).
fn one_line(text: &Text) -> Vec<Span> {
    text.spans
        .iter()
        .map(|s| {
            let mut s = s.clone();
            if s.text.contains('\n') {
                s.text = s.text.replace('\n', " ").into();
            }
            s
        })
        .collect()
}

impl<T: Into<Text>> From<T> for Panel {
    fn from(content: T) -> Self {
        Panel::new(content)
    }
}

impl Renderable for Panel {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let box_chars = self.border_style.to_box();
        let max_width = context.width;
        let width = self.width.unwrap_or(max_width).min(max_width);
        let pad_x = self.padding.left + self.padding.right;

        let mut child_width = if self.expand || self.width.is_some() {
            width.saturating_sub(2)
        } else {
            width
                .saturating_sub(2)
                .min(self.content.max_width().saturating_add(pad_x))
        };
        if self.title.is_some() {
            child_width = max_width
                .saturating_sub(2)
                .min(child_width.max(self.title_width() + 2));
        }
        let width = child_width + 2;
        let text_width = child_width.saturating_sub(pad_x);

        let border = self.content_style.combine(&self.style);
        let cell = box_chars.cell;
        let blank = Span::styled(" ".repeat(child_width), self.content_style);
        let side = |c: char| Span::styled(c.to_string(), border);

        let mut segments = Vec::new();
        segments.push(self.border_line(
            &box_chars.top,
            self.title.as_ref(),
            self.title_align,
            self.title_style,
            width,
        ));
        for _ in 0..self.padding.top {
            segments.push(Segment::line(vec![
                side(cell.left),
                blank.clone(),
                side(cell.right),
            ]));
        }
        for line in render_lines(&*self.content, text_width, self.content_style) {
            let mut spans = vec![side(cell.left)];
            if self.padding.left > 0 {
                spans.push(Span::styled(
                    " ".repeat(self.padding.left),
                    self.content_style,
                ));
            }
            spans.extend(line.into_iter().map(|mut span| {
                span.style = self.content_style.combine(&span.style);
                span
            }));
            if self.padding.right > 0 {
                spans.push(Span::styled(
                    " ".repeat(self.padding.right),
                    self.content_style,
                ));
            }
            spans.push(side(cell.right));
            segments.push(Segment::line(spans));
        }
        for _ in 0..self.padding.bottom {
            segments.push(Segment::line(vec![
                side(cell.left),
                blank.clone(),
                side(cell.right),
            ]));
        }
        segments.push(self.border_line(
            &box_chars.bottom,
            self.subtitle.as_ref(),
            self.subtitle_align,
            self.subtitle_style,
            width,
        ));
        segments
    }

    fn min_width(&self) -> usize {
        self.natural_width()
    }

    fn max_width(&self) -> usize {
        self.natural_width()
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
            height: None,
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
            height: None,
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
            height: None,
        };
        let segments = panel.render(&context);

        let top = segments[0].plain_text();
        assert!(top.starts_with('╔'));
    }

    fn plain(r: &dyn Renderable, width: usize) -> String {
        crate::renderable::render_lines(r, width, Style::new())
            .iter()
            .map(|l| l.iter().map(|s| s.text.as_ref()).collect::<String>())
            .map(|l| l.trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_nested_panel_matches_rich() {
        let panel = Panel::renderable(Panel::new("in"))
            .expand(true)
            .title("T")
            .title_align(Alignment::Left)
            .subtitle("s")
            .subtitle_align(Alignment::Right);
        assert_eq!(
            plain(&panel, 30),
            "╭─ T ────────────────────────╮\n\
             │ ╭────╮                     │\n\
             │ │ in │                     │\n\
             │ ╰────╯                     │\n\
             ╰──────────────────────── s ─╯"
        );
    }

    #[test]
    fn test_fixed_width_and_padding_spec() {
        let panel = Panel::new("x")
            .width(12)
            .padding_spec(PaddingSpec::symmetric(1, 2));
        assert_eq!(
            plain(&panel, 30),
            "╭──────────╮\n│          │\n│  x       │\n│          │\n╰──────────╯"
        );
        assert_eq!(panel.max_width(), 12);
    }

    #[test]
    fn test_fit_measures_display_width() {
        let panel = Panel::new("日本語");
        assert_eq!(plain(&panel, 30), "╭────────╮\n│ 日本語 │\n╰────────╯");
        assert_eq!(panel.max_width(), 10);
    }

    #[test]
    fn test_title_text_keeps_styles_and_custom_box() {
        let title = crate::markup::parse("[bold]Hi[/]");
        let custom = BorderStyle::Custom(box_drawing::ASCII);
        let panel = Panel::new("x").title_text(title).border_style(custom);
        let segs = panel.render(&RenderContext {
            width: 20,
            height: None,
        });
        let hi = segs[0]
            .spans
            .iter()
            .find(|s| s.text.contains("Hi"))
            .unwrap();
        assert!(hi.style.bold);
        assert_eq!(segs[0].plain_text(), "+- Hi -+");
    }
}
