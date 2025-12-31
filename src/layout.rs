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

    /// Set a minimum size for this layout.
    pub fn with_minimum_size(mut self, size: u16) -> Self {
        self.minimum_size = size;
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
        let mut flexible_indices = Vec::new();

        // 1. Assign fixed sizes
        for (i, child) in self.children.iter().enumerate() {
            if let Some(fixed) = child.size {
                let s = std::cmp::min(fixed, remaining);
                sizes[i] = s;
                remaining -= s;
            } else {
                flexible_indices.push(i);
            }
        }

        // 2. Resolve flexible sizes
        let mut candidates = flexible_indices;

        while !candidates.is_empty() {
            let total_ratio: u32 = candidates.iter().map(|&i| self.children[i].ratio).sum();

            // If remaining is 0 or no ratio, fill rest with 0
            if remaining == 0 || total_ratio == 0 {
                for &i in &candidates {
                    sizes[i] = 0;
                }
                break;
            }

            let unit = remaining as f64 / total_ratio as f64;

            // Find if any candidate needs to be fixed to min_size
            let mut violator = None;
            for (idx_in_candidates, &i) in candidates.iter().enumerate() {
                let child = &self.children[i];
                let ideal = child.ratio as f64 * unit;
                if ideal < child.minimum_size as f64 {
                    violator = Some(idx_in_candidates);
                    break; // Fix one at a time
                }
            }

            if let Some(idx_c) = violator {
                let i = candidates.remove(idx_c);
                let child = &self.children[i];
                let s = std::cmp::min(child.minimum_size, remaining);
                sizes[i] = s;
                remaining -= s;
            } else {
                // No violators, distribute rest
                let mut distributed = 0;
                for (idx, &i) in candidates.iter().enumerate() {
                    let child = &self.children[i];
                    let s = if idx == candidates.len() - 1 {
                        remaining - distributed
                    } else {
                        (child.ratio as f64 * unit).round() as u16
                    };
                    sizes[i] = s;
                    distributed += s;
                }
                break;
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
            Direction::Horizontal => (context.width as u16, context.height.unwrap_or(0) as u16),
            Direction::Vertical => (
                context.width as u16,
                context.height.unwrap_or(0) as u16,
            ),
        };

        let mut segments = Vec::new();

        if self.direction == Direction::Vertical {
            if let Some(total_height) = context.height {
                // Fixed height: Calculate splits based on height
                let splits = self.calculate_splits(total_height as u16);
                
                for (i, child) in self.children.iter().enumerate() {
                    let h = splits[i] as usize;
                    if h == 0 {
                        continue;
                    }

                    // Render child with constrained height
                    let child_ctx = RenderContext {
                        width: context.width,
                        height: Some(h),
                    };
                    let child_segments = child.render(&child_ctx);

                    // Ensure we output exactly `h` lines
                    // 1. Take up to `h` lines from render output
                    let mut count = 0;
                    for segment in child_segments {
                        if count < h {
                            segments.push(segment);
                            count += 1;
                        } else {
                            break;
                        }
                    }

                    // 2. Pad with empty lines if short
                    if count < h {
                        let blank_line = " ".repeat(context.width);
                        for _ in count..h {
                            segments.push(Segment::new(vec![crate::text::Span::raw(blank_line.clone())]));
                        }
                    }
                }
            } else {
                // Unconstrained height: Stack children vertically (Flow layout)
                for child in &self.children {
                    let child_segments = child.render(context);
                    segments.extend(child_segments);
                }
            }
        } else {
            // Horizontal split (Columns)
            let splits = self.calculate_splits(width);
            let mut columns_output: Vec<Vec<Segment>> = Vec::new();
            let mut max_lines = 0;

            // If we have a fixed height, we expect all columns to be that height (or padded to it)
            // If explicit height is None, we determine max height from content.
            let target_height = context.height;

            for (i, child) in self.children.iter().enumerate() {
                let w = splits[i] as usize;
                if w == 0 {
                    columns_output.push(Vec::new());
                    continue;
                }

                // Pass through the parent's height constraint to children
                let child_ctx = RenderContext { 
                    width: w, 
                    height: target_height,
                };
                let child_segs = child.render(&child_ctx);
                max_lines = std::cmp::max(max_lines, child_segs.len());
                columns_output.push(child_segs);
            }

            // If we have a target height, use it as the number of lines to output
            // (Assuming children respected it, max_lines should match target_height, 
            // but we use max_lines if target_height is None, or target_height if Some)
             let final_lines = target_height.unwrap_or(max_lines);

            // Zip lines
            for line_idx in 0..final_lines {
                let mut line_spans = Vec::new();
                for (col_idx, _child) in self.children.iter().enumerate() {
                    let w = splits[col_idx] as usize;
                    let segs = &columns_output[col_idx];

                    if line_idx < segs.len() {
                        line_spans.extend(segs[line_idx].spans.clone());
                    } else {
                        // Empty space for this column
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
    #[test]
    fn test_calculate_splits_min_size_simple() {
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_ratio(1).with_minimum_size(60),
            Layout::new().with_ratio(1),
        ]);
        let splits = layout.calculate_splits(100);
        assert_eq!(splits, vec![60, 40]);
    }

    #[test]
    fn test_calculate_splits_min_size_priority() {
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_ratio(1).with_minimum_size(80),
            Layout::new().with_ratio(1).with_minimum_size(10),
        ]);
        let splits = layout.calculate_splits(100);
        assert_eq!(splits, vec![80, 20]);
    }

    #[test]
    fn test_calculate_splits_complex_min() {
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_size(5),
            Layout::new().with_ratio(1).with_minimum_size(10),
            Layout::new().with_ratio(1),
        ]);
        let splits = layout.calculate_splits(20);
        assert_eq!(splits, vec![5, 10, 5]);
    }

    #[test]
    fn test_vertical_split_ratios() {
        let mut layout = Layout::new();
        layout.split_column(vec![
            Layout::new().with_ratio(1).with_name("Top"),
            Layout::new().with_ratio(1).with_name("Bottom"),
        ]);

        // Mock context with height
        let context = RenderContext { width: 80, height: Some(10) };
        let segments = layout.render(&context);

        // Should have 10 lines total
        assert_eq!(segments.len(), 10);
        // 80 chars wide - check first line
        if !segments.is_empty() {
             assert_eq!(segments[0].plain_text().len(), 80);
        }
    }

    #[test]
    fn test_vertical_split_stacking() {
        let mut layout = Layout::new();
        layout.split_column(vec![
            Layout::new().with_size(1).with_name("Top"),
            Layout::new().with_name("Bottom"),
        ]);

        // Unconstrained height
        let context = RenderContext { width: 80, height: None };
        let segments = layout.render(&context);

        // Each leaf layout renders 1 blank line by default if empty
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_horizontal_split_propagates_height() {
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_ratio(1),
            Layout::new().with_ratio(1),
        ]);

        // If we pass a height, it should be enforced on children (columns)
        let context = RenderContext { width: 80, height: Some(5) };
        let segments = layout.render(&context);

        // Should have 5 lines
        assert_eq!(segments.len(), 5);
    }
}
