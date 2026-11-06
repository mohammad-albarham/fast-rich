//! Tables for displaying structured data.
//!
//! Tables support headers, multiple columns with alignment and width control,
//! titles and captions, and various border styles. Cells can hold any
//! [`Renderable`] (see [`Table::add_row_renderables`]), so panels, trees or
//! other tables can be nested, as in Rich.
//!
//! Column widths follow Rich's algorithm: every column starts at its
//! natural (widest) width; when the table is too wide the widest wrapping
//! columns shrink first, and `expand` distributes spare space (by `ratio`
//! when set).

use crate::box_drawing::Line;
use crate::console::RenderContext;
use crate::panel::BorderStyle;
use crate::renderable::{fit_line, render_lines, Renderable, Segment, SharedRenderable};
use crate::style::Style;
use crate::text::{Alignment, Overflow, Span, Text};
use std::sync::Arc;

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
    /// Fixed content width (padding excluded), like Rich's `width=`
    Fixed(usize),
    /// Minimum content width, like Rich's `min_width=`
    Min(usize),
    /// Maximum content width, like Rich's `max_width=`
    Max(usize),
}

/// A table cell: text (aligned, wrapped and cut by its column's settings)
/// or any other renderable.
#[derive(Clone)]
pub enum Cell {
    /// Text laid out with the column's alignment, wrap and overflow.
    Text(Text),
    /// Any renderable, rendered at the column width.
    Renderable(SharedRenderable),
}

impl<T: Into<Text>> From<T> for Cell {
    fn from(text: T) -> Self {
        Cell::Text(text.into())
    }
}

impl Cell {
    /// Wrap any renderable as a cell.
    pub fn renderable(r: impl Renderable + Send + Sync + 'static) -> Self {
        Cell::Renderable(Arc::new(r))
    }
}

impl Cell {
    fn min_width(&self) -> usize {
        match self {
            Cell::Text(t) => t.min_width(),
            Cell::Renderable(r) => r.min_width(),
        }
    }

    fn max_width(&self) -> usize {
        match self {
            Cell::Text(t) => t.max_width(),
            Cell::Renderable(r) => r.max_width(),
        }
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Text(t) => f.debug_tuple("Text").field(&t.plain_text()).finish(),
            Cell::Renderable(_) => f.write_str("Renderable"),
        }
    }
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
    /// How non-wrapping cells are cut when too wide
    overflow: Overflow,
    /// Share of spare width when the table expands
    ratio: Option<usize>,
    /// Header as a renderable (overrides `header`)
    header_cell: Option<Cell>,
}

impl Column {
    /// Create a new column with a header.
    pub fn new(header: &str) -> Self {
        Column {
            header: header.to_string(),
            align: Alignment::Left,
            width: ColumnWidth::Auto,
            header_style: Style::new().bold(),
            style: Style::new(),
            wrap: true,
            overflow: Overflow::Ellipsis,
            ratio: None,
            header_cell: None,
        }
    }

    /// Use styled text (e.g. parsed markup) as the header.
    pub fn header_text(mut self, header: Text) -> Self {
        self.header = header.plain_text();
        self.header_cell = Some(Cell::Text(header));
        self
    }

