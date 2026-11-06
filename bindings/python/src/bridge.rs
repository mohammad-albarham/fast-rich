//! Convert a "spec tree" built by the Python layer into native renderables.
//!
//! The Python package keeps Rich's object model (Text, Panel, Table, ...).
//! At print time `fast_rich.console` turns each object into a spec: a tuple
//! whose first item names the kind, e.g. `("panel", child, {...})`. This
//! module builds the matching `fast_rich` renderable so all layout and ANSI
//! output happens in Rust. Objects that implement Rich's `__rich_console__`
//! become a [`PyCallback`] that asks Python for their content at render
//! time, with the real width.

use std::cell::RefCell;
use std::sync::Arc;

use fast_rich::align::{Align, VerticalAlignment};
use fast_rich::box_drawing::{BoxStyle, Line};
use fast_rich::columns::{ColumnMode, Columns};
use fast_rich::console::RenderContext;
use fast_rich::constrain::Constrain;
use fast_rich::markdown::Markdown;
use fast_rich::padding::{Padding, PaddingSpec};
use fast_rich::panel::{BorderStyle, Panel};
use fast_rich::renderable::{segments_to_lines, Renderable, Segment, SharedRenderable};
use fast_rich::rule::Rule;
use fast_rich::style::{Color, Style};
use fast_rich::syntax::{Syntax, SyntaxConfig, Theme as SyntaxTheme};
use fast_rich::table::{Cell, Column, ColumnWidth, Row, Table};
use fast_rich::text::{Alignment, Overflow, Span, Text};
use fast_rich::tree::{GuideStyle, Tree, TreeNode};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

thread_local! {
    /// First Python error raised inside a render callback; `take_error`
    /// re-raises it once rendering returns to Python.
    static RENDER_ERROR: RefCell<Option<PyErr>> = const { RefCell::new(None) };
}

fn set_error(err: PyErr) {
    RENDER_ERROR.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(err);
        }
    });
}

/// Return (and clear) an error raised during the last render.
pub fn take_error() -> Option<PyErr> {
    RENDER_ERROR.with(|slot| slot.borrow_mut().take())
}

/// Parse a style string produced by `fast_rich.style.Style._native()`.
pub fn style(s: &str) -> Style {
    if s.is_empty() {
        Style::new()
    } else {
        Style::parse(s)
    }
}

fn color_name(color: &Color) -> String {
    match color {
        Color::Default => "default".into(),
        Color::Black => "black".into(),
        Color::Red => "red".into(),
        Color::Green => "green".into(),
        Color::Yellow => "yellow".into(),
        Color::Blue => "blue".into(),
        Color::Magenta => "magenta".into(),
        Color::Cyan => "cyan".into(),
        Color::White => "white".into(),
        Color::BrightBlack => "bright_black".into(),
        Color::BrightRed => "bright_red".into(),
        Color::BrightGreen => "bright_green".into(),
        Color::BrightYellow => "bright_yellow".into(),
        Color::BrightBlue => "bright_blue".into(),
        Color::BrightMagenta => "bright_magenta".into(),
        Color::BrightCyan => "bright_cyan".into(),
        Color::BrightWhite => "bright_white".into(),
        Color::Ansi256(n) => format!("color({n})"),
        Color::Rgb { r, g, b } => format!("#{r:02x}{g:02x}{b:02x}"),
    }
}

/// Format a crate style the way Rich writes styles (`"bold red on blue"`).
pub fn style_to_string(st: &Style) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (on, name) in [
        (st.bold, "bold"),
        (st.dim, "dim"),
        (st.italic, "italic"),
        (st.underline, "underline"),
        (st.blink, "blink"),
        (st.reverse, "reverse"),
        (st.hidden, "conceal"),
        (st.strikethrough, "strike"),
    ] {
        if on {
            parts.push(name.into());
        }
    }
    if let Some(fg) = &st.foreground {
        parts.push(color_name(fg));
    }
    if let Some(bg) = &st.background {
        parts.push(format!("on {}", color_name(bg)));
    }
    parts.join(" ")
}

