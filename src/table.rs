//! Tables for displaying structured data.
//!
//! Tables support headers, multiple columns with alignment and width control,
//! and various border styles.

use crate::box_drawing::Line;
use crate::console::RenderContext;
use crate::panel::BorderStyle;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Alignment, Overflow, Span, Text};
use unicode_width::UnicodeWidthStr;

/// Column alignment.
///
/// Deprecated alias of [`Alignment`](crate::text::Alignment) — use that
/// instead; the four alignment enums are being consolidated.
#[deprecated(since = "0.3.3", note = "use `fast_rich::text::Alignment` instead")]
pub type ColumnAlign = crate::text::Alignment;

/// Column width specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnWidth {
    /// Automatic width based on content
    #[default]
    Auto,
    /// Fixed width
    Fixed(usize),
    /// Min width
    Min(usize),
    /// Max width
    Max(usize),
}

/// A table column definition.
#[derive(Debug, Clone)]
pub struct Column {
    /// Column header
    pub header: String,
    /// Column alignment
    pub align: Alignment,
    /// Column width
    pub width: ColumnWidth,
    /// Header style
    pub header_style: Style,
    /// Cell style
    pub style: Style,
    /// Whether to wrap content
    pub wrap: bool,
    /// Minimum width (computed, reserved for future use)
    #[allow(dead_code)]
    min_width: usize,
    /// Maximum width (computed, reserved for future use)
    #[allow(dead_code)]
    max_width: usize,
}

impl Column {
    /// Create a new column with a header.
    pub fn new(header: &str) -> Self {
        let header_width = UnicodeWidthStr::width(header);
        Column {
            header: header.to_string(),
            align: Alignment::Left,
            width: ColumnWidth::Auto,
            header_style: Style::new().bold(),
            style: Style::new(),
            wrap: true,
            min_width: header_width,
            max_width: header_width,
        }
    }

    /// Set the column alignment.
    pub fn align(mut self, align: crate::text::Alignment) -> Self {
        self.align = align;
        self
    }

    /// Set the column width.
    pub fn width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    /// Set the header style.
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    /// Set the cell style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set whether to wrap content.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Center align shorthand.
    pub fn center(self) -> Self {
        self.align(Alignment::Center)
    }

    /// Right align shorthand.
    pub fn right(self) -> Self {
        self.align(Alignment::Right)
    }
}

/// A row of table cells.
#[derive(Debug, Clone)]
pub struct Row {
    cells: Vec<Text>,
    style: Option<Style>,
}

impl Row {
    /// Create a new row with cells.
    pub fn new<I, T>(cells: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        Row {
            cells: cells.into_iter().map(Into::into).collect(),
            style: None,
        }
    }

    /// Set a style for the entire row.
    ///
    /// The row style is the base style for all cells in the row; cell and
    /// column styles layer on top of it.
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// The row's base style, if any.
    pub fn base_style(&self) -> Option<Style> {
        self.style
    }
}

// Removed TableBorderChars struct and implementation

/// A table for displaying structured data.
#[derive(Debug, Clone)]
pub struct Table {
    /// Column definitions
    columns: Vec<Column>,
    /// Data rows
    rows: Vec<Row>,
    /// Border style
    border_style: BorderStyle,
    /// Border style (colors etc)
    style: Style,
    /// Show header row
    show_header: bool,
    /// Show border
    show_border: bool,
    /// Show row separators
    show_row_lines: bool,
    /// Padding in cells
    padding: usize,
    /// Title
    title: Option<String>,
    /// Expand to full width
    expand: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    /// Create a new empty table.
    pub fn new() -> Self {
        Table {
            columns: Vec::new(),
            rows: Vec::new(),
            border_style: BorderStyle::Rounded,
            style: Style::new(),
            show_header: true,
            show_border: true,
            show_row_lines: false,
            padding: 1,
            title: None,
            expand: false,
        }
    }

    /// Add a column to the table.
    pub fn add_column<C: Into<Column>>(&mut self, column: C) -> &mut Self {
        self.columns.push(column.into());
        self
    }

    /// Add a column by header name.
    pub fn column(mut self, header: &str) -> Self {
        self.columns.push(Column::new(header));
        self
    }

