//! Padding wrapper for renderables.
//!
//! Adds configurable padding (spaces) around any renderable content.

use crate::console::RenderContext;
use crate::renderable::{render_lines, BoxedRenderable, Renderable, Segment};
use crate::style::Style;
use crate::text::Span;

/// Padding specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaddingSpec {
    /// Top padding (lines)
    pub top: usize,
    /// Right padding (spaces)
    pub right: usize,
    /// Bottom padding (lines)
    pub bottom: usize,
    /// Left padding (spaces)
    pub left: usize,
}

impl PaddingSpec {
    /// Create padding with all sides equal.
    pub fn all(size: usize) -> Self {
        PaddingSpec {
            top: size,
            right: size,
            bottom: size,
            left: size,
        }
    }

    /// Create padding with vertical and horizontal values.
    pub fn symmetric(vertical: usize, horizontal: usize) -> Self {
        PaddingSpec {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create padding for each side individually.
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        PaddingSpec {
            top,
            right,
            bottom,
            left,
        }
    }

    /// No padding.
    pub fn none() -> Self {
        PaddingSpec::all(0)
    }
}

/// A renderable that adds padding around its child.
pub struct Padding {
    child: BoxedRenderable,
    spec: PaddingSpec,
    expand: bool,
    style: Style,
}

impl Padding {
    /// Create a new padding wrapper.
    pub fn new(child: impl Renderable + Send + Sync + 'static, spec: PaddingSpec) -> Self {
        Padding {
            child: Box::new(child),
            spec,
            expand: true,
            style: Style::new(),
        }
    }

    /// Fill the available width (default, like Rich) or fit the child.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Style for the padding area (Rich's `Padding(style=...)`).
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Create padding with all sides equal.
    pub fn all(child: impl Renderable + Send + Sync + 'static, size: usize) -> Self {
        Padding::new(child, PaddingSpec::all(size))
    }

    /// Create padding with vertical and horizontal values.
    pub fn symmetric(
        child: impl Renderable + Send + Sync + 'static,
        vertical: usize,
        horizontal: usize,
    ) -> Self {
        Padding::new(child, PaddingSpec::symmetric(vertical, horizontal))
    }
}

impl Renderable for Padding {
    /// Rich's `Padding`: the child is rendered at the width left after the
    /// left/right padding and every line is padded to the full width.
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let horizontal = self.spec.left + self.spec.right;
        let width = if self.expand {
            context.width
        } else {
            self.child
                .max_width()
                .saturating_add(horizontal)
                .min(context.width)
        };
        let child_width = width.saturating_sub(horizontal);
        let blank = || Segment::line(vec![Span::styled(" ".repeat(width), self.style)]);
        let mut result = Vec::new();
        result.extend((0..self.spec.top).map(|_| blank()));
        for line in render_lines(&*self.child, child_width, self.style) {
            let mut spans = Vec::with_capacity(line.len() + 2);
            if self.spec.left > 0 {
                spans.push(Span::styled(" ".repeat(self.spec.left), self.style));
            }
            spans.extend(line);
            if self.spec.right > 0 {
                spans.push(Span::styled(" ".repeat(self.spec.right), self.style));
            }
            result.push(Segment::line(spans));
        }
        result.extend((0..self.spec.bottom).map(|_| blank()));
        result
    }

    fn min_width(&self) -> usize {
        self.child
            .min_width()
            .saturating_add(self.spec.left + self.spec.right)
    }

    fn max_width(&self) -> usize {
        self.child
            .max_width()
            .saturating_add(self.spec.left + self.spec.right)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::RenderContext;
    use crate::text::Text;

    #[test]
    fn test_padding_spec_builders() {
        let spec = PaddingSpec::all(2);
        assert_eq!((spec.top, spec.right, spec.bottom, spec.left), (2, 2, 2, 2));
        let sym = PaddingSpec::symmetric(1, 3);
        assert_eq!((sym.top, sym.left), (1, 3));
        let none = PaddingSpec::none();
        assert_eq!(none.top + none.right + none.bottom + none.left, 0);
    }

    #[test]
    fn test_padding_render_pads_to_column_edge() {
        let padded = Padding::all(Text::plain("hi".to_string()), 1);
        let segments = padded.render(&RenderContext {
            width: 40,
            height: None,
        });
        assert!(!segments.is_empty());
        let text: String = segments.iter().map(|s| s.plain_text()).collect();
        assert!(text.contains(" hi "), "got {text:?}");
    }
}
