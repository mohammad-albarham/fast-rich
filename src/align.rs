//! Alignment wrapper for renderables.

use crate::console::RenderContext;
use crate::renderable::{render_lines, BoxedRenderable, Renderable, Segment};
use crate::style::Style;
use crate::text::{Alignment, Span};

/// Vertical alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalAlignment {
    /// Top-aligned (default)
    #[default]
    Top,
    /// Middle-aligned
    Middle,
    /// Bottom-aligned
    Bottom,
}

/// A renderable that aligns its child within the available space.
pub struct Align {
    child: BoxedRenderable,
    align: Alignment,
    vertical: VerticalAlignment,
    height: Option<usize>,
    pad: bool,
}

impl Align {
    /// Control whether the aligned output is padded to the full width
    /// (default: `true`).
    pub fn pad(mut self, pad: bool) -> Self {
        self.pad = pad;
        self
    }

    /// Create a new left-aligned wrapper.
    /// Align `child` with the given horizontal alignment.
    pub fn new(child: impl Renderable + Send + Sync + 'static, align: Alignment) -> Self {
        Align {
            child: Box::new(child),
            align,
            vertical: VerticalAlignment::Top,
            height: None,
            pad: true,
        }
    }

    pub fn left(child: impl Renderable + Send + Sync + 'static) -> Self {
        Align {
            child: Box::new(child),
            align: Alignment::Left,
            vertical: VerticalAlignment::Top,
            height: None,
            pad: true,
        }
    }

    /// Create a new center-aligned wrapper.
    pub fn center(child: impl Renderable + Send + Sync + 'static) -> Self {
        Align {
            child: Box::new(child),
            align: Alignment::Center,
            vertical: VerticalAlignment::Top,
            height: None,
            pad: true,
        }
    }

    /// Create a new right-aligned wrapper.
    pub fn right(child: impl Renderable + Send + Sync + 'static) -> Self {
        Align {
            child: Box::new(child),
            align: Alignment::Right,
            vertical: VerticalAlignment::Top,
            height: None,
            pad: true,
        }
    }

    /// Set vertical alignment.
    pub fn vertical(mut self, vertical: VerticalAlignment) -> Self {
        self.vertical = vertical;
        self
    }

    /// Set height constraint.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }
}

impl Renderable for Align {
    /// Rich's `Align`: the child is rendered as a block at its natural
    /// width (capped by the available width), every line padded to the
    /// block width, then the block is placed left/center/right.
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let width = context.width;
        let block_width = self.child.max_width().min(width);
        let mut lines = render_lines(&*self.child, block_width, Style::new());
        if let Some(target) = self.height {
            let blank = || vec![Span::raw(" ".repeat(block_width))];
            let diff = target.saturating_sub(lines.len());
            let top = match self.vertical {
                VerticalAlignment::Top => 0,
                VerticalAlignment::Middle => diff / 2,
                VerticalAlignment::Bottom => diff,
            };
            let mut shaped = Vec::with_capacity(lines.len() + diff);
            shaped.extend((0..top).map(|_| blank()));
            shaped.append(&mut lines);
            shaped.extend((0..diff - top).map(|_| blank()));
            lines = shaped;
        }
        let excess = width.saturating_sub(block_width);
        let (left, right) = match self.align {
            Alignment::Left => (0, if self.pad { excess } else { 0 }),
            Alignment::Center => (excess / 2, if self.pad { excess - excess / 2 } else { 0 }),
            Alignment::Right => (excess, 0),
        };
        lines
            .into_iter()
            .map(|line| {
                let mut spans = Vec::with_capacity(line.len() + 2);
                if left > 0 {
                    spans.push(Span::raw(" ".repeat(left)));
                }
                spans.extend(line);
                if right > 0 {
                    spans.push(Span::raw(" ".repeat(right)));
                }
                Segment::line(spans)
            })
            .collect()
    }

    fn min_width(&self) -> usize {
        self.child.min_width()
    }

    fn max_width(&self) -> usize {
        self.child.max_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::RenderContext;
    use crate::text::Text;

    fn render_aligned(a: &Align, width: usize) -> Vec<String> {
        a.render(&RenderContext {
            width,
            height: None,
        })
        .iter()
        .map(|s| s.plain_text())
        .collect()
    }

    #[test]
    fn test_center_alignment_pads_both_sides() {
        let align = Align::center(Text::plain("hi".to_string()));
        let lines = render_aligned(&align, 10);
        assert!(lines[0].contains("hi"));
        assert!(
            lines[0].starts_with(' '),
            "center should indent: {:?}",
            lines[0]
        );
    }

    #[test]
    fn test_right_alignment_indents() {
        let align = Align::right(Text::plain("hi".to_string()));
        let lines = render_aligned(&align, 10);
        assert!(lines[0].trim_end().ends_with("hi"));
        assert!(
            lines[0].starts_with("  "),
            "right should push away from edge: {:?}",
            lines[0]
        );
    }

    #[test]
    fn test_pad_false_omits_trailing_padding() {
        let padded = Align::left(Text::plain("hi".to_string())).pad(true);
        let unpadded = Align::left(Text::plain("hi".to_string())).pad(false);
        let a = render_aligned(&padded, 10);
        let b = render_aligned(&unpadded, 10);
        assert_eq!(a[0], "hi        ");
        assert_eq!(b[0], "hi");
    }
}