fn kind_err(kind: &str) -> PyErr {
    PyValueError::new_err(format!("fast_rich: unknown renderable spec kind {kind:?}"))
}

fn get<'py, T: FromPyObject<'py>>(opts: &Bound<'py, PyDict>, key: &str, default: T) -> PyResult<T> {
    match opts.get_item(key)? {
        Some(v) if !v.is_none() => v.extract(),
        _ => Ok(default),
    }
}

fn get_opt<'py, T: FromPyObject<'py>>(opts: &Bound<'py, PyDict>, key: &str) -> PyResult<Option<T>> {
    match opts.get_item(key)? {
        Some(v) if !v.is_none() => Ok(Some(v.extract()?)),
        _ => Ok(None),
    }
}

fn alignment(s: &str) -> Alignment {
    match s {
        "center" => Alignment::Center,
        "right" => Alignment::Right,
        _ => Alignment::Left,
    }
}

/// Build a crate box from Rich's 8-line box definition.
fn box_style(def: &str) -> BoxStyle {
    let rows: Vec<Vec<char>> = def.lines().map(|l| l.chars().collect()).collect();
    let line = |i: usize| {
        let r = rows.get(i).cloned().unwrap_or_default();
        let c = |j: usize| r.get(j).copied().unwrap_or(' ');
        // Rich rows are "left, horizontal, divider, right".
        Line::new(c(0), c(1), c(3), c(2))
    };
    BoxStyle {
        top: line(0),
        header: line(1),
        head: line(2),
        cell: line(3),
        mid: line(4),
        bottom: line(7),
    }
}

/// Build a crate `Text` from Python's `("text", plain, spans, style, justify, overflow, no_wrap)`.
fn build_text(t: &Bound<'_, PyTuple>) -> PyResult<Text> {
    let plain: String = t.get_item(1)?.extract()?;
    let spans: Vec<(usize, usize, String, Option<String>)> = t.get_item(2)?.extract()?;
    let base: String = t.get_item(3)?.extract()?;
    let justify: Option<String> = t.get_item(4)?.extract()?;
    let overflow: Option<String> = t.get_item(5)?.extract()?;
    let no_wrap: Option<bool> = t.get_item(6)?.extract()?;

    let chars: Vec<char> = plain.chars().collect();
    let n = chars.len();
    let base = style(&base);
    let parsed: Vec<(usize, usize, Style, Option<String>)> = spans
        .into_iter()
        .filter_map(|(s, e, st, link)| {
            let (s, e) = (s.min(n), e.min(n));
            (s < e).then(|| (s, e, style(&st), link))
        })
        .collect();

    // Cut the text at every span boundary; later spans win, like Rich.
    let mut cuts: Vec<usize> = vec![0, n];
    for (s, e, _, _) in &parsed {
        cuts.push(*s);
        cuts.push(*e);
    }
    cuts.sort_unstable();
    cuts.dedup();
    let mut out = Text::new();
    for w in cuts.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a >= b {
            continue;
        }
        let mut st = base;
        let mut link = None;
        for (s, e, sp, l) in &parsed {
            if *s <= a && b <= *e {
                st = st.combine(sp);
                if l.is_some() {
                    link = l.clone();
                }
            }
        }
        let text: String = chars[a..b].iter().collect();
        out.push_span(match link {
            Some(url) => Span::linked(text, st, url),
            None => Span::styled(text, st),
        });
    }
    out.alignment = alignment(justify.as_deref().unwrap_or("left"));
    out.overflow = match (no_wrap.unwrap_or(false), overflow.as_deref()) {
        (true, Some("ellipsis")) => Overflow::Ellipsis,
        (true, Some("ignore")) => Overflow::Visible,
        (true, _) => Overflow::Truncate,
        (false, Some("ellipsis")) => Overflow::WrapEllipsis,
        (false, Some("crop")) => Overflow::WrapCrop,
        (false, _) => Overflow::Wrap,
    };
    Ok(out)
}

