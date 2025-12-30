//! Tables for displaying structured data.
//!
//! Tables support headers, multiple columns with alignment and width control,
//! and various border styles.

use crate::console::RenderContext;
use crate::panel::BorderStyle;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Span, Text};
use unicode_width::UnicodeWidthStr;

/// Column alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnAlign {
    #[default]
    Left,
    Center,
    Right,
}

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
    pub align: ColumnAlign,
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
            align: ColumnAlign::Left,
            width: ColumnWidth::Auto,
            header_style: Style::new().bold(),
            style: Style::new(),
            wrap: true,
            min_width: header_width,
            max_width: header_width,
        }
    }

    /// Set the column alignment.
    pub fn align(mut self, align: ColumnAlign) -> Self {
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
        self.align(ColumnAlign::Center)
    }

    /// Right align shorthand.
    pub fn right(self) -> Self {
        self.align(ColumnAlign::Right)
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
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

/// Table border characters.
#[derive(Debug, Clone, Copy)]
struct TableBorderChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
    cross: char,
    top_t: char,
    bottom_t: char,
    left_t: char,
    right_t: char,
}

impl TableBorderChars {
    fn from_style(style: BorderStyle) -> Self {
        match style {
            BorderStyle::Rounded => TableBorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
                cross: '┼',
                top_t: '┬',
                bottom_t: '┴',
                left_t: '├',
                right_t: '┤',
            },
            BorderStyle::Square => TableBorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
                cross: '┼',
                top_t: '┬',
                bottom_t: '┴',
                left_t: '├',
                right_t: '┤',
            },
            BorderStyle::Heavy => TableBorderChars {
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                horizontal: '━',
                vertical: '┃',
                cross: '╋',
                top_t: '┳',
                bottom_t: '┻',
                left_t: '┣',
                right_t: '┫',
            },
            BorderStyle::Double => TableBorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
                cross: '╬',
                top_t: '╦',
                bottom_t: '╩',
                left_t: '╠',
                right_t: '╣',
            },
            BorderStyle::Ascii => TableBorderChars {
                top_left: '+',
                top_right: '+',
                bottom_left: '+',
                bottom_right: '+',
                horizontal: '-',
                vertical: '|',
                cross: '+',
                top_t: '+',
                bottom_t: '+',
                left_t: '+',
                right_t: '+',
            },
            BorderStyle::Minimal | BorderStyle::Hidden => TableBorderChars {
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                horizontal: '─',
                vertical: ' ',
                cross: '─',
                top_t: '─',
                bottom_t: '─',
                left_t: '─',
                right_t: '─',
            },
        }
    }
}

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

    /// Calculate column widths based on content.
    fn calculate_widths(&self, available_width: usize) -> Vec<usize> {
        let num_cols = self.columns.len();
        if num_cols == 0 {
            return vec![];
        }

        // Calculate content widths
        let mut max_widths: Vec<usize> = self
            .columns
            .iter()
            .map(|c| UnicodeWidthStr::width(c.header.as_str()))
            .collect();

        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i < max_widths.len() {
                    max_widths[i] = max_widths[i].max(cell.width());
                }
            }
        }

        // Calculate overhead (borders, padding)
        let overhead = if self.show_border {
            1 + num_cols + 1 + (self.padding * 2 * num_cols)
        } else {
            (num_cols - 1) + (self.padding * 2 * num_cols)
        };

        let content_width = available_width.saturating_sub(overhead);

        // Simple proportional distribution
        let total_content: usize = max_widths.iter().sum();
        if total_content == 0 {
            return vec![content_width / num_cols.max(1); num_cols];
        }

        if total_content <= content_width {
            // Everything fits
            if self.expand {
                // Distribute extra space
                let extra = content_width - total_content;
                let per_col = extra / num_cols;
                max_widths.iter().map(|w| w + per_col).collect()
            } else {
                max_widths
            }
        } else {
            // Need to shrink - proportional distribution
            max_widths
                .iter()
                .map(|w| {
                    let ratio = *w as f64 / total_content as f64;
                    ((content_width as f64 * ratio) as usize).max(1)
                })
                .collect()
        }
    }

    fn render_horizontal_line(
        &self,
        widths: &[usize],
        chars: &TableBorderChars,
        left: char,
        mid: char,
        right: char,
    ) -> Segment {
        let mut spans = vec![Span::styled(left.to_string(), self.style)];

        for (i, &width) in widths.iter().enumerate() {
            let cell_width = width + self.padding * 2;
            spans.push(Span::styled(
                chars.horizontal.to_string().repeat(cell_width),
                self.style,
            ));
            if i < widths.len() - 1 {
                spans.push(Span::styled(mid.to_string(), self.style));
            }
        }

        spans.push(Span::styled(right.to_string(), self.style));
        Segment::line(spans)
    }

    fn render_row(
        &self,
        cells: &[Text],
        widths: &[usize],
        chars: &TableBorderChars,
        cell_styles: &[Style],
    ) -> Vec<Segment> {
        // For simplicity, render single-line rows
        // A full implementation would handle wrapping
        let mut spans = Vec::new();

        if self.show_border {
            spans.push(Span::styled(chars.vertical.to_string(), self.style));
        }

        for (i, width) in widths.iter().enumerate() {
            let cell = cells.get(i);
            let content = cell.map(|c| c.plain_text()).unwrap_or_default();
            let _content_width = UnicodeWidthStr::width(content.as_str());
            let cell_style = cell_styles.get(i).copied().unwrap_or_default();

            let align = self.columns.get(i).map(|c| c.align).unwrap_or_default();
            let padded = pad_string(&content, *width, align);

            // Add padding
            spans.push(Span::raw(" ".repeat(self.padding)));
            spans.push(Span::styled(padded, cell_style));
            spans.push(Span::raw(" ".repeat(self.padding)));

            if i < widths.len() - 1 || self.show_border {
                spans.push(Span::styled(chars.vertical.to_string(), self.style));
            }
        }

        vec![Segment::line(spans)]
    }
}

