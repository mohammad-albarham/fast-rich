use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::text::Span;
use std::sync::Arc;

/// A node in the layout tree.
#[derive(Clone)]
pub struct Layout {
    /// Renderable content (optional).
    renderable: Option<Arc<dyn Renderable + Send + Sync>>,
    /// Child layouts (if this is a split container).
    children: Vec<Layout>,
    /// Direction of children.
    direction: Direction,
    /// Fixed size (width or height depend on parent direction).
    #[allow(dead_code)]
    size: Option<usize>,
    /// Ratio of remaining space.
    #[allow(dead_code)]
    ratio: u32,
    /// Name of this layout section.
    #[allow(dead_code)]
    name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Layout {
    /// Create a new layout with optional content.
    pub fn new() -> Self {
        Self {
            renderable: None,
            children: Vec::new(),
            direction: Direction::Vertical,
            size: None,
            ratio: 1,
            name: None,
        }
    }

    /// Set the renderable content for this layout part.
    pub fn update<R: Renderable + Send + Sync + 'static>(&mut self, renderable: R) {
        self.renderable = Some(Arc::new(renderable));
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

    /// Set a name for debugging/lookup.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Layout {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if self.children.is_empty() {
            // Leaf node: Render content
            if let Some(r) = &self.renderable {
                return r.render(context);
            }
            // Empty box if no content
            return vec![Segment::new(vec![Span::raw(" ".repeat(context.width))])];
        }

        // Branch node: Render children based on direction
        let mut result = Vec::new();

        if self.direction == Direction::Vertical {
            // Split height: Simple vertical stacking
            for child in &self.children {
                let segments = child.render(context);
                result.extend(segments);
                result.push(Segment::empty_line());
            }
        } else {
            // Split width (Horizontal)
            // Simple delegation to existing Columns-like logic would be complex here without proper 2D buffer.
            // For MVP, we just stack vertically or render children sequentially.
            // TODO: Proper columns implementation for Layout using 'Columns' logic.
            for child in &self.children {
                result.extend(child.render(context));
            }
        }

        result
    }
}
