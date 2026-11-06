//! The Renderable trait defines objects that can be rendered to the console.
//!
//! This is the core protocol for rich terminal output, similar to Rich's Console Protocol.

use crate::console::RenderContext;
use crate::text::Span;

/// A segment of renderable output.
#[derive(Debug, Clone)]
pub struct Segment {
    /// The styled spans that make up this segment
    pub spans: Vec<Span>,
    /// Whether this segment ends with a newline
    pub newline: bool,
}

impl Segment {
    /// Create a new segment without a newline.
    pub fn new(spans: Vec<Span>) -> Self {
        Segment {
            spans,
            newline: false,
        }
    }

    /// Create a new segment with a newline at the end.
    pub fn line(spans: Vec<Span>) -> Self {
        Segment {
            spans,
            newline: true,
        }
    }

    /// Create an empty line segment.
    pub fn empty_line() -> Self {
        Segment {
            spans: Vec::new(),
            newline: true,
        }
    }

    /// Create a segment from a single span.
    pub fn from_span(span: Span) -> Self {
        Segment {
            spans: vec![span],
            newline: false,
        }
    }

    /// Get the total display width of this segment.
    pub fn width(&self) -> usize {
        self.spans.iter().map(|s| s.width()).sum()
    }

    /// Get the plain text content.
    pub fn plain_text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_ref()).collect()
    }
}

/// Trait for objects that can be rendered to the console.
///
/// This is the core abstraction for renderable content, similar to Rich's
/// `__rich_console__` protocol.
pub trait Renderable {
    /// Render this object to a sequence of segments.
    ///
    /// The `context` provides information about the rendering environment,
    /// such as available width.
    fn render(&self, context: &RenderContext) -> Vec<Segment>;

    /// Get the minimum width required to render this object.
    fn min_width(&self) -> usize {
        1
    }

    /// Get the maximum/natural width of this object.
    fn max_width(&self) -> usize {
        usize::MAX
    }
}

/// Implement Renderable for String.
impl Renderable for String {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        vec![Segment::new(vec![Span::raw(self.clone())])]
    }

    fn max_width(&self) -> usize {
        unicode_width::UnicodeWidthStr::width(self.as_str())
    }
}

/// Implement Renderable for &str.
impl Renderable for &str {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        vec![Segment::new(vec![Span::raw(self.to_string())])]
    }

    fn max_width(&self) -> usize {
        unicode_width::UnicodeWidthStr::width(*self)
    }
}

/// Implement Renderable for Text.
impl Renderable for crate::text::Text {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let lines = self.wrap(context.width);
        lines
            .into_iter()
            .map(|line| {
                let aligned = self.align_line(line, context.width);
                Segment::line(aligned)
            })
            .collect()
    }

    fn min_width(&self) -> usize {
        // Minimum width is the longest word
        self.spans
            .iter()
            .flat_map(|s| s.text.split_whitespace())
            .map(unicode_width::UnicodeWidthStr::width)
            .max()
            .unwrap_or(1)
    }

    fn max_width(&self) -> usize {
        // Natural width is the widest line, not the sum of all lines.
        self.plain_text()
            .split('\n')
            .map(unicode_width::UnicodeWidthStr::width)
            .max()
            .unwrap_or(0)
    }
}

/// A boxed renderable for dynamic dispatch.
pub type BoxedRenderable = Box<dyn Renderable + Send + Sync>;

/// A shared renderable: what containers (`Panel`, `Table` cells, `Tree`
/// labels) store so they stay `Clone`.
pub type SharedRenderable = std::sync::Arc<dyn Renderable + Send + Sync>;

impl Renderable for SharedRenderable {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        (**self).render(context)
    }

    fn min_width(&self) -> usize {
        (**self).min_width()
    }

    fn max_width(&self) -> usize {
        (**self).max_width()
    }
}