    /// Add multiple columns by header names.
    pub fn columns<I, S>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for header in headers {
            self.columns.push(Column::new(header.as_ref()));
        }
        self
    }

    /// Add a row to the table.
    pub fn add_row<I, T>(&mut self, cells: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        self.rows.push(Row::new(cells));
        self
    }

    /// Add a row from string slices (convenience method).
    pub fn add_row_strs(&mut self, cells: &[&str]) -> &mut Self {
        let text_cells: Vec<Text> = cells.iter().map(|s| Text::plain(s.to_string())).collect();
        self.rows.push(Row {
            cells: text_cells,
            style: None,
        });
        self
    }

    /// Add a Row object to the table.
    pub fn add_row_obj(&mut self, row: Row) -> &mut Self {
        self.rows.push(row);
        self
    }

    /// Set the border style.
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set the table style (border colors).
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the table title.
    #[deprecated(since = "0.3.3", note = "use `Table::title` instead")]
    pub fn set_title(self, title: &str) -> Self {
        self.title(title)
    }

    /// Set whether to show the header.
    pub fn show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Set whether to show the border.
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set whether to show row separator lines.
    pub fn show_row_lines(mut self, show: bool) -> Self {
        self.show_row_lines = show;
        self
    }

    /// Set cell padding.
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Set the table title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set whether to expand to full width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Calculate column widths based on content and per-column
    /// [`ColumnWidth`] settings.
    fn calculate_widths(&self, available_width: usize) -> Vec<usize> {
        let num_cols = self.columns.len();
        if num_cols == 0 {
            return vec![];
        }

        // Calculate content widths (header + widest cell line)
        let mut naturals: Vec<usize> = self
            .columns
            .iter()
            .map(|c| UnicodeWidthStr::width(c.header.as_str()))
            .collect();

        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i < naturals.len() {
                    // Measure the widest *line* of the cell, not the whole text
                    let widest_line = cell
                        .plain_text()
                        .lines()
                        .map(UnicodeWidthStr::width)
                        .max()
                        .unwrap_or(0);
                    naturals[i] = naturals[i].max(widest_line);
                }
            }
        }

        // Overlay explicit width requests
        let is_fixed: Vec<bool> = self
            .columns
            .iter()
            .map(|c| matches!(c.width, ColumnWidth::Fixed(_)))
            .collect();

        let mut effective = naturals.clone();
        for (i, col) in self.columns.iter().enumerate() {
            match col.width {
                ColumnWidth::Auto => {}
                ColumnWidth::Fixed(w) => effective[i] = w,
                ColumnWidth::Min(m) => effective[i] = effective[i].max(m),
                ColumnWidth::Max(m) => effective[i] = effective[i].min(m),
            }
        }

        // Calculate overhead (borders, padding)
        let overhead = if self.show_border {
            1 + num_cols + 1 + (self.padding * 2 * num_cols)
        } else {
            (num_cols - 1) + (self.padding * 2 * num_cols)
        };

        let content_width = available_width.saturating_sub(overhead);

        // Split columns into fixed and flexible buckets. Only `Fixed`
        // columns opt out of shrink/expand distribution; `Min`/`Max` adjust
        // the natural width but stay flexible.
        let mut widths = vec![0usize; num_cols];
        let mut fixed_total = 0usize;
        let mut flex_natural_total = 0usize;
        let mut flex_count = 0usize;
        for i in 0..num_cols {
            if is_fixed[i] {
                widths[i] = effective[i];
                fixed_total += widths[i];
            } else {
                widths[i] = effective[i];
                flex_natural_total += effective[i];
                flex_count += 1;
            }
        }

        let flex_budget = content_width.saturating_sub(fixed_total);

        if flex_natural_total == 0 {
            // Only fixed columns (or no content): give flexible columns equal
            // shares of what's left.
            if flex_count > 0 {
                let share = flex_budget.checked_div(flex_count).unwrap_or(0);
                let mut extra = flex_budget.checked_rem(flex_count).unwrap_or(0);
                for i in 0..num_cols {
                    if !is_fixed[i] {
                        widths[i] = share
                            + if extra > 0 {
                                extra -= 1;
                                1
                            } else {
                                0
                            };
                    }
                }
            }
            return widths;
        }

        if flex_natural_total <= flex_budget {
            // Everything fits
            if self.expand {
                let extra = flex_budget - flex_natural_total;
                if flex_count > 0 {
                    let per_col = extra.checked_div(flex_count).unwrap_or(0);
                    let mut remainder = extra.checked_rem(flex_count).unwrap_or(0);
                    for i in 0..num_cols {
                        if !is_fixed[i] {
                            widths[i] += per_col;
                            if remainder > 0 {
                                widths[i] += 1;
                                remainder -= 1;
                            }
                        }
                    }
                }
            }
        } else {
            // Need to shrink: proportional distribution of the flex budget.
            // Columns get at least 1 while the budget lasts; when the budget
            // is smaller than the column count the total still never exceeds
            // the available width.
            let mut granted = 0usize;
            // hand out 1-column minimums in natural-width order first
            let mut order: Vec<usize> = (0..num_cols).filter(|&i| !is_fixed[i]).collect();
            order.sort_by_key(|&i| std::cmp::Reverse(effective[i]));
            let min_count = flex_budget.min(flex_count);
            for &i in order.iter().take(min_count) {
                widths[i] = 1;
                granted += 1;
            }
            let remaining_budget = flex_budget - granted;
            if remaining_budget > 0 && flex_natural_total > 0 {
                for i in 0..num_cols {
                    if !is_fixed[i] && widths[i] > 0 {
                        let ratio = effective[i] as f64 / flex_natural_total as f64;
                        let share = ((remaining_budget as f64) * ratio) as usize;
                        widths[i] += share;
                    }
                }
            }
        }

        widths
    }

    fn render_horizontal_line(&self, widths: &[usize], line: &Line) -> Segment {
        let mut spans = vec![Span::styled(line.left.to_string(), self.style)];

        for (i, &width) in widths.iter().enumerate() {
            let cell_width = width + self.padding * 2;
            spans.push(Span::styled(
                line.mid.to_string().repeat(cell_width),
                self.style,
            ));
            if i < widths.len() - 1 {
                spans.push(Span::styled(line.cross.to_string(), self.style));
            }
        }

        spans.push(Span::styled(line.right.to_string(), self.style));
        Segment::line(spans)
    }

    fn render_row(
        &self,
        cells: &[Text],
        widths: &[usize],
        line: &Line,
        row_base: Option<Style>,
    ) -> Vec<Segment> {
        // Lay out each cell into wrapped/padded lines of spans, then emit one
        // segment per physical row so multi-line cells keep borders aligned
        // and cell styling intact.
        let mut cell_lines: Vec<Vec<Vec<Span>>> = Vec::with_capacity(widths.len());
        let mut row_height = 1usize;

        for (i, width) in widths.iter().enumerate() {
            let lines = match cells.get(i) {
                Some(cell) if !cell.is_empty() => self.cell_lines(cell, *width, i),
                _ => vec![Vec::new()],
            };
            row_height = row_height.max(lines.len());
            cell_lines.push(lines);
        }

        let mut segments = Vec::with_capacity(row_height);

        for r in 0..row_height {
            let mut spans = Vec::new();

            if self.show_border {
                spans.push(Span::styled(line.left.to_string(), self.style));
            }

            for (i, width) in widths.iter().enumerate() {
                spans.push(Span::raw(" ".repeat(self.padding)));

                let line_spans = cell_lines
                    .get(i)
                    .and_then(|lines| lines.get(r))
                    .cloned()
                    .unwrap_or_default();
                let content_width: usize = line_spans.iter().map(|s| s.width()).sum();

                let align = self.columns.get(i).map(|c| c.align).unwrap_or_default();
                let (left_pad, right_pad) = alignment_padding(align, content_width, *width);
                if left_pad > 0 {
                    spans.push(Span::raw(" ".repeat(left_pad)));
                }

                let column_style = self.columns.get(i).map(|c| c.style).unwrap_or_default();
                for mut span in line_spans {
                    // Base layering: row style, then column style, then the
                    // span's own style.
                    if let Some(base) = row_base {
                        span.style = base.combine(&span.style);
                    }
                    if !column_style.is_empty() {
                        span.style = column_style.combine(&span.style);
                    }
                    spans.push(span);
                }

                if right_pad > 0 {
                    spans.push(Span::raw(" ".repeat(right_pad)));
                }

                spans.push(Span::raw(" ".repeat(self.padding)));

                if i < widths.len() - 1 {
                    spans.push(Span::styled(line.cross.to_string(), self.style));
                } else if self.show_border {
                    spans.push(Span::styled(line.right.to_string(), self.style));
                }
            }

            segments.push(Segment::line(spans));
        }

        segments
    }

    /// Lay out one cell into padded lines of spans at the given width.
    ///
    /// Respects the column's `wrap` setting: wrapping columns wrap at the
    /// column width, non-wrapping columns only break on explicit newlines.
    fn cell_lines(&self, cell: &Text, width: usize, col_index: usize) -> Vec<Vec<Span>> {
        let wrap = self.columns.get(col_index).map(|c| c.wrap).unwrap_or(true);
        let lines = if wrap {
            cell.wrap(width)
        } else {
            let mut visible = cell.clone();
            visible.overflow = Overflow::Visible;
            let mut lines = visible.wrap(width);
            // Non-wrapping columns clip lines that overflow the column width.
            for line in &mut lines {
                let line_width: usize = line.iter().map(|s| s.width()).sum();
                if line_width > width {
                    let plain: String = line.iter().map(|s| s.text.as_ref()).collect();
                    let style = line.first().map(|s| s.style).unwrap_or_default();
                    *line = vec![Span::styled(truncate_string(&plain, width), style)];
                }
            }
            lines
        };
        if lines.is_empty() {
            vec![Vec::new()]
        } else {
            lines
        }
    }
}

