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
        self.width()
    }
}

/// A boxed renderable for dynamic dispatch.
pub type BoxedRenderable = Box<dyn Renderable + Send + Sync>;

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
        let context = RenderContext { width: 80 };
        let segments = s.render(&context);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].plain_text(), "Hello, World!");
    }
}