fn pad_string(s: &str, width: usize, align: ColumnAlign) -> String {
    let content_width = UnicodeWidthStr::width(s);
    if content_width >= width {
        return truncate_string(s, width);
    }

    let padding = width - content_width;
    match align {
        ColumnAlign::Left => format!("{}{}", s, " ".repeat(padding)),
        ColumnAlign::Right => format!("{}{}", " ".repeat(padding), s),
        ColumnAlign::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
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

        let chars = TableBorderChars::from_style(self.border_style);
        let widths = self.calculate_widths(context.width);
        let mut segments = Vec::new();

        // Top border
        if self.show_border {
            segments.push(self.render_horizontal_line(
                &widths,
                &chars,
                chars.top_left,
                chars.top_t,
                chars.top_right,
            ));
        }

        // Header row
        if self.show_header {
            let header_cells: Vec<Text> = self
                .columns
                .iter()
                .map(|c| Text::styled(c.header.clone(), c.header_style))
                .collect();
            let header_styles: Vec<Style> = self.columns.iter().map(|c| c.header_style).collect();
            segments.extend(self.render_row(&header_cells, &widths, &chars, &header_styles));

            // Header separator
            if self.show_border || self.show_row_lines {
                segments.push(self.render_horizontal_line(
                    &widths,
                    &chars,
                    chars.left_t,
                    chars.cross,
                    chars.right_t,
                ));
            }
        }

        // Data rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            let cell_styles: Vec<Style> = self.columns.iter().map(|c| c.style).collect();
            segments.extend(self.render_row(&row.cells, &widths, &chars, &cell_styles));

            // Row separator
            if self.show_row_lines && row_idx < self.rows.len() - 1 {
                segments.push(self.render_horizontal_line(
                    &widths,
                    &chars,
                    chars.left_t,
                    chars.cross,
                    chars.right_t,
                ));
            }
        }

        // Bottom border
        if self.show_border {
            segments.push(self.render_horizontal_line(
                &widths,
                &chars,
                chars.bottom_left,
                chars.bottom_t,
                chars.bottom_right,
            ));
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

        let context = RenderContext { width: 40 };
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
    fn test_pad_string() {
        assert_eq!(pad_string("hi", 5, ColumnAlign::Left), "hi   ");
        assert_eq!(pad_string("hi", 5, ColumnAlign::Right), "   hi");
        assert_eq!(pad_string("hi", 5, ColumnAlign::Center), " hi  ");
    }
}
