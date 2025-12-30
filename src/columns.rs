//! Multi-column layout for displaying content in columns.
//!
//! Similar to Rich's Columns for displaying lists in columns.

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Span, Text};
use unicode_width::UnicodeWidthStr;

/// Column layout mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnMode {
    /// Equal width columns
    #[default]
    Equal,
    /// Optimal width based on content
    Fit,
}

/// A multi-column layout container.
#[derive(Debug, Clone)]
pub struct Columns {
    /// Items to display
    items: Vec<Text>,
    /// Number of columns (0 = auto)
    num_columns: usize,
    /// Column mode
    mode: ColumnMode,
    /// Gap between columns
    gap: usize,
    /// Expand to full width
    expand: bool,
    /// Style for items
    style: Style,
}

impl Columns {
    /// Create a new Columns layout from items.
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        Columns {
            items: items.into_iter().map(Into::into).collect(),
            num_columns: 0,
            mode: ColumnMode::Equal,
            gap: 2,
            expand: true,
            style: Style::new(),
        }
    }

    /// Set the number of columns (0 = auto).
    pub fn num_columns(mut self, n: usize) -> Self {
        self.num_columns = n;
        self
    }

    /// Set the column mode.
    pub fn mode(mut self, mode: ColumnMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the gap between columns.
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }

    /// Set whether to expand to full width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Set the item style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Calculate the number of columns based on content and width.
    fn calculate_columns(&self, width: usize) -> usize {
        if self.num_columns > 0 {
            return self.num_columns;
        }

        if self.items.is_empty() {
            return 1;
        }

        // Find max item width
        let max_item_width = self
            .items
            .iter()
            .map(|item| item.width())
            .max()
            .unwrap_or(1);

        // Calculate how many columns fit
        let min_col_width = max_item_width + self.gap;
        let cols = (width + self.gap) / min_col_width.max(1);

        cols.max(1).min(self.items.len())
    }
}

impl Renderable for Columns {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if self.items.is_empty() {
            return vec![];
        }

        let num_cols = self.calculate_columns(context.width);
        let num_rows = self.items.len().div_ceil(num_cols);

        // Calculate column widths
        let total_gap = self.gap * (num_cols.saturating_sub(1));
        let available = context.width.saturating_sub(total_gap);
        let col_width = available / num_cols.max(1);

        let mut segments = Vec::new();

        for row_idx in 0..num_rows {
            let mut row_spans = Vec::new();

            for col_idx in 0..num_cols {
                let item_idx = row_idx * num_cols + col_idx;

                if col_idx > 0 {
                    // Add gap
                    row_spans.push(Span::raw(" ".repeat(self.gap)));
                }

                if item_idx < self.items.len() {
                    let item = &self.items[item_idx];
                    let content = item.plain_text();
                    let content_width = UnicodeWidthStr::width(content.as_str());

                    // Truncate or pad to column width
                    let displayed = if content_width > col_width {
                        truncate_to_width(&content, col_width)
                    } else {
                        let padding = col_width - content_width;
                        format!("{}{}", content, " ".repeat(padding))
                    };

                    row_spans.push(Span::styled(displayed, self.style));
                } else {
                    // Empty cell
                    row_spans.push(Span::raw(" ".repeat(col_width)));
                }
            }

            segments.push(Segment::line(row_spans));
        }

        segments
    }
}

fn truncate_to_width(s: &str, width: usize) -> String {
    use unicode_segmentation::UnicodeSegmentation;

    let mut result = String::new();
    let mut current_width = 0;

    for grapheme in s.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if current_width + grapheme_width > width {
            break;
        }
        result.push_str(grapheme);
        current_width += grapheme_width;
    }

    // Pad to width
    while current_width < width {
        result.push(' ');
        current_width += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_basic() {
        let items = vec!["a", "b", "c", "d", "e", "f"];
        let columns = Columns::new(items).num_columns(3);

        let context = RenderContext { width: 30 };
        let segments = columns.render(&context);

        // Should have 2 rows (6 items / 3 cols)
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_columns_auto() {
        let items = vec!["short", "items", "here"];
        let columns = Columns::new(items);

        let context = RenderContext { width: 40 };
        let segments = columns.render(&context);

        // Should auto-calculate columns
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_columns_empty() {
        let columns = Columns::new(Vec::<&str>::new());
        let context = RenderContext { width: 40 };
        let segments = columns.render(&context);
        assert!(segments.is_empty());
    }
}
