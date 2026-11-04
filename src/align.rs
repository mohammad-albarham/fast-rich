//! Alignment wrapper for renderables.

use crate::console::RenderContext;
use crate::renderable::{BoxedRenderable, Renderable, Segment};
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
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let segments = self.child.render(context);
        let width = context.width;
        let mut aligned_segments = Vec::with_capacity(segments.len());

        for segment in segments {
            let mut line = segment.spans.clone();
            let line_width: usize = line.iter().map(|s| s.width()).sum();

            if line_width < width {
                let padding = width - line_width;
                match self.align {
                    Alignment::Left => {
                        if self.pad {
                            line.push(Span::raw(" ".repeat(padding)));
                        }
                    }
                    Alignment::Right => {
                        let mut new_line = vec![Span::raw(" ".repeat(padding))];
                        new_line.extend(line);
                        line = new_line;
                    }
                    Alignment::Center => {
                        let left_pad = padding / 2;
                        let right_pad = padding - left_pad;
                        let mut new_line = vec![Span::raw(" ".repeat(left_pad))];
                        new_line.extend(line);
                        if self.pad {
                            new_line.push(Span::raw(" ".repeat(right_pad)));
                        }
                        line = new_line;
                    }
                }
            }

            // Reconstruct segment
            if segment.newline {
                aligned_segments.push(Segment::line(line));
            } else {
                aligned_segments.push(Segment::new(line));
            }
        }

        // Handle vertical alignment if height is set (TODO: Context height?)
        // For now only if explicit height is set.
        if let Some(target_height) = self.height {
            let current_height = aligned_segments.len();
            if current_height < target_height {
                let diff = target_height - current_height;
                match self.vertical {
                    VerticalAlignment::Top => {
                        // Add empty lines at bottom
                        for _ in 0..diff {
                            aligned_segments
                                .push(Segment::line(vec![Span::raw(" ".repeat(width))]));
                        }
                    }
                    VerticalAlignment::Bottom => {
                        let mut new_segments = Vec::with_capacity(target_height);
                        for _ in 0..diff {
                            new_segments.push(Segment::line(vec![Span::raw(" ".repeat(width))]));
                        }
                        new_segments.extend(aligned_segments);
                        aligned_segments = new_segments;
                    }
                    VerticalAlignment::Middle => {
                        let top_pad = diff / 2;
                        let bottom_pad = diff - top_pad;
                        let mut new_segments = Vec::with_capacity(target_height);
                        for _ in 0..top_pad {
                            new_segments.push(Segment::line(vec![Span::raw(" ".repeat(width))]));
                        }
                        new_segments.extend(aligned_segments);
                        for _ in 0..bottom_pad {
                            new_segments.push(Segment::line(vec![Span::raw(" ".repeat(width))]));
                        }
                        aligned_segments = new_segments;
                    }
                }
            }
        }

        aligned_segments
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
