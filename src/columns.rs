//! Multi-column layout for displaying content in columns.
//!
//! Similar to Rich's Columns for displaying lists in columns.

use crate::align::Align;
use crate::console::RenderContext;
use crate::renderable::{fit_line, render_lines, Renderable, Segment, SharedRenderable};
use crate::style::Style;
use crate::text::{Alignment, Span, Text};
use std::sync::Arc;

/// Column layout mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnMode {
    /// Equal width columns
    #[default]
    Equal,
    /// Optimal width based on content
    Fit,
}

/// A multi-column layout container (Rich's `Columns`).
///
/// Items are laid out in as many columns as fit the width, each column as
/// wide as its widest item, separated by `gap` spaces.
#[derive(Clone)]
pub struct Columns {
    items: Vec<SharedRenderable>,
    num_columns: usize,
    mode: ColumnMode,
    gap: usize,
    expand: bool,
    style: Style,
    column_first: bool,
    right_to_left: bool,
    align: Option<Alignment>,
    width: Option<usize>,
}

impl std::fmt::Debug for Columns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Columns")
            .field("items", &self.items.len())
            .field("num_columns", &self.num_columns)
            .field("mode", &self.mode)
            .field("gap", &self.gap)
            .field("expand", &self.expand)
            .finish_non_exhaustive()
    }
}

impl Columns {
    /// Create columns from text items.
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        Self::renderables(
            items
                .into_iter()
                .map(|t| Arc::new(t.into()) as SharedRenderable),
        )
    }

    /// Create columns from any renderables.
    pub fn renderables<I>(items: I) -> Self
    where
        I: IntoIterator<Item = SharedRenderable>,
    {
        Columns {
            items: items.into_iter().collect(),
            num_columns: 0,
            mode: ColumnMode::Fit,
            gap: 1,
            expand: false,
            style: Style::new(),
            column_first: false,
            right_to_left: false,
            align: None,
            width: None,
        }
    }

    /// Force a number of columns (0 = as many as fit).
    pub fn num_columns(mut self, n: usize) -> Self {
        self.num_columns = n;
        self
    }

    /// `Equal` gives every column the widest item's width (Rich's
    /// `equal=True`); `Fit` sizes each column to its own items.
    pub fn mode(mut self, mode: ColumnMode) -> Self {
        self.mode = mode;
        self
    }

    /// Spaces between columns (default 1, like Rich's `padding=(0, 1)`).
    pub fn gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }

    /// Spread the columns over the full width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Base style applied to every item.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Fill columns top-to-bottom instead of row by row.
    pub fn column_first(mut self, column_first: bool) -> Self {
        self.column_first = column_first;
        self
    }

    /// Lay out each row right to left.
    pub fn right_to_left(mut self, right_to_left: bool) -> Self {
        self.right_to_left = right_to_left;
        self
    }

    /// Align each item within its column.
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = Some(align);
        self
    }

    /// Fixed width for every column.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    fn item_widths(&self, max_width: usize) -> Vec<usize> {
        self.items
            .iter()
            .map(|r| r.max_width().min(max_width))
            .collect()
    }

    /// Widths used to choose the column count: with `Equal`, every item
    /// counts as wide as the widest (Rich's `equal=True`).
    fn count_widths(&self, widths: Vec<usize>) -> Vec<usize> {
        if self.mode == ColumnMode::Equal {
            let widest = widths.iter().copied().max().unwrap_or(0);
            vec![widest; widths.len()]
        } else {
            widths
        }
    }

    /// Item order for `column_count` columns (`None` = empty filler cell).
    fn order(&self, column_count: usize) -> Vec<Option<usize>> {
        let n = self.items.len();
        let mut order: Vec<Option<usize>> = if self.column_first {
            let mut lengths = vec![n / column_count; column_count];
            for len in lengths.iter_mut().take(n % column_count) {
                *len += 1;
            }
            let rows = n.div_ceil(column_count);
            let mut cells = vec![vec![None; column_count]; rows];
            let (mut row, mut col) = (0, 0);
            for index in 0..n {
                cells[row][col] = Some(index);
                lengths[col] -= 1;
                if lengths[col] > 0 {
                    row += 1;
                } else {
                    col += 1;
                    row = 0;
                }
            }
            cells
                .into_iter()
                .flatten()
                .take_while(Option::is_some)
                .collect()
        } else {
            (0..n).map(Some).collect()
        };
        if n % column_count != 0 {
            order.extend((0..column_count - n % column_count).map(|_| None));
        }
        order
    }

    /// Rich's column-count search: start with one column per item and drop
    /// columns until the widest items of each column fit.
    fn column_count(&self, widths: &[usize], max_width: usize) -> usize {
        if self.num_columns > 0 {
            return self.num_columns;
        }
        if let Some(w) = self.width {
            return (max_width / (w + self.gap).max(1)).max(1);
        }
        let mut count = self.items.len();
        'search: while count > 1 {
            let mut col_widths: Vec<usize> = Vec::new();
            for (i, index) in self.order(count).into_iter().enumerate() {
                let col = i % count;
                let w = index.map_or(0, |ix| widths[ix]);
                if col_widths.len() <= col {
                    col_widths.push(w);
                } else {
                    col_widths[col] = col_widths[col].max(w);
                }
                let total: usize =
                    col_widths.iter().sum::<usize>() + self.gap * (col_widths.len() - 1);
                if total > max_width {
                    count = col_widths.len() - 1;
                    continue 'search;
                }
            }
            break;
        }
        count.max(1)
    }
}

