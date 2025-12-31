use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use std::sync::Arc;

/// A node in the layout tree for creating splits and grids.
#[derive(Clone)]
pub struct Layout {
    /// Renderable content (optional, for leaf nodes).
    renderable: Option<Arc<dyn Renderable + Send + Sync>>,
    /// Child layouts.
    children: Vec<Layout>,
    /// Split direction.
    direction: Direction,
    /// Fixed size (width or height depending on parent direction).
    size: Option<u16>,
    /// Ratio for flexible sizing.
    ratio: u32,
    /// Name for debugging.
    name: Option<String>,
    /// Minimum size.
    #[allow(dead_code)]
    minimum_size: u16,
    /// Is this layout visible?
    visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Layout {
    /// Create a new empty layout.
    pub fn new() -> Self {
        Self {
            renderable: None,
            children: Vec::new(),
            direction: Direction::Vertical,
            size: None,
            ratio: 1,
            name: None,
            minimum_size: 0,
            visible: true,
        }
    }

    /// Set the name of the layout (useful for debugging).
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set a fixed size for this layout.
    pub fn with_size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Set a ratio for this layout (default is 1).
    pub fn with_ratio(mut self, ratio: u32) -> Self {
        self.ratio = ratio;
        self
    }

    /// Set the renderable content.
    pub fn update<R: Renderable + Send + Sync + 'static>(&mut self, renderable: R) {
        self.renderable = Some(Arc::new(renderable));
    }

    /// Get mutable access to children.
    pub fn children_mut(&mut self) -> &mut Vec<Layout> {
        &mut self.children
    }

    /// Split the layout horizontally (into columns).
    pub fn split_row(&mut self, layouts: Vec<Layout>) {
        self.direction = Direction::Horizontal;
        self.children = layouts;
    }

    /// Split the layout vertically (into rows).
    pub fn split_column(&mut self, layouts: Vec<Layout>) {
        self.direction = Direction::Vertical;
        self.children = layouts;
    }

    /// Calculate split sizes for a given total space.
    fn calculate_splits(&self, total_size: u16) -> Vec<u16> {
        let count = self.children.len();
        if count == 0 {
            return Vec::new();
        }

        let mut sizes = vec![0; count];
        let mut remaining = total_size;
        let mut total_ratio = 0;
        let mut flexible_indices = Vec::new();

        // 1. Assign fixed sizes & min sizes
        for (i, child) in self.children.iter().enumerate() {
            if let Some(fixed) = child.size {
                let s = std::cmp::min(fixed, remaining);
                sizes[i] = s;
                remaining -= s;
            } else {
                total_ratio += child.ratio;
                flexible_indices.push(i);
            }
        }

        // 2. Distribute remaining space by ratio
        if !flexible_indices.is_empty() && total_ratio > 0 {
            let unit = remaining as f32 / total_ratio as f32;
            let mut distributed = 0;

            for (idx, &i) in flexible_indices.iter().enumerate() {
                let child = &self.children[i];
                // For the last item, give the rest to avoid rounding errors
                let s = if idx == flexible_indices.len() - 1 {
                    remaining - distributed
                } else {
                    (child.ratio as f32 * unit).round() as u16
                };
                sizes[i] = s;
                distributed += s;
            }
        }

        sizes
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Layout {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if !self.visible {
            return Vec::new();
        }

        // Leaf node: Render content
        if self.children.is_empty() {
            if let Some(r) = &self.renderable {
                // TODO: Optimization - Pass specific constraint context based on parent split
                // For now, we render with full context width, but we might want to clip/shape it using new context
                return r.render(context);
            }
            // Empty placeholder
            let blank_line = " ".repeat(context.width);
            return vec![Segment::new(vec![crate::text::Span::raw(blank_line)])];
        }

        // Branch node: Calculate splits
        let (width, _height) = match self.direction {
            Direction::Horizontal => (context.width as u16, 0), // Width split
            Direction::Vertical => (0, 100), // Height split - TODO: Detect height from context or use arbitrary
        };

        // Note: Vertical split (rows) depends on available height which RenderContext doesn't explicitly track yet
        // in a constrained way (it mostly track width).
        // For CLI output, vertical stacking is just rendering one after another.
        // Horizontal split requires column logic.

        let mut segments = Vec::new();

        if self.direction == Direction::Vertical {
            // Stack children vertically
            for child in &self.children {
                let child_segments = child.render(context);
                segments.extend(child_segments);
                // If child doesn't fill width or needed spacing?
            }
        } else {
            // Horizontal split (Columns)
            // This is complex without a buffer. We need to render side-by-side.
            // Simplified approach: Render all children with reduced width context, then zip lines.

            let splits = self.calculate_splits(width);
            let mut columns_output: Vec<Vec<Segment>> = Vec::new();
            let mut max_lines = 0;

            for (i, child) in self.children.iter().enumerate() {
                let w = splits[i] as usize;
                if w == 0 {
                    columns_output.push(Vec::new());
                    continue;
                }

                let child_ctx = RenderContext { width: w };
                let child_segs = child.render(&child_ctx);
                max_lines = std::cmp::max(max_lines, child_segs.len());
                columns_output.push(child_segs);
            }

            // Zip lines
            for line_idx in 0..max_lines {
                let mut line_spans = Vec::new();
                for (col_idx, _child) in self.children.iter().enumerate() {
                    let w = splits[col_idx] as usize;
                    let segs = &columns_output[col_idx];

                    if line_idx < segs.len() {
                        line_spans.extend(segs[line_idx].spans.clone());
                    } else {
                        // Padding for shorter columns
                        line_spans.push(crate::text::Span::raw(" ".repeat(w)));
                    }
                }
                segments.push(Segment::line(line_spans));
            }
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_splits_ratios() {
        // Equal split
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_ratio(1),
            Layout::new().with_ratio(1),
        ]);
        let splits = layout.calculate_splits(100);
        assert_eq!(splits, vec![50, 50]);

        // 1:3 split
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_ratio(1),
            Layout::new().with_ratio(3),
        ]);
        let splits = layout.calculate_splits(100);
        assert_eq!(splits, vec![25, 75]);
    }

    #[test]
    fn test_calculate_splits_fixed() {
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_size(10),
            Layout::new().with_size(20),
        ]);
        let splits = layout.calculate_splits(100);
        // If NO ratio items, implementation should just give fixed?
        // Wait, if NO ratio items, `flexible_indices` is empty, so loop 2 doesn't run.
        // So expected is [10, 20]. (Correct)
        assert_eq!(splits[0], 10);
        assert_eq!(splits[1], 20);
    }

    #[test]
    fn test_calculate_splits_mixed() {
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_size(10), // Fixed 10
            Layout::new().with_ratio(1), // Takes half of remaining (90/2 = 45)
            Layout::new().with_ratio(1), // Takes other half (45)
        ]);
        let splits = layout.calculate_splits(100);
        assert_eq!(splits, vec![10, 45, 45]);
    }

    #[test]
    fn test_calculate_splits_rounding() {
        // 100 / 3 = 33.333
        // Should be 33, 33, 34
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_ratio(1),
            Layout::new().with_ratio(1),
            Layout::new().with_ratio(1),
        ]);
        let splits = layout.calculate_splits(100);
        assert_eq!(splits, vec![33, 33, 34]);
        assert_eq!(splits.iter().sum::<u16>(), 100);
    }
}