/// A `__rich_console__` object rendered by calling back into Python.
pub struct PyCallback {
    obj: Py<PyAny>,
    console: Py<PyAny>,
    measure: (usize, usize),
}

impl PyCallback {
    fn new(py: Python<'_>, obj: Py<PyAny>, console: Py<PyAny>) -> Self {
        // (min, max) from `__rich_measure__` when the object defines it.
        let measure = console
            .bind(py)
            .call_method1("_measure_callback", (obj.bind(py),))
            .and_then(|m| m.extract::<(usize, usize)>())
            .unwrap_or_else(|e| {
                set_error(e);
                (1, usize::MAX)
            });
        PyCallback {
            obj,
            console,
            measure,
        }
    }
}

impl Renderable for PyCallback {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        Python::with_gil(|py| {
            let result = self
                .console
                .bind(py)
                .call_method1("_render_callback", (self.obj.bind(py), context.width))
                .and_then(|spec| build(py, &spec, &self.console));
            match result {
                Ok(native) => native.render(context),
                Err(e) => {
                    set_error(e);
                    Vec::new()
                }
            }
        })
    }

    fn min_width(&self) -> usize {
        self.measure.0
    }

    fn max_width(&self) -> usize {
        self.measure.1
    }
}

/// Raw segments, `(text, style, link)`, split into lines at `\n`.
struct Segments(Vec<Segment>);

impl Renderable for Segments {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        self.0.clone()
    }

    fn min_width(&self) -> usize {
        self.max_width()
    }

    fn max_width(&self) -> usize {
        segments_to_lines(self.0.clone())
            .iter()
            .map(|l| fast_rich::renderable::line_width(l))
            .max()
            .unwrap_or(0)
    }
}

/// Children rendered one after another (Rich's `Group`).
struct Group(Vec<SharedRenderable>);

impl Renderable for Group {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let mut out = Vec::new();
        for child in &self.0 {
            for line in segments_to_lines(child.render(context)) {
                out.push(Segment::line(line));
            }
        }
        out
    }

    fn min_width(&self) -> usize {
        self.0.iter().map(|c| c.min_width()).max().unwrap_or(0)
    }

    fn max_width(&self) -> usize {
        self.0.iter().map(|c| c.max_width()).max().unwrap_or(0)
    }
}

/// A child with a base style applied under its own styles (Rich's `Styled`).
struct Styled(SharedRenderable, Style);

impl Renderable for Styled {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        self.0
            .render(context)
            .into_iter()
            .map(|mut seg| {
                for span in &mut seg.spans {
                    span.style = self.1.combine(&span.style);
                }
                seg
            })
            .collect()
    }

    fn min_width(&self) -> usize {
        self.0.min_width()
    }

    fn max_width(&self) -> usize {
        self.0.max_width()
    }
}

fn build_children(py: Python<'_>, items: &Bound<'_, PyAny>, console: &Py<PyAny>) -> PyResult<Vec<SharedRenderable>> {
    let list = items.downcast::<PyList>()?;
    list.iter().map(|item| build(py, &item, console)).collect()
}

fn build_tree_node(py: Python<'_>, node: &Bound<'_, PyAny>, console: &Py<PyAny>) -> PyResult<TreeNode> {
    let node = node.downcast::<PyTuple>()?;
    let label = build(py, &node.get_item(0)?, console)?;
    let st: String = node.get_item(1)?.extract()?;
    let expanded: bool = node.get_item(2)?.extract()?;
    let mut out = TreeNode::renderable(label).style(style(&st)).expanded(expanded);
    for child in node.get_item(3)?.downcast::<PyList>()?.iter() {
        out.add(build_tree_node(py, &child, console)?);
    }
    Ok(out)
}