impl Renderable for Columns {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if self.items.is_empty() {
            return vec![];
        }
        let max_width = context.width;
        let widths = self.item_widths(max_width);
        let count = self.column_count(&self.count_widths(widths.clone()), max_width);
        let order = self.order(count);

        // Column widths: fixed, or the widest item in each column.
        let mut col_widths = vec![0usize; count];
        for (i, index) in order.iter().enumerate() {
            let w = match (self.width, index) {
                (Some(w), _) => w,
                (None, Some(ix)) => widths[*ix],
                (None, None) => 0,
            };
            col_widths[i % count] = col_widths[i % count].max(w);
        }
        let gaps = self.gap * (count - 1);
        let mut total: usize = col_widths.iter().sum::<usize>() + gaps;
        if total > max_width {
            // Too wide even for one column per row: shrink evenly.
            let budget = max_width.saturating_sub(gaps);
            let per = budget / count.max(1);
            for w in &mut col_widths {
                *w = (*w).min(per.max(1));
            }
            total = col_widths.iter().sum::<usize>() + gaps;
        }
        if self.expand && total < max_width {
            // Rich spreads the spare width by each column's width including
            // its gap (the grid's right padding).
            let weights: Vec<usize> = col_widths
                .iter()
                .enumerate()
                .map(|(i, &w)| w + if i + 1 < count { self.gap } else { 0 })
                .collect();
            let pads = crate::table::ratio_distribute(max_width - total, &weights, None);
            for (w, p) in col_widths.iter_mut().zip(pads) {
                *w += p;
            }
        }

        let mut segments = Vec::new();
        for row in order.chunks(count) {
            let mut cells: Vec<(usize, Vec<Vec<Span>>)> = row
                .iter()
                .enumerate()
                .map(|(col, index)| {
                    let w = col_widths[col];
                    let lines = match index {
                        Some(ix) => {
                            let item = &self.items[*ix];
                            match self.align {
                                Some(a) => {
                                    let aligned = Align::new(item.clone(), a);
                                    render_lines(&aligned, w, Style::new())
                                }
                                None => render_lines(&**item, w, Style::new()),
                            }
                        }
                        None => Vec::new(),
                    };
                    (w, lines)
                })
                .collect();
            if self.right_to_left {
                cells.reverse();
            }
            let height = cells.iter().map(|(_, l)| l.len()).max().unwrap_or(0).max(1);
            for r in 0..height {
                let mut spans = Vec::new();
                for (i, (w, lines)) in cells.iter().enumerate() {
                    if i > 0 && self.gap > 0 {
                        spans.push(Span::raw(" ".repeat(self.gap)));
                    }
                    let line = lines.get(r).cloned().unwrap_or_default();
                    spans.extend(fit_line(line, *w, Style::new()).into_iter().map(|mut s| {
                        s.style = self.style.combine(&s.style);
                        s
                    }));
                }
                segments.push(Segment::line(spans));
            }
        }
        segments
    }

    fn min_width(&self) -> usize {
        self.items.iter().map(|r| r.min_width()).max().unwrap_or(0)
    }

    fn max_width(&self) -> usize {
        let total = self
            .items
            .iter()
            .map(|r| r.max_width())
            .fold(0usize, usize::saturating_add);
        total.saturating_add(self.gap * self.items.len().saturating_sub(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_basic() {
        let items = vec!["a", "b", "c", "d", "e", "f"];
        let columns = Columns::new(items).num_columns(3);

        let context = RenderContext {
            width: 30,
            height: None,
        };
        let segments = columns.render(&context);

        // Should have 2 rows (6 items / 3 cols)
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn test_columns_auto() {
        let items = vec!["short", "items", "here"];
        let columns = Columns::new(items);

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = columns.render(&context);

        // Should auto-calculate columns
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_columns_empty() {
        let columns = Columns::new(Vec::<&str>::new());
        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = columns.render(&context);
        assert!(segments.is_empty());
    }

    fn plain(r: &dyn Renderable, width: usize) -> String {
        crate::renderable::render_lines(r, width, Style::new())
            .iter()
            .map(|l| l.iter().map(|s| s.text.as_ref()).collect::<String>())
            .map(|l| l.trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_matches_rich_fit() {
        let c = Columns::new((0..10).map(|i| format!("item{i}")));
        assert_eq!(
            plain(&c, 30),
            "item0 item1 item2 item3 item4\nitem5 item6 item7 item8 item9"
        );
    }

    #[test]
    fn test_matches_rich_equal_column_first() {
        let c = Columns::new((0..7).map(|i| format!("x{}", "y".repeat(i))))
            .mode(ColumnMode::Equal)
            .column_first(true);
        assert_eq!(plain(&c, 20), "x    xyyyy\nxy   xyyyyy\nxyy  xyyyyyy\nxyyy");
    }

    #[test]
    fn test_matches_rich_expand() {
        let c = Columns::new(["a", "bb", "ccc"]).expand(true);
        assert_eq!(plain(&c, 20), "a    bb      ccc");
    }
}