/// Split rendered segments into physical lines.
///
/// A line ends at a segment with `newline` set or at a `\n` inside a span,
/// so renderables that emit raw multi-line strings are handled too.
pub fn segments_to_lines(segments: Vec<Segment>) -> Vec<Vec<Span>> {
    let mut lines = Vec::new();
    let mut current: Vec<Span> = Vec::new();
    let mut open = false;
    for segment in segments {
        for span in segment.spans {
            if span.text.contains('\n') {
                let mut parts = span.text.split('\n').peekable();
                while let Some(part) = parts.next() {
                    if !part.is_empty() {
                        current.push(Span {
                            text: part.to_string().into(),
                            style: span.style,
                            link: span.link.clone(),
                        });
                        open = true;
                    }
                    if parts.peek().is_some() {
                        lines.push(std::mem::take(&mut current));
                        open = false;
                    }
                }
            } else {
                if !span.text.is_empty() {
                    open = true;
                }
                current.push(span);
            }
        }
        if segment.newline {
            lines.push(std::mem::take(&mut current));
            open = false;
        }
    }
    if open || !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// Render `renderable` at `width` and return its lines, each padded with
/// spaces (in `pad_style`) or cropped to exactly `width` cells — like Rich's
/// `Console.render_lines(pad=True)`.
///
/// ```
/// use fast_rich::renderable::render_lines;
/// use fast_rich::style::Style;
/// use fast_rich::text::Text;
/// let lines = render_lines(&Text::plain("ab\ncdef"), 3, Style::new());
/// let plain: Vec<String> = lines
///     .iter()
///     .map(|l| l.iter().map(|s| s.text.as_ref()).collect())
///     .collect();
/// assert_eq!(plain, ["ab ", "cde", "f  "]);
/// ```
pub fn render_lines(
    renderable: &dyn Renderable,
    width: usize,
    pad_style: crate::style::Style,
) -> Vec<Vec<Span>> {
    // Like Rich's Console.render: no space, nothing to render.
    if width == 0 {
        return Vec::new();
    }
    let context = RenderContext {
        width,
        height: None,
    };
    segments_to_lines(renderable.render(&context))
        .into_iter()
        .map(|line| fit_line(line, width, pad_style))
        .collect()
}

/// Crop a line to at most `width` cells (never splitting a wide character)
/// and pad it with spaces in `pad_style` up to exactly `width`.
pub fn fit_line(line: Vec<Span>, width: usize, pad_style: crate::style::Style) -> Vec<Span> {
    use unicode_segmentation::UnicodeSegmentation;
    use unicode_width::UnicodeWidthStr;

    let mut out = Vec::with_capacity(line.len() + 1);
    let mut used = 0usize;
    for span in line {
        let w = span.width();
        if used + w <= width {
            used += w;
            out.push(span);
            continue;
        }
        let mut kept = String::new();
        for g in span.text.graphemes(true) {
            let gw = UnicodeWidthStr::width(g);
            if used + gw > width {
                break;
            }
            used += gw;
            kept.push_str(g);
        }
        if !kept.is_empty() {
            out.push(Span {
                text: kept.into(),
                style: span.style,
                link: span.link,
            });
        }
        break;
    }
    if used < width {
        out.push(Span::styled(" ".repeat(width - used), pad_style));
    }
    out
}

/// Display width of a line of spans.
pub fn line_width(line: &[Span]) -> usize {
    line.iter().map(Span::width).sum()
}

impl Renderable for BoxedRenderable {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        (**self).render(context)
    }

    fn min_width(&self) -> usize {
        (**self).min_width()
    }

    fn max_width(&self) -> usize {
        (**self).max_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_width() {
        let segment = Segment::new(vec![Span::raw("hello")]);
        assert_eq!(segment.width(), 5);
    }

    #[test]
    fn test_segment_plain_text() {
        let segment = Segment::new(vec![Span::raw("hello"), Span::raw(" world")]);
        assert_eq!(segment.plain_text(), "hello world");
    }

    #[test]
    fn test_string_renderable() {
        let s = "Hello, World!".to_string();
        let context = RenderContext {
            width: 80,
            height: None,
        };
        let segments = s.render(&context);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].plain_text(), "Hello, World!");
    }
}
