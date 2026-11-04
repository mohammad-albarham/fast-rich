//! Padding wrapper for renderables.
//!
//! Adds configurable padding (spaces) around any renderable content.

use crate::console::RenderContext;
use crate::renderable::{BoxedRenderable, Renderable, Segment};
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
}

impl Padding {
    /// Create a new padding wrapper.
    pub fn new(child: impl Renderable + Send + Sync + 'static, spec: PaddingSpec) -> Self {
        Padding {
            child: Box::new(child),
            spec,
        }
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
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        // Calculate available width for child after horizontal padding
        let child_width = context
            .width
            .saturating_sub(self.spec.left + self.spec.right);
        let child_context = RenderContext {
            width: child_width,
            height: None,
        };

        // Render child
        let child_segments = self.child.render(&child_context);

        let mut result = Vec::new();

        // Top padding (empty lines)
        for _ in 0..self.spec.top {
            result.push(Segment::line(vec![Span::raw(" ".repeat(context.width))]));
        }

        // Lines are padded to the widest child line so the right padding
        // sits at a consistent column (the "column edge") instead of
        // following ragged content.
        let content_width = child_segments
            .iter()
            .map(|seg| seg.spans.iter().map(|s| s.width()).sum::<usize>())
            .max()
            .unwrap_or(0);

        // Add left/right padding to each child segment
        for segment in child_segments {
            let mut padded_spans = Vec::new();

            // Left padding
            if self.spec.left > 0 {
                padded_spans.push(Span::raw(" ".repeat(self.spec.left)));
            }

            // Child content
            padded_spans.extend(segment.spans.clone());

            // Right padding, filling out to the column edge
            let line_width: usize = segment.spans.iter().map(|s| s.width()).sum();
            let right_pad = content_width.saturating_sub(line_width) + self.spec.right;
            if right_pad > 0 {
                padded_spans.push(Span::raw(" ".repeat(right_pad)));
            }

            if segment.newline {
                result.push(Segment::line(padded_spans));
            } else {
                result.push(Segment::new(padded_spans));
            }
        }

        // Bottom padding (empty lines)
        for _ in 0..self.spec.bottom {
            result.push(Segment::line(vec![Span::raw(" ".repeat(context.width))]));
        }

        result
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
