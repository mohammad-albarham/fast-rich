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
        let child_context = RenderContext { width: child_width };

        // Render child
        let child_segments = self.child.render(&child_context);

        let mut result = Vec::new();

        // Top padding (empty lines)
        for _ in 0..self.spec.top {
            result.push(Segment::line(vec![Span::raw(" ".repeat(context.width))]));
        }

        // Add left/right padding to each child segment
        for segment in child_segments {
            let mut padded_spans = Vec::new();

            // Left padding
            if self.spec.left > 0 {
                padded_spans.push(Span::raw(" ".repeat(self.spec.left)));
            }

            // Child content
            padded_spans.extend(segment.spans);

            // Right padding
            if self.spec.right > 0 {
                padded_spans.push(Span::raw(" ".repeat(self.spec.right)));
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