/// Compute left/right padding for a content of `content_width` in a cell of
/// `cell_width`, according to alignment.
fn alignment_padding(align: Alignment, content_width: usize, cell_width: usize) -> (usize, usize) {
    if content_width >= cell_width {
        return (0, 0);
    }
    let padding = cell_width - content_width;
    match align {
        Alignment::Left => (0, padding),
        Alignment::Right => (padding, 0),
        Alignment::Center => {
            let left = padding / 2;
            (left, padding - left)
        }
    }
}

fn truncate_string(s: &str, width: usize) -> String {
    use unicode_segmentation::UnicodeSegmentation;

    let mut result = String::new();
    let mut current_width = 0;

    for grapheme in s.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if current_width + grapheme_width > width {
            if width > 1 && current_width < width {
                result.push('…');
                current_width += 1;
            }
            break;
        }
        result.push_str(grapheme);
        current_width += grapheme_width;
    }

    // Pad if shorter
    while current_width < width {
        result.push(' ');
        current_width += 1;
    }

    result
}

impl From<&str> for Column {
    fn from(s: &str) -> Self {
        Column::new(s)
    }
}

impl From<String> for Column {
    fn from(s: String) -> Self {
        Column::new(&s)
    }
}

impl Renderable for Table {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if self.columns.is_empty() {
            return vec![];
        }