    /// Use any renderable as the header.
    pub fn header_renderable(mut self, header: impl Renderable + Send + Sync + 'static) -> Self {
        self.header_cell = Some(Cell::Renderable(Arc::new(header)));
        self
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

    /// Disable wrapping (Rich's `no_wrap=True`).
    pub fn no_wrap(self, no_wrap: bool) -> Self {
        self.wrap(!no_wrap)
    }

    /// How a non-wrapping cell that is too wide is cut: `Ellipsis`
    /// (default, like Rich), `Truncate`, or `Visible`.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Share of the spare width this column gets when the table expands.
    pub fn ratio(mut self, ratio: usize) -> Self {
        self.ratio = Some(ratio);
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

    fn header_cell(&self) -> Cell {
        self.header_cell
            .clone()
            .unwrap_or_else(|| Cell::Text(Text::plain(self.header.clone())))
    }

    fn fixed_width(&self) -> Option<usize> {
        match self.width {
            ColumnWidth::Fixed(w) => Some(w),
            _ => None,
        }
    }

    fn flexible(&self) -> bool {
        self.ratio.is_some()
    }
}

/// A row of table cells.
#[derive(Debug, Clone)]
pub struct Row {
    cells: Vec<Cell>,
    style: Option<Style>,
    end_section: bool,
}

impl Row {
    /// Create a new row with cells.
    pub fn new<I, T>(cells: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        Row {
            cells: cells.into_iter().map(|c| Cell::Text(c.into())).collect(),
            style: None,
            end_section: false,
        }
    }

    /// Create a row from cells that may mix text and renderables.
    pub fn cells<I>(cells: I) -> Self
    where
        I: IntoIterator<Item = Cell>,
    {
        Row {
            cells: cells.into_iter().collect(),
            style: None,
            end_section: false,
        }
    }

    /// Create a row whose cells are arbitrary renderables.
    pub fn renderables<I>(cells: I) -> Self
    where
        I: IntoIterator<Item = SharedRenderable>,
    {
        Row {
            cells: cells.into_iter().map(Cell::Renderable).collect(),
            style: None,
            end_section: false,
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

    /// Draw a separator line after this row (Rich's `end_section`).
    pub fn end_section(mut self, end_section: bool) -> Self {
        self.end_section = end_section;
        self
    }

    /// The row's base style, if any.
    pub fn base_style(&self) -> Option<Style> {
        self.style
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
    /// Show the outer edge of the border
    show_edge: bool,
    /// Show row separators
    show_row_lines: bool,
    /// Padding in cells
    padding: usize,
    /// Title
    title: Option<Text>,
    /// Title style and alignment
    title_style: Style,
    title_align: Alignment,
    /// Caption below the table
    caption: Option<Text>,
    caption_style: Style,
    caption_align: Alignment,
    /// Base style for all header cells
    header_style: Style,
    /// Alternating row styles
    row_styles: Vec<Style>,
    /// Expand to full width
    expand: bool,
    /// Fixed total width
    width: Option<usize>,
    /// Minimum total width
    min_width: Option<usize>,
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
            show_edge: true,
            show_row_lines: false,
            padding: 1,
            title: None,
            title_style: Style::new().italic(),
            title_align: Alignment::Center,
            caption: None,
            caption_style: Style::new().italic().dim(),
            caption_align: Alignment::Center,
            header_style: Style::new(),
            row_styles: Vec::new(),
            expand: false,
            width: None,
            min_width: None,
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
        self.rows
            .push(Row::new(cells.iter().map(|s| Text::plain(s.to_string()))));
        self
    }

    /// Add a row whose cells are any renderables (panels, trees, tables…).
    ///
    /// ```
    /// use std::sync::Arc;
    /// use fast_rich::panel::Panel;
    /// use fast_rich::renderable::SharedRenderable;
    /// use fast_rich::table::Table;
    /// let mut table = Table::new().columns(["Boxed"]);
    /// let cell: SharedRenderable = Arc::new(Panel::new("hi"));
    /// table.add_row_renderables(vec![cell]);
    /// ```
    pub fn add_row_renderables(&mut self, cells: Vec<SharedRenderable>) -> &mut Self {
        self.rows.push(Row::renderables(cells));
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

    /// Set whether to show the border. Without a border, columns are
    /// separated by padding only (Rich's `box=None`).
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set whether to draw the outer edge of the border (Rich's `show_edge`).
    pub fn show_edge(mut self, show: bool) -> Self {
        self.show_edge = show;
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
    pub fn title(self, title: &str) -> Self {
        self.title_text(Text::plain(title.to_string()))
    }

    /// Set the title from styled text.
    pub fn title_text(mut self, title: Text) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the title style (default italic, like Rich).
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Set the title alignment (default center).
    pub fn title_align(mut self, align: Alignment) -> Self {
        self.title_align = align;
        self
    }

    /// Set a caption shown below the table.
    pub fn caption(self, caption: &str) -> Self {
        self.caption_text(Text::plain(caption.to_string()))
    }

    /// Set the caption from styled text.
    pub fn caption_text(mut self, caption: Text) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Set the caption style (default italic dim, like Rich).
    pub fn caption_style(mut self, style: Style) -> Self {
        self.caption_style = style;
        self
    }

    /// Set the caption alignment (default center).
    pub fn caption_align(mut self, align: Alignment) -> Self {
        self.caption_align = align;
        self
    }

    /// Base style applied under every column's header style.
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    /// Styles applied to body rows in turn (e.g. zebra stripes).
    pub fn row_styles(mut self, styles: Vec<Style>) -> Self {
        self.row_styles = styles;
        self
    }

    /// Set whether to expand to full width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Set a fixed total width (borders included).
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set a minimum total width (borders included).
    pub fn min_width(mut self, width: usize) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Width taken by borders and column separators.
    fn extra_width(&self) -> usize {
        if !self.show_border || self.columns.is_empty() {
            return 0;
        }
        let edges = if self.show_edge { 2 } else { 0 };
        edges + self.columns.len() - 1
    }

    fn padding_width(&self) -> usize {
        self.padding * 2
    }

    /// (min, max) cell width of a column, padding included — Rich's
    /// `_measure_column`.
    fn measure_column(&self, index: usize, max_width: usize) -> (usize, usize) {
        let column = &self.columns[index];
        let pad = self.padding_width();
        if let Some(w) = column.fixed_width() {
            let w = (w + pad).min(max_width);
            return (w, w);
        }
        let mut min_w = 0;
        let mut max_w = 0;
        let mut measure = |cell: &Cell| {
            min_w = min_w.max(cell.min_width().saturating_add(pad));
            max_w = max_w.max(cell.max_width().saturating_add(pad));
        };
        if self.show_header {
            measure(&column.header_cell());
        }
        for row in &self.rows {
            if let Some(cell) = row.cells.get(index) {
                measure(cell);
            }
        }
        // Measurement.with_maximum(max_width)
        let mut max_w = max_w.min(max_width);
        let mut min_w = min_w.min(max_w);
        match column.width {
            ColumnWidth::Min(m) => {
                min_w = min_w.max(m + pad);
                max_w = max_w.max(min_w);
            }
            ColumnWidth::Max(m) => {
                max_w = max_w.min(m + pad);
                min_w = min_w.min(max_w);
            }
            _ => {}
        }
        (min_w, max_w)
    }

    /// Cell widths (padding included) — Rich's `_calculate_column_widths`.
    fn cell_widths(&self, available_width: usize) -> Vec<usize> {
        let n = self.columns.len();
        if n == 0 {
            return vec![];
        }
        let max_width = self
            .width
            .unwrap_or(available_width)
            .min(available_width)
            .saturating_sub(self.extra_width());
        let ranges: Vec<(usize, usize)> =
            (0..n).map(|i| self.measure_column(i, max_width)).collect();
        let mut widths: Vec<usize> = ranges.iter().map(|r| r.1.max(1)).collect();

        if self.expand && self.columns.iter().any(|c| c.ratio.unwrap_or(0) > 0) {
            let fixed: Vec<usize> = self
                .columns
                .iter()
                .zip(&ranges)
                .map(|(c, r)| if c.flexible() { 0 } else { r.1 })
                .collect();
            let flex: Vec<usize> = (0..n).filter(|&i| self.columns[i].flexible()).collect();
            let ratios: Vec<usize> = flex
                .iter()
                .map(|&i| self.columns[i].ratio.unwrap_or(0))
                .collect();
            let minimums: Vec<usize> = flex
                .iter()
                .map(|&i| self.columns[i].fixed_width().unwrap_or(1) + self.padding_width())
                .collect();
            let flexible_width = max_width.saturating_sub(fixed.iter().sum());
            let flex_widths = ratio_distribute(flexible_width, &ratios, Some(&minimums));
            for (k, &i) in flex.iter().enumerate() {
                widths[i] = fixed[i] + flex_widths[k];
            }
        }

        let mut table_width: usize = widths.iter().sum();
        if table_width > max_width {
            let wrapable: Vec<bool> = self
                .columns
                .iter()
                .map(|c| c.fixed_width().is_none() && c.wrap)
                .collect();
            widths = collapse_widths(widths, &wrapable, max_width);
            table_width = widths.iter().sum();
            if table_width > max_width {
                // Last resort: reduce every column evenly.
                let excess = table_width - max_width;
                let ones = vec![1; n];
                widths = ratio_reduce(excess, &ones, &widths.clone(), &widths);
            }
            // Re-measure at the granted widths (Measurement.with_maximum).
            widths = widths
                .iter()
                .enumerate()
                .map(|(i, &w)| self.measure_column(i, w).1)
                .collect();
            table_width = widths.iter().sum();
        }

        // Grow to fill when expanding, or up to `min_width` (Rich grows to
        // min(min_width, max_width) even when expanding).
        let min_target = self.min_width.map(|m| m.saturating_sub(self.extra_width()));
        let grow =
            (self.expand && table_width < max_width) || min_target.is_some_and(|m| table_width < m);
        if grow {
            let target = min_target.map_or(max_width, |m| m.min(max_width));
            if target > table_width && widths.iter().any(|&w| w > 0) {
                let pads = ratio_distribute(target - table_width, &widths, None);
                for (w, p) in widths.iter_mut().zip(pads) {
                    *w += p;
                }
            }
        }
        widths
    }

    /// Content widths (padding excluded) for the given available width.
    #[cfg(test)]
    fn calculate_widths(&self, available_width: usize) -> Vec<usize> {
        let pad = self.padding_width();
        self.cell_widths(available_width)
            .into_iter()
            .map(|w| w.saturating_sub(pad))
            .collect()
    }

    fn horizontal_line(&self, widths: &[usize], line: &Line) -> Segment {
        let mut text = String::new();
        if self.show_edge {
            text.push(line.left);
        }
        for (i, &w) in widths.iter().enumerate() {
            text.push_str(&line.mid.to_string().repeat(w));
            if i + 1 < widths.len() {
                text.push(line.cross);
            }
        }
        if self.show_edge {
            text.push(line.right);
        }
        Segment::line(vec![Span::styled(text, self.style)])
    }

    /// Render one cell into exactly `width`-wide lines (padding included).
    fn cell_lines(
        &self,
        cell: &Cell,
        column: &Column,
        width: usize,
        base: Style,
    ) -> Vec<Vec<Span>> {
        let content_width = width.saturating_sub(self.padding_width());
        let lines = match cell {
            Cell::Text(text) => {
                let mut text = text.clone();
                text.alignment = column.align;
                // Rich: wrapping columns still cut over-long words with the
                // column's overflow (ellipsis by default).
                text.overflow = match (column.wrap, column.overflow) {
                    (true, Overflow::Ellipsis | Overflow::WrapEllipsis) => Overflow::WrapEllipsis,
                    (true, Overflow::Truncate | Overflow::WrapCrop) => Overflow::WrapCrop,
                    (true, _) => Overflow::Wrap,
                    (false, Overflow::Wrap | Overflow::WrapEllipsis) => Overflow::Ellipsis,
                    (false, Overflow::WrapCrop) => Overflow::Truncate,
                    (false, other) => other,
                };
                render_lines(&text, content_width, Style::new())
            }
            Cell::Renderable(r) => render_lines(&**r, content_width, Style::new()),
        };
        let pad = Span::raw(" ".repeat(self.padding));
        lines
            .into_iter()
            .map(|line| {
                let mut spans = Vec::with_capacity(line.len() + 2);
                if self.padding > 0 {
                    spans.push(pad.clone());
                }
                spans.extend(line);
                if self.padding > 0 {
                    spans.push(pad.clone());
                }
                spans
                    .into_iter()
                    .map(|mut s| {
                        s.style = base.combine(&s.style);
                        s
                    })
                    .collect()
            })
            .collect()
    }

    fn render_row(
        &self,
        cells: &[Cell],
        widths: &[usize],
        line: &Line,
        header: bool,
        row_style: Style,
    ) -> Vec<Segment> {
        let empty = Cell::Text(Text::new());
        let bases: Vec<Style> = self
            .columns
            .iter()
            .map(|column| {
                let cell_style = if header {
                    self.header_style.combine(&column.header_style)
                } else {
                    column.style
                };
                row_style.combine(&cell_style)
            })
            .collect();
        let mut rendered: Vec<Vec<Vec<Span>>> = Vec::with_capacity(widths.len());
        for (i, &width) in widths.iter().enumerate() {
            let cell = cells.get(i).unwrap_or(&empty);
            rendered.push(self.cell_lines(cell, &self.columns[i], width, bases[i]));
        }
        let height = rendered.iter().map(Vec::len).max().unwrap_or(1).max(1);
        let mut segments = Vec::with_capacity(height);
        for r in 0..height {
            let mut spans = Vec::new();
            if self.show_border && self.show_edge {
                spans.push(Span::styled(line.left.to_string(), self.style));
            }
            for (i, &width) in widths.iter().enumerate() {
                let cell_line = rendered[i].get(r).cloned().unwrap_or_default();
                spans.extend(fit_line(cell_line, width, bases[i]));
                if self.show_border && i + 1 < widths.len() {
                    spans.push(Span::styled(line.cross.to_string(), self.style));
                }
            }
            if self.show_border && self.show_edge {
                spans.push(Span::styled(line.right.to_string(), self.style));
            }
            segments.push(Segment::line(spans));
        }
        segments
    }

    fn annotation(
        &self,
        text: &Text,
        style: Style,
        align: Alignment,
        width: usize,
    ) -> Vec<Segment> {
        let mut text = text.clone();
        text.alignment = align;
        for span in &mut text.spans {
            span.style = style.combine(&span.style);
        }
        render_lines(&text, width, Style::new())
            .into_iter()
            .map(Segment::line)
            .collect()
    }
}

/// Shrink the widest wrapping columns until the total fits — Rich's
/// `Table._collapse_widths`.
fn collapse_widths(mut widths: Vec<usize>, wrapable: &[bool], max_width: usize) -> Vec<usize> {
    let mut total: usize = widths.iter().sum();
    if !wrapable.iter().any(|&w| w) {
        return widths;
    }
    while total > 0 && total > max_width {
        let excess = total - max_width;
        let max_column = widths
            .iter()
            .zip(wrapable)
            .filter(|(_, &w)| w)
            .map(|(&v, _)| v)
            .max()
            .unwrap_or(0);
        let second_max = widths
            .iter()
            .zip(wrapable)
            .map(|(&v, &w)| if w && v != max_column { v } else { 0 })
            .max()
            .unwrap_or(0);
        let difference = max_column - second_max;
        let ratios: Vec<usize> = widths
            .iter()
            .zip(wrapable)
            .map(|(&v, &w)| usize::from(w && v == max_column))
            .collect();
        if !ratios.iter().any(|&r| r > 0) || difference == 0 {
            break;
        }
        let max_reduce = vec![excess.min(difference); widths.len()];
        widths = ratio_reduce(excess, &ratios, &max_reduce, &widths);
        total = widths.iter().sum();
    }
    widths
}

/// Python's `round()` (round half to even) for a non-negative ratio.
fn round_half_even(numerator: usize, denominator: usize) -> usize {
    let q = numerator / denominator;
    let r = numerator % denominator;
    match (2 * r).cmp(&denominator) {
        std::cmp::Ordering::Less => q,
        std::cmp::Ordering::Greater => q + 1,
        std::cmp::Ordering::Equal => q + (q % 2),
    }
}

/// Reduce `values` by `total`, in proportion to `ratios`, each by at most
/// its `maximums` entry — Rich's `_ratio.ratio_reduce`.
fn ratio_reduce(
    total: usize,
    ratios: &[usize],
    maximums: &[usize],
    values: &[usize],
) -> Vec<usize> {
    let ratios: Vec<usize> = ratios
        .iter()
        .zip(maximums)
        .map(|(&r, &m)| if m > 0 { r } else { 0 })
        .collect();
    let mut total_ratio: usize = ratios.iter().sum();
    if total_ratio == 0 {
        return values.to_vec();
    }
    let mut remaining = total;
    let mut result = Vec::with_capacity(values.len());
    for ((&ratio, &maximum), &value) in ratios.iter().zip(maximums).zip(values) {
        if ratio > 0 && total_ratio > 0 {
            let distributed = maximum.min(round_half_even(ratio * remaining, total_ratio));
            result.push(value.saturating_sub(distributed));
            remaining = remaining.saturating_sub(distributed);
            total_ratio -= ratio;
        } else {
            result.push(value);
        }
    }
    result
}

/// Split `total` in proportion to `ratios` (each at least its minimum) —
/// Rich's `_ratio.ratio_distribute`.
fn ratio_distribute(total: usize, ratios: &[usize], minimums: Option<&[usize]>) -> Vec<usize> {
    let ratios: Vec<usize> = match minimums {
        Some(mins) => ratios
            .iter()
            .zip(mins)
            .map(|(&r, &m)| if m > 0 { r } else { 0 })
            .collect(),
        None => ratios.to_vec(),
    };
    let mut total_ratio: usize = ratios.iter().sum();
    let mut remaining = total;
    let mut out = Vec::with_capacity(ratios.len());
    for (i, &ratio) in ratios.iter().enumerate() {
        let minimum = minimums.and_then(|m| m.get(i).copied()).unwrap_or(0);
        let distributed = if total_ratio > 0 {
            minimum.max((ratio * remaining).div_ceil(total_ratio))
        } else {
            remaining
        };
        out.push(distributed);
        total_ratio -= ratio;
        remaining = remaining.saturating_sub(distributed);
    }
    out
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
        let widths = self.cell_widths(context.width);
        let table_width: usize = widths.iter().sum::<usize>() + self.extra_width();
        let mut segments = Vec::new();

        if let Some(title) = &self.title {
            segments.extend(self.annotation(
                title,
                self.title_style,
                self.title_align,
                table_width,
            ));
        }

        if self.show_border && self.show_edge {
            segments.push(self.horizontal_line(&widths, &box_chars.top));
        }

        if self.show_header {
            let header_cells: Vec<Cell> = self.columns.iter().map(Column::header_cell).collect();
            segments.extend(self.render_row(
                &header_cells,
                &widths,
                &box_chars.header,
                true,
                Style::new(),
            ));
            if self.show_border {
                segments.push(self.horizontal_line(&widths, &box_chars.head));
            }
        }

        for (row_idx, row) in self.rows.iter().enumerate() {
            let mut row_style = Style::new();
            if !self.row_styles.is_empty() {
                row_style = row_style.combine(&self.row_styles[row_idx % self.row_styles.len()]);
            }
            if let Some(s) = row.base_style() {
                row_style = row_style.combine(&s);
            }
            segments.extend(self.render_row(
                &row.cells,
                &widths,
                &box_chars.cell,
                false,
                row_style,
            ));

            let last = row_idx + 1 == self.rows.len();
            if self.show_border && !last && (self.show_row_lines || row.end_section) {
                segments.push(self.horizontal_line(&widths, &box_chars.mid));
            }
        }

        if self.show_border && self.show_edge {
            segments.push(self.horizontal_line(&widths, &box_chars.bottom));
        }

        if let Some(caption) = &self.caption {
            segments.extend(self.annotation(
                caption,
                self.caption_style,
                self.caption_align,
                table_width,
            ));
        }

        segments
    }

    fn min_width(&self) -> usize {
        if let Some(w) = self.width {
            return w;
        }
        let total: usize = (0..self.columns.len())
            .map(|i| self.measure_column(i, usize::MAX).0)
            .sum();
        (total + self.extra_width()).max(self.min_width.unwrap_or(0))
    }

    fn max_width(&self) -> usize {
        if let Some(w) = self.width {
            return w;
        }
        let total: usize = (0..self.columns.len())
            .map(|i| self.measure_column(i, usize::MAX).1)
            .fold(0usize, usize::saturating_add);
        total
            .saturating_add(self.extra_width())
            .max(self.min_width.unwrap_or(0))
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
