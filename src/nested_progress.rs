//! Nested progress trees.
//!
//! [`NestedProgress`] models a task hierarchy with completion tracking and
//! renders as an indented tree of progress bars, one line per node.

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::Span;

/// The width of the bar drawn for each node.
const BAR_WIDTH: usize = 20;

/// A nested progress structure: a task with optional children.
///
/// Rendering produces one line per node, indented by depth, showing the
/// description, a bar, and the completion percentage.
#[derive(Debug, Clone)]
pub struct NestedProgress {
    description: String,
    total: u64,
    current: u64,
    children: Vec<NestedProgress>,
}

impl NestedProgress {
    /// Create a new nested progress.
    pub fn new(description: impl Into<String>, total: u64) -> Self {
        NestedProgress {
            description: description.into(),
            total,
            current: 0,
            children: Vec::new(),
        }
    }

    /// Add a child progress.
    pub fn add_child(&mut self, description: impl Into<String>, total: u64) -> &mut NestedProgress {
        self.children.push(NestedProgress::new(description, total));
        self.children.last_mut().expect("just pushed")
    }

    /// Update progress.
    pub fn update(&mut self, amount: u64) {
        self.current = self.current.saturating_add(amount).min(self.total);
    }

    /// Get completion percentage (0-100). A task with zero total is
    /// considered complete.
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }

    /// Get the number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Render this node (and descendants) into segments.
    fn render_node(&self, segments: &mut Vec<Segment>, depth: usize) {
        let indent = "  ".repeat(depth);
        let percent = self.percent();
        let filled = ((BAR_WIDTH as f64 * percent / 100.0).round() as usize).min(BAR_WIDTH);
        let empty = BAR_WIDTH - filled;

        let mut spans = Vec::new();
        spans.push(Span::styled(
            format!("{indent}{}", self.description),
            Style::new(),
        ));
        spans.push(Span::raw(" "));
        if filled > 0 {
            spans.push(Span::styled(
                "━".repeat(filled),
                Style::new().foreground(Color::Green),
            ));
        }
        if empty > 0 {
            spans.push(Span::styled(
                "─".repeat(empty),
                Style::new().foreground(Color::BrightBlack),
            ));
        }
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            format!("{percent:>3.0}%"),
            Style::new().foreground(Color::Cyan),
        ));

        segments.push(Segment::line(spans));

        for child in &self.children {
            child.render_node(segments, depth + 1);
        }
    }
}

impl Renderable for NestedProgress {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        let mut segments = Vec::new();
        self.render_node(&mut segments, 0);
        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::Console;

    #[test]
    fn test_nested_progress_creation() {
        let nested = NestedProgress::new("Root task", 100);
        assert_eq!(nested.child_count(), 0);
    }

    #[test]
    fn test_add_child() {
        let mut nested = NestedProgress::new("Parent", 100);
        nested.add_child("Child task", 50);
        assert_eq!(nested.child_count(), 1);
    }

    #[test]
    fn test_update() {
        let mut nested = NestedProgress::new("Task", 100);
        nested.update(50);
        assert_eq!(nested.percent(), 50.0);
    }

    #[test]
    fn test_update_clamps_to_total() {
        let mut nested = NestedProgress::new("Task", 10);
        nested.update(999);
        assert_eq!(nested.percent(), 100.0);
    }

    #[test]
    fn test_render_one_line_per_node() {
        let mut root = NestedProgress::new("Root", 100);
        root.add_child("A", 10);
        {
            let b = root.add_child("B", 10);
            b.add_child("B1", 5);
        }
        root.update(40);

        let context = RenderContext {
            width: 80,
            height: None,
        };
        let segments = root.render(&context);
        assert_eq!(segments.len(), 4, "one line per node");

        let plain: Vec<String> = segments.iter().map(|s| s.plain_text()).collect();
        assert!(plain[0].starts_with("Root"));
        assert!(plain[1].starts_with("  A"));
        assert!(plain[3].starts_with("    B1"), "grandchild doubly indented");
        assert!(plain[0].contains("40%"));
    }

    #[test]
    fn test_renderable_via_console() {
        let console = Console::capture();
        let mut root = NestedProgress::new("Build", 100);
        root.add_child("compile", 50);
        console.print_renderable(&root);
        let out = console.get_captured_output();
        assert!(out.contains("Build"));
        assert!(out.contains("compile"));
    }
}
