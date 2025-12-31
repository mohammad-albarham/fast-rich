//! Render groups for combining multiple renderables.
//!
//! Groups renderables vertically with optional spacing and dividers.

use crate::console::RenderContext;
use crate::renderable::{BoxedRenderable, Renderable, Segment};
use crate::rule::Rule;
use crate::text::Span;

/// Fit strategy for renderables in a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fit {
    /// Each renderable uses the full width
    Fill,
    /// Each renderable uses minimum width
    Shrink,
}

/// A group of renderables rendered vertically.
pub struct RenderGroup {
    renderables: Vec<BoxedRenderable>,
    spacing: usize,
    fit: Fit,
    divider: Option<String>,
}

impl RenderGroup {
    /// Create a new empty render group.
    pub fn new() -> Self {
        RenderGroup {
            renderables: Vec::new(),
            spacing: 0,
            fit: Fit::Fill,
            divider: None,
        }
    }

    /// Add a renderable to the group.
    pub fn add(&mut self, renderable: impl Renderable + Send + Sync + 'static) -> &mut Self {
        self.renderables.push(Box::new(renderable));
        self
    }

    /// Set the spacing between renderables (in lines).
    pub fn spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the fit strategy.
    pub fn fit(mut self, fit: Fit) -> Self {
        self.fit = fit;
        self
    }

    /// Add a divider between renderables.
    pub fn divider(mut self, divider: impl Into<String>) -> Self {
        self.divider = Some(divider.into());
        self
    }

    /// Create a group from a vector of renderables.
    pub fn from_renderables(renderables: Vec<BoxedRenderable>) -> Self {
        RenderGroup {
            renderables,
            spacing: 0,
            fit: Fit::Fill,
            divider: None,
        }
    }
}

impl Default for RenderGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for RenderGroup {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let mut result = Vec::new();

        for (i, renderable) in self.renderables.iter().enumerate() {
            // Render the item
            let item_context = match self.fit {
                Fit::Fill => context.clone(),
                Fit::Shrink => RenderContext {
                    width: renderable.min_width().min(context.width),
                },
            };

            let segments = renderable.render(&item_context);
            result.extend(segments);

            // Add spacing or divider between items (but not after last)
            if i < self.renderables.len() - 1 {
                // Add divider if set
                if let Some(ref divider_text) = self.divider {
                    let rule = Rule::new(divider_text);
                    let divider_segments = rule.render(context);
                    result.extend(divider_segments);
                }

                // Add spacing
                for _ in 0..self.spacing {
                    result.push(Segment::line(vec![Span::raw(" ".repeat(context.width))]));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Text;

    #[test]
    fn test_render_group_creation() {
        let group = RenderGroup::new();
        assert_eq!(group.renderables.len(), 0);
    }

    #[test]
    fn test_add_renderables() {
        let mut group = RenderGroup::new();
        group.add(Text::plain("First"));
        group.add(Text::plain("Second"));

        assert_eq!(group.renderables.len(), 2);
    }

    #[test]
    fn test_spacing() {
        let group = RenderGroup::new().spacing(2);
        assert_eq!(group.spacing, 2);
    }

    #[test]
    fn test_fit() {
        let group = RenderGroup::new().fit(Fit::Shrink);
        assert_eq!(group.fit, Fit::Shrink);
    }

    #[test]
    fn test_divider() {
        let group = RenderGroup::new().divider("---");
        assert!(group.divider.is_some());
    }
}