        let box_chars = self.border_style.to_box();
        let widths = self.calculate_widths(context.width);
        let mut segments = Vec::new();

        // Calculate total table width for title centering
        let content_width: usize = widths.iter().map(|w| w + self.padding * 2).sum();
        let border_overhead = if self.show_border {
            widths.len() + 1
        } else {
            widths.len() - 1
        };
        let table_width = content_width + border_overhead;

        // Title
        if let Some(title) = &self.title {
            let title_width = UnicodeWidthStr::width(title.as_str());
            if title_width <= table_width {
                let padding = table_width - title_width;
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;

                let mut spans = Vec::new();
                if left_pad > 0 {
                    spans.push(Span::raw(" ".repeat(left_pad)));
                }
                spans.push(Span::styled(title.clone(), Style::new().bold()));
                if right_pad > 0 {
                    spans.push(Span::raw(" ".repeat(right_pad)));
                }
                segments.push(Segment::line(spans));
            } else {
                // Truncate or just print? Just print for now.
                segments.push(Segment::line(vec![Span::styled(
                    title.clone(),
                    Style::new().bold(),
                )]));
            }
        }

        // Top border
        if self.show_border {
            segments.push(self.render_horizontal_line(&widths, &box_chars.top));
        }

        // Header row
        if self.show_header {
            let header_cells: Vec<Text> = self
                .columns
                .iter()
                .map(|c| Text::styled(c.header.clone(), c.header_style))
                .collect();
            // Use header box line for vertical separators in header
            segments.extend(self.render_row(&header_cells, &widths, &box_chars.header, None));

            // Header separator
            if self.show_border || self.show_row_lines {
                segments.push(self.render_horizontal_line(&widths, &box_chars.head));
            }
        }

        // Data rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            // Use cell box line for vertical separators in body
            segments.extend(self.render_row(
                &row.cells,
                &widths,
                &box_chars.cell,
                row.base_style(),
            ));