/// Build a native renderable from a spec.
pub fn build(py: Python<'_>, spec: &Bound<'_, PyAny>, console: &Py<PyAny>) -> PyResult<SharedRenderable> {
    let t = spec.downcast::<PyTuple>()?;
    let kind: String = t.get_item(0)?.extract()?;
    let native: SharedRenderable = match kind.as_str() {
        "text" => Arc::new(build_text(t)?),
        "segments" => {
            let items: Vec<(String, String, Option<String>)> = t.get_item(1)?.extract()?;
            let spans: Vec<Span> = items
                .into_iter()
                .map(|(text, st, link)| match link {
                    Some(url) => Span::linked(text, style(&st), url),
                    None => Span::styled(text, style(&st)),
                })
                .collect();
            let lines = segments_to_lines(vec![Segment::new(spans)]);
            Arc::new(Segments(lines.into_iter().map(Segment::line).collect()))
        }
        "group" => Arc::new(Group(build_children(py, &t.get_item(1)?, console)?)),
        "styled" => {
            let child = build(py, &t.get_item(1)?, console)?;
            let st: String = t.get_item(2)?.extract()?;
            Arc::new(Styled(child, style(&st)))
        }
        "callback" => Arc::new(PyCallback::new(py, t.get_item(1)?.unbind(), console.clone_ref(py))),
        "panel" => {
            let child = build(py, &t.get_item(1)?, console)?;
            let opts = t.get_item(2)?;
            let opts = opts.downcast::<PyDict>()?;
            let mut p = Panel::renderable(child)
                .border_style(BorderStyle::Custom(box_style(&get::<String>(opts, "box", String::new())?)))
                .expand(get(opts, "expand", true)?)
                .title_align(alignment(&get::<String>(opts, "title_align", "center".into())?))
                .subtitle_align(alignment(&get::<String>(opts, "subtitle_align", "center".into())?))
                .style(style(&get::<String>(opts, "border_style", String::new())?))
                .content_style(style(&get::<String>(opts, "style", String::new())?));
            let (top, right, bottom, left): (usize, usize, usize, usize) = get(opts, "padding", (0, 1, 0, 1))?;
            p = p.padding_spec(PaddingSpec::new(top, right, bottom, left));
            if let Some(w) = get_opt::<usize>(opts, "width")? {
                p = p.width(w);
            }
            if let Some(h) = get_opt::<usize>(opts, "height")? {
                p = p.height(h);
            }
            if let Some(title) = opts.get_item("title")? {
                if !title.is_none() {
                    p = p.title_text(build_text(title.downcast::<PyTuple>()?)?);
                }
            }
            if let Some(sub) = opts.get_item("subtitle")? {
                if !sub.is_none() {
                    p = p.subtitle_text(build_text(sub.downcast::<PyTuple>()?)?);
                }
            }
            Arc::new(p)
        }
        "table" => {
            let opts = t.get_item(1)?;
            let opts = opts.downcast::<PyDict>()?;
            let mut table = Table::new()
                .show_header(get(opts, "show_header", true)?)
                .show_edge(get(opts, "show_edge", true)?)
                .show_row_lines(get(opts, "show_lines", false)?)
                .pad_edge(get(opts, "pad_edge", true)?)
                .collapse_padding(get(opts, "collapse_padding", false)?)
                .expand(get(opts, "expand", false)?)
                .style(style(&get::<String>(opts, "border_style", String::new())?))
                .header_style(style(&get::<String>(opts, "header_style", String::new())?))
                .title_style(style(&get::<String>(opts, "title_style", "italic".into())?))
                .caption_style(style(&get::<String>(opts, "caption_style", "italic dim".into())?))
                .title_align(alignment(&get::<String>(opts, "title_justify", "center".into())?))
                .caption_align(alignment(&get::<String>(opts, "caption_justify", "center".into())?));
            let (pad_left, pad_right): (usize, usize) = get(opts, "padding", (1, 1))?;
            table = table.padding_sides(pad_left, pad_right);
            match get_opt::<String>(opts, "box")? {
                Some(b) => table = table.border_style(BorderStyle::Custom(box_style(&b))),
                None => table = table.show_border(false),
            }
            if let Some(w) = get_opt::<usize>(opts, "width")? {
                table = table.width(w);
            }
            if let Some(w) = get_opt::<usize>(opts, "min_width")? {
                table = table.min_width(w);
            }
            if let Some(styles) = get_opt::<Vec<String>>(opts, "row_styles")? {
                table = table.row_styles(styles.iter().map(|s| style(s)).collect());
            }
            if let Some(title) = opts.get_item("title")? {
                if !title.is_none() {
                    table = table.title_text(build_text(title.downcast::<PyTuple>()?)?);
                }
            }
            if let Some(caption) = opts.get_item("caption")? {
                if !caption.is_none() {
                    table = table.caption_text(build_text(caption.downcast::<PyTuple>()?)?);
                }
            }
            for col in t.get_item(2)?.downcast::<PyList>()?.iter() {
                let col = col.downcast::<PyTuple>()?;
                let header = col.get_item(0)?;
                let copts = col.get_item(1)?;
                let copts = copts.downcast::<PyDict>()?;
                let mut c = Column::new("")
                    .align(alignment(&get::<String>(copts, "justify", "left".into())?))
                    .style(style(&get::<String>(copts, "style", String::new())?))
                    .header_style(style(&get::<String>(copts, "header_style", String::new())?))
                    .no_wrap(get(copts, "no_wrap", false)?)
                    .overflow(match get::<String>(copts, "overflow", "ellipsis".into())?.as_str() {
                        "crop" => Overflow::Truncate,
                        "fold" => Overflow::Wrap,
                        "ignore" => Overflow::Visible,
                        _ => Overflow::Ellipsis,
                    });
                let header_t = header.downcast::<PyTuple>()?;
                let header_kind: String = header_t.get_item(0)?.extract()?;
                c = if header_kind == "text" {
                    c.header_text(build_text(header_t)?)
                } else {
                    c.header_renderable(build(py, &header, console)?)
                };
                if let Some(w) = get_opt::<usize>(copts, "width")? {
                    c = c.width(ColumnWidth::Fixed(w));
                } else if let Some(w) = get_opt::<usize>(copts, "min_width")? {
                    c = c.width(ColumnWidth::Min(w));
                } else if let Some(w) = get_opt::<usize>(copts, "max_width")? {
                    c = c.width(ColumnWidth::Max(w));
                }
                if let Some(r) = get_opt::<usize>(copts, "ratio")? {
                    c = c.ratio(r);
                }
                table.add_column(c);
            }
            for row in t.get_item(3)?.downcast::<PyList>()?.iter() {
                let row = row.downcast::<PyTuple>()?;
                let mut cells = Vec::new();
                for cell in row.get_item(0)?.downcast::<PyList>()?.iter() {
                    let ct = cell.downcast::<PyTuple>()?;
                    let ck: String = ct.get_item(0)?.extract()?;
                    cells.push(if ck == "text" {
                        Cell::Text(build_text(ct)?)
                    } else {
                        Cell::Renderable(build(py, &cell, console)?)
                    });
                }
                let ropts = row.get_item(1)?;
                let ropts = ropts.downcast::<PyDict>()?;
                let mut r = Row::cells(cells).end_section(get(ropts, "end_section", false)?);
                if let Some(st) = get_opt::<String>(ropts, "style")? {
                    r = r.style(style(&st));
                }
                table.add_row_obj(r);
            }
            Arc::new(table)
        }
        "tree" => {
            let root = build_tree_node(py, &t.get_item(1)?, console)?;
            let opts = t.get_item(2)?;
            let opts = opts.downcast::<PyDict>()?;
            let guide = match get::<String>(opts, "guide", "unicode".into())?.as_str() {
                "bold" => GuideStyle::Bold,
                "double" => GuideStyle::Double,
                "ascii" => GuideStyle::Ascii,
                _ => GuideStyle::Unicode,
            };
            Arc::new(
                Tree::new(root)
                    .guide_style(guide)
                    .style(style(&get::<String>(opts, "guide_style", String::new())?))
                    .hide_root(get(opts, "hide_root", false)?),
            )
        }
        "rule" => {
            let opts = t.get_item(1)?;
            let opts = opts.downcast::<PyDict>()?;
            let mut rule = Rule::line()
                .characters(&get::<String>(opts, "characters", "─".into())?)
                .style(style(&get::<String>(opts, "style", String::new())?))
                .align(alignment(&get::<String>(opts, "align", "center".into())?));
            if let Some(title) = opts.get_item("title")? {
                if !title.is_none() {
                    rule = rule.title_text(build_text(title.downcast::<PyTuple>()?)?);
                }
            }
            Arc::new(rule)
        }
        "align" => {
            let child = build(py, &t.get_item(1)?, console)?;
            let opts = t.get_item(2)?;
            let opts = opts.downcast::<PyDict>()?;
            let mut a = Align::new(child, alignment(&get::<String>(opts, "align", "left".into())?))
                .pad(get(opts, "pad", true)?)
                .vertical(match get::<String>(opts, "vertical", "top".into())?.as_str() {
                    "middle" => VerticalAlignment::Middle,
                    "bottom" => VerticalAlignment::Bottom,
                    _ => VerticalAlignment::Top,
                });
            if let Some(h) = get_opt::<usize>(opts, "height")? {
                a = a.height(h);
            }
            Arc::new(a)
        }
        "padding" => {
            let child = build(py, &t.get_item(1)?, console)?;
            let opts = t.get_item(2)?;
            let opts = opts.downcast::<PyDict>()?;
            let (top, right, bottom, left): (usize, usize, usize, usize) = get(opts, "pad", (0, 0, 0, 0))?;
            Arc::new(
                Padding::new(child, PaddingSpec::new(top, right, bottom, left))
                    .expand(get(opts, "expand", true)?)
                    .style(style(&get::<String>(opts, "style", String::new())?)),
            )
        }
        "constrain" => {
            let child = build(py, &t.get_item(1)?, console)?;
            let width: Option<usize> = t.get_item(2)?.extract()?;
            Arc::new(Constrain::new(child, width))
        }
        "columns" => {
            let items = build_children(py, &t.get_item(1)?, console)?;
            let opts = t.get_item(2)?;
            let opts = opts.downcast::<PyDict>()?;
            let mut c = Columns::renderables(items)
                .gap(get(opts, "gap", 1)?)
                .expand(get(opts, "expand", false)?)
                .column_first(get(opts, "column_first", false)?)
                .right_to_left(get(opts, "right_to_left", false)?)
                .mode(if get(opts, "equal", false)? {
                    ColumnMode::Equal
                } else {
                    ColumnMode::Fit
                });
            if let Some(a) = get_opt::<String>(opts, "align")? {
                c = c.align(alignment(&a));
            }
            if let Some(w) = get_opt::<usize>(opts, "width")? {
                c = c.width(w);
            }
            Arc::new(c)
        }
        "markdown" => {
            let source: String = t.get_item(1)?.extract()?;
            Arc::new(Markdown::new(&source))
        }
        "syntax" => {
            let code: String = t.get_item(1)?.extract()?;
            let lexer: String = t.get_item(2)?.extract()?;
            let opts = t.get_item(3)?;
            let opts = opts.downcast::<PyDict>()?;
            let theme = match get::<String>(opts, "theme", "monokai".into())?.as_str() {
                "github" | "github-dark" | "inspiredgithub" | "default" => SyntaxTheme::InspiredGitHub,
                "solarized-dark" => SyntaxTheme::SolarizedDark,
                "solarized-light" => SyntaxTheme::SolarizedLight,
                "ocean" | "base16-ocean.dark" => SyntaxTheme::Base16OceanDark,
                _ => SyntaxTheme::Monokai,
            };
            let config = SyntaxConfig {
                line_numbers: get(opts, "line_numbers", false)?,
                panel: false,
                theme,
                start_line: get(opts, "start_line", 1)?,
                highlight_lines: get(opts, "highlight_lines", Vec::new())?,
                ..SyntaxConfig::default()
            };
            Arc::new(Syntax::new(&code, &lexer).config(config))
        }
        other => return Err(kind_err(other)),
    };
    Ok(native)
}