            // Row separator
            if self.show_row_lines && row_idx < self.rows.len() - 1 {
                segments.push(self.render_horizontal_line(&widths, &box_chars.mid));
            }
        }

        // Bottom border
        if self.show_border {
            segments.push(self.render_horizontal_line(&widths, &box_chars.bottom));
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_basic() {
        let mut table = Table::new();
        table.add_column("Name");
        table.add_column("Age");
        table.add_row_strs(&["Alice", "30"]);
        table.add_row_strs(&["Bob", "25"]);

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = table.render(&context);

        assert!(!segments.is_empty());

        // Check that output contains our data
        let text: String = segments.iter().map(|s| s.plain_text()).collect();
        assert!(text.contains("Name"));
        assert!(text.contains("Alice"));
        assert!(text.contains("Bob"));
    }

    #[test]
    fn test_table_builder() {
        let table = Table::new()
            .columns(["A", "B", "C"])
            .border_style(BorderStyle::Square);

        assert_eq!(table.columns.len(), 3);
    }

    #[test]
    fn test_alignment_padding() {
        assert_eq!(alignment_padding(Alignment::Left, 2, 5), (0, 3));
        assert_eq!(alignment_padding(Alignment::Right, 2, 5), (3, 0));
        assert_eq!(alignment_padding(Alignment::Center, 2, 5), (1, 2));
        assert_eq!(alignment_padding(Alignment::Left, 5, 5), (0, 0));
        assert_eq!(alignment_padding(Alignment::Left, 7, 5), (0, 0));
    }

    #[test]
    fn test_truncated_cell_exactly_column_width() {
        // Regression: pushing the ellipsis must count toward the width.
        let s = truncate_string("abcd\u{1F600}", 5); // 4 + 2-wide emoji = 6 cols
        assert_eq!(UnicodeWidthStr::width(s.as_str()), 5, "got {s:?}");
        assert!(s.ends_with('\u{2026}'));
    }

    #[test]
    fn test_column_width_fixed_is_honored() {
        let mut table = Table::new();
        table.add_column(Column::new("A").width(ColumnWidth::Fixed(10)));
        table.add_row_strs(&["tiny"]);
        let widths = table.calculate_widths(80);
        assert_eq!(widths, vec![10]);
    }

    #[test]
    fn test_column_width_min_and_max() {
        let mut table = Table::new();
        table.add_column(Column::new("A").width(ColumnWidth::Min(20)));
        table.add_column(Column::new("B").width(ColumnWidth::Max(3)));
        table.add_row_strs(&["x", "longer-than-three"]);
        let widths = table.calculate_widths(200);
        assert_eq!(widths, vec![20, 3]);
    }

    #[test]
    fn test_multiline_cell_renders_aligned_rows() {
        let mut table = Table::new();
        table.add_column("A");
        table.add_column("B");
        table.add_row_strs(&["one\ntwo", "x"]);

        let context = RenderContext {
            width: 60,
            height: None,
        };
        let segments = table.render(&context);
        // border, header, head-sep, 2 data lines, bottom = 6
        assert!(segments.len() >= 6);

        // Every segment must have the same display width (borders aligned)
        let row_widths: Vec<usize> = segments
            .iter()
            .map(|s| s.spans.iter().map(|sp| sp.width()).sum::<usize>())
            .collect();
        let all_eq = row_widths.iter().all(|&w| w == row_widths[0]);
        assert!(all_eq, "rows must align: {row_widths:?}");
    }

    #[test]
    fn test_cell_styles_survive() {
        use crate::style::Color;
        let mut table = Table::new();
        table.add_column("A");
        let mut cell = Text::plain("hot".to_string());
        cell.spans[0].style = Style::new().foreground(Color::Red);
        table.add_row(vec![cell]);

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = table.render(&context);
        let found = segments.iter().any(|seg| {
            seg.spans
                .iter()
                .any(|sp| sp.text.contains("hot") && sp.style.foreground == Some(Color::Red))
        });
        assert!(found, "cell style must not be flattened away");
    }

    #[test]
    fn test_row_style_applied() {
        use crate::style::Color;
        let mut table = Table::new();
        table.add_column("A");
        let row = Row::new(vec!["styled"]).style(Style::new().foreground(Color::Green));
        table.add_row_obj(row);

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = table.render(&context);
        let found = segments.iter().any(|seg| {
            seg.spans
                .iter()
                .any(|sp| sp.text.contains("styled") && sp.style.foreground == Some(Color::Green))
        });
        assert!(found, "row style must be applied to cells");
    }

    #[test]
    fn test_column_wrap_wraps_long_cells() {
        let mut table = Table::new();
        table.add_column(Column::new("A").width(ColumnWidth::Fixed(8)));
        table.add_row_strs(&["aaaa bbbb cccc"]);

        let context = RenderContext {
            width: 60,
            height: None,
        };
        let segments = table.render(&context);
        // header + sep + 2 wrapped lines + bottom
        let text: Vec<String> = segments.iter().map(|s| s.plain_text()).collect();
        assert!(
            text.iter().any(|l| l.contains("aaaa")) && text.iter().any(|l| l.contains("bbbb")),
            "content should wrap across lines: {text:?}"
        );
    }
}
