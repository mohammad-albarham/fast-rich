//! Pretty printing: a port of Python Rich's `rich.pretty`.
//!
//! Rust has no runtime reflection, so the input is a value's compact
//! [`Debug`] output (`format!("{:?}")`). It is parsed into a tree of nodes
//! (lists, tuples, maps, sets, structs, tuple structs and atoms) and laid
//! out with Rich's algorithm: a container stays on one line if it fits the
//! width, otherwise it is expanded one child per line, indented by
//! `indent_size`, and the same check runs again for every child.
//!
//! ```
//! use fast_rich::pretty::{pretty_repr, PrettyOptions};
//!
//! let data = vec![("alpha", vec![1, 2]), ("beta", vec![3])];
//! let narrow = pretty_repr(&data, &PrettyOptions::default().max_width(24));
//! assert_eq!(
//!     narrow,
//!     "[\n    (\"alpha\", [1, 2]),\n    (\"beta\", [3])\n]"
//! );
//! ```
//!
//! Three entry points:
//!
//! - [`pretty_repr`] returns the laid-out string (no styles).
//! - [`Pretty`] is a [`Renderable`] that fits the render width, highlights
//!   with [`ReprHighlighter`] and can draw indent guides.
//! - [`pprint`] prints a [`Pretty`] to stdout.
//!
//! [`Console::print_debug`](crate::Console::print_debug) and
//! [`inspect`](crate::inspect::inspect) use [`Pretty`].
//!
//! # Differences from Rich
//!
//! - Values are shown with Rust's `Debug` syntax, not Python's `repr`:
//!   strings use `"` quotes, booleans are `true`/`false`, an empty set is
//!   `{}`.
//! - Structs keep Rust syntax (`Point { x: 1, y: 2 }`), expanded like
//!   `{:#?}` but only when they don't fit. Tuple structs and enum variants
//!   (`Some(1)`, `Rgb(1, 2, 3)`) are laid out like Rich lays out a
//!   namedtuple. Rich has no equivalent of Rust structs, so these follow
//!   Rich's dataclass rules: `max_length` does not abbreviate their fields
//!   and past `max_depth` they become `Name { ... }` / `Name(...)`.
//! - As in Rich, an expanded container has a comma after every child but
//!   the last (`{:#?}` also puts one after the last).
//! - Anything the parser does not recognize (custom `Debug` output such as
//!   `1.5s` or `PhantomData<u8>`) is kept verbatim as one atom. If the whole
//!   output is malformed (unbalanced brackets, an unterminated string) it is
//!   shown as one atom.
//! - Containers nested deeper than 256 levels are kept as one atom of raw
//!   text, so very deep values cannot overflow the stack.

use std::fmt::Debug;
use std::sync::Arc;

use unicode_width::UnicodeWidthStr;

use crate::console::RenderContext;
use crate::highlighter::{Highlighter, ReprHighlighter};
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Overflow, Text};
use crate::theme::Theme;

/// Containers nested deeper than this are kept as raw text, which bounds
/// the recursion of the layout code.
const MAX_PARSE_DEPTH: usize = 256;

// ============================================================================
// Options
// ============================================================================

/// Layout options for [`pretty_repr`], [`Pretty`] and [`pprint`], named
/// after Rich's arguments.
///
/// ```
/// use fast_rich::pretty::{pretty_repr, PrettyOptions};
///
/// let options = PrettyOptions::default().max_length(2);
/// assert_eq!(pretty_repr(&vec![1, 2, 3, 4], &options), "[1, 2, ... +2]");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrettyOptions {
    /// Width to fit the output into, in terminal cells (default 80).
    /// [`Pretty`] and [`pprint`] use the render width instead.
    pub max_width: usize,
    /// Spaces per indent level (default 4).
    pub indent_size: usize,
    /// Show at most this many children of a list, tuple, map or set, then
    /// `... +K` for the rest.
    pub max_length: Option<usize>,
    /// Crop strings longer than this many characters, marking the cut as
    /// `"abc"+5` (5 more characters).
    pub max_string: Option<usize>,
    /// Containers at this depth or deeper become `[...]`, `{...}`, `(...)`
    /// (the outermost value is depth 0).
    pub max_depth: Option<usize>,
    /// Expand every container, even ones that fit.
    pub expand_all: bool,
    /// Draw indent guides (used by [`Pretty`] and [`pprint`], not by
    /// [`pretty_repr`]).
    pub indent_guides: bool,
}

impl Default for PrettyOptions {
    fn default() -> Self {
        PrettyOptions {
            max_width: 80,
            indent_size: 4,
            max_length: None,
            max_string: None,
            max_depth: None,
            expand_all: false,
            indent_guides: false,
        }
    }
}

impl PrettyOptions {
    /// Same as [`PrettyOptions::default`].
    ///
    /// ```
    /// use fast_rich::pretty::PrettyOptions;
    /// assert_eq!(PrettyOptions::new().indent_size, 4);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set [`max_width`](Self::max_width).
    ///
    /// ```
    /// use fast_rich::pretty::{pretty_repr, PrettyOptions};
    /// let out = pretty_repr(&[1, 2], &PrettyOptions::new().max_width(3));
    /// assert_eq!(out, "[\n    1,\n    2\n]");
    /// ```
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }

    /// Set [`indent_size`](Self::indent_size).
    ///
    /// ```
    /// use fast_rich::pretty::{pretty_repr, PrettyOptions};
    /// let options = PrettyOptions::new().max_width(0).indent_size(2);
    /// assert_eq!(pretty_repr(&[1], &options), "[\n  1\n]");
    /// ```
    pub fn indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    /// Set [`max_length`](Self::max_length).
    ///
    /// ```
    /// use fast_rich::pretty::{pretty_repr, PrettyOptions};
    /// let out = pretty_repr(&vec![1, 2, 3], &PrettyOptions::new().max_length(0));
    /// assert_eq!(out, "[... +3]");
    /// ```
    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// Set [`max_string`](Self::max_string).
    ///
    /// ```
    /// use fast_rich::pretty::{pretty_repr, PrettyOptions};
    /// let out = pretty_repr(&"abcdef", &PrettyOptions::new().max_string(2));
    /// assert_eq!(out, "\"ab\"+4");
    /// ```
    pub fn max_string(mut self, length: usize) -> Self {
        self.max_string = Some(length);
        self
    }

    /// Set [`max_depth`](Self::max_depth).
    ///
    /// ```
    /// use fast_rich::pretty::{pretty_repr, PrettyOptions};
    /// let out = pretty_repr(&vec![vec![1], vec![]], &PrettyOptions::new().max_depth(1));
    /// assert_eq!(out, "[[...], [...]]");
    /// ```
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Set [`expand_all`](Self::expand_all).
    ///
    /// ```
    /// use fast_rich::pretty::{pretty_repr, PrettyOptions};
    /// let out = pretty_repr(&[1], &PrettyOptions::new().expand_all(true));
    /// assert_eq!(out, "[\n    1\n]");
    /// ```
    pub fn expand_all(mut self, expand: bool) -> Self {
        self.expand_all = expand;
        self
    }

    /// Set [`indent_guides`](Self::indent_guides).
    ///
    /// ```
    /// use fast_rich::pretty::PrettyOptions;
    /// assert!(PrettyOptions::new().indent_guides(true).indent_guides);
    /// ```
    pub fn indent_guides(mut self, guides: bool) -> Self {
        self.indent_guides = guides;
        self
    }
}

// ============================================================================
// Parsed Debug output
// ============================================================================

/// A value parsed from `Debug` output.
#[derive(Debug, Clone, PartialEq)]
enum Value {
    /// Anything shown verbatim: numbers, `None`, chars, unit structs,
    /// unrecognized text.
    Atom(String),
    /// A string literal, quotes included (`"a\n"`).
    Str(String),
    Container(Box<Container>),
}

#[derive(Debug, Clone, PartialEq)]
struct Container {
    kind: Kind,
    items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    /// `[..]`
    List,
    /// `(..)`
    Tuple,
    /// `{..}`: a map (items have keys) or a set.
    Brace,
    /// `Name { field: value, .. }`
    Struct(String),
    /// `Name(..)`: tuple structs and enum variants.
    TupleStruct(String),
}

#[derive(Debug, Clone, PartialEq)]
struct Item {
    key: Option<Value>,
    value: Value,
}

/// A lexical piece of one item, with byte spans into the source.
enum Piece {
    Text(usize, usize),
    Str(usize, usize),
    Char(usize, usize),
    Group {
        open: u8,
        items: Vec<Item>,
        start: usize,
        end: usize,
    },
}

impl Piece {
    fn span(&self) -> (usize, usize) {
        match *self {
            Piece::Text(a, b) | Piece::Str(a, b) | Piece::Char(a, b) => (a, b),
            Piece::Group { start, end, .. } => (start, end),
        }
    }
}

/// An open bracket being parsed (the root frame has `open == 0`).
struct Frame {
    open: u8,
    start: usize,
    items: Vec<Item>,
    cur: Vec<Piece>,
    key: Option<Value>,
}

impl Frame {
    fn new(open: u8, start: usize) -> Self {
        Frame {
            open,
            start,
            items: Vec::new(),
            cur: Vec::new(),
            key: None,
        }
    }

    fn push_text(&mut self, start: usize, end: usize) {
        if let Some(Piece::Text(_, b)) = self.cur.last_mut() {
            if *b == start {
                *b = end;
                return;
            }
        }
        self.cur.push(Piece::Text(start, end));
    }
}

/// Parse `Debug` output. Never panics; malformed input becomes one atom.
fn parse(src: &str) -> Value {
    parse_inner(src).unwrap_or_else(|| Value::Atom(src.to_string()))
}

fn closer(open: u8) -> u8 {
    match open {
        b'[' => b']',
        b'(' => b')',
        _ => b'}',
    }
}

/// End (exclusive) of the string literal starting at `i` (a `"`).
fn scan_string(bytes: &[u8], i: usize) -> Option<usize> {
    let mut j = i + 1;
    while j < bytes.len() {
        match bytes[j] {
            b'\\' => j += 2,
            b'"' => return Some(j + 1),
            _ => j += 1,
        }
    }
    None
}

/// End (exclusive) of a char literal starting at `i` (a `'`), if it is one.
fn scan_char(src: &str, i: usize) -> Option<usize> {
    let rest = &src[i + 1..];
    let mut chars = rest.char_indices();
    let (_, c) = chars.next()?;
    if c == '\\' {
        // `'\n'`, `'\''`, `'\u{1f600}'`: the closing quote comes after the
        // escaped character.
        let (_, escaped) = chars.next()?;
        let after = 1 + escaped.len_utf8();
        let close = rest.get(after..)?.find('\'')?;
        if close > 10 {
            return None;
        }
        Some(i + 1 + after + close + 1)
    } else if c == '\'' {
        None
    } else {
        rest[c.len_utf8()..]
            .starts_with('\'')
            .then(|| i + 1 + c.len_utf8() + 1)
    }
}

fn parse_inner(src: &str) -> Option<Value> {
    let bytes = src.as_bytes();
    let mut frames = vec![Frame::new(0, 0)];
    // Raw-text mode for containers deeper than MAX_PARSE_DEPTH: the
    // expected closing brackets, and where the raw group started.
    let mut skip: Vec<u8> = Vec::new();
    let mut skip_start = 0;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        // Strings and chars are tokens in both modes, so brackets inside
        // them never count.
        if b == b'"' {
            let end = scan_string(bytes, i)?;
            if skip.is_empty() {
                frames.last_mut()?.cur.push(Piece::Str(i, end));
            }
            i = end;
            continue;
        }
        if b == b'\'' {
            if let Some(end) = scan_char(src, i) {
                if skip.is_empty() {
                    frames.last_mut()?.cur.push(Piece::Char(i, end));
                }
                i = end;
                continue;
            }
        }

        if !skip.is_empty() {
            match b {
                b'[' | b'(' | b'{' => skip.push(closer(b)),
                b']' | b')' | b'}' => {
                    if skip.pop() != Some(b) {
                        return None;
                    }
                    if skip.is_empty() {
                        frames.last_mut()?.push_text(skip_start, i + 1);
                    }
                }
                _ => {}
            }
            i += 1;
            continue;
        }

        match b {
            b'[' | b'(' | b'{' => {
                if frames.len() > MAX_PARSE_DEPTH {
                    skip.push(closer(b));
                    skip_start = i;
                } else {
                    frames.push(Frame::new(b, i));
                }
            }
            b']' | b')' | b'}' => {
                let frame = frames.pop()?;
                if frame.open == 0 || closer(frame.open) != b {
                    return None;
                }
                let Frame {
                    open,
                    start,
                    mut items,
                    cur,
                    key,
                } = frame;
                if has_content(src, &cur) {
                    items.push(Item {
                        key,
                        value: pieces_to_value(src, cur),
                    });
                } else if key.is_some() {
                    return None;
                }
                frames.last_mut()?.cur.push(Piece::Group {
                    open,
                    items,
                    start,
                    end: i + 1,
                });
            }
            b',' => {
                let frame = frames.last_mut()?;
                if frame.open == 0 || !has_content(src, &frame.cur) {
                    return None;
                }
                let cur = std::mem::take(&mut frame.cur);
                let key = frame.key.take();
                let value = pieces_to_value(src, cur);
                frame.items.push(Item { key, value });
            }
            b':' if bytes.get(i + 1) == Some(&b' ') && (i == 0 || bytes[i - 1] != b':') => {
                let frame = frames.last_mut()?;
                if frame.open == b'{' && frame.key.is_none() && has_content(src, &frame.cur) {
                    let cur = std::mem::take(&mut frame.cur);
                    frame.key = Some(pieces_to_value(src, cur));
                    i += 2;
                    continue;
                }
                frame.push_text(i, i + 1);
            }
            _ => {
                // Copy a run of plain bytes at once (all multi-byte UTF-8
                // bytes are >= 0x80, so `j` stays on a char boundary).
                let mut j = i + 1;
                while j < bytes.len()
                    && !matches!(
                        bytes[j],
                        b'"' | b'\'' | b'[' | b'(' | b'{' | b']' | b')' | b'}' | b',' | b':'
                    )
                {
                    j += 1;
                }
                frames.last_mut()?.push_text(i, j);
                i = j;
                continue;
            }
        }
        i += 1;
    }

    if !skip.is_empty() || frames.len() != 1 {
        return None;
    }
    let root = frames.pop()?;
    if !root.items.is_empty() || root.key.is_some() {
        return None;
    }
    if !has_content(src, &root.cur) {
        return Some(Value::Atom(src.to_string()));
    }
    Some(pieces_to_value(src, root.cur))
}

fn is_blank(src: &str, piece: &Piece) -> bool {
    matches!(*piece, Piece::Text(a, b) if src[a..b].trim().is_empty())
}

fn has_content(src: &str, pieces: &[Piece]) -> bool {
    pieces.iter().any(|p| !is_blank(src, p))
}

fn is_name(name: &str) -> bool {
    !name.is_empty() && !name.contains(char::is_whitespace)
}

/// Turn the pieces of one item into a value. Unrecognized shapes become
/// an atom of their (trimmed) source text.
fn pieces_to_value(src: &str, pieces: Vec<Piece>) -> Value {
    let start = pieces.first().map_or(0, |p| p.span().0);
    let end = pieces.last().map_or(0, |p| p.span().1);
    let raw = || Value::Atom(src[start..end].trim().to_string());

    let mut sig: Vec<Piece> = pieces.into_iter().filter(|p| !is_blank(src, p)).collect();
    match sig.len() {
        1 => match sig.pop() {
            Some(Piece::Str(a, b)) => Value::Str(src[a..b].to_string()),
            Some(Piece::Text(a, b)) | Some(Piece::Char(a, b)) => {
                Value::Atom(src[a..b].trim().to_string())
            }
            Some(Piece::Group { open, items, .. }) => {
                let kind = match open {
                    b'[' => Kind::List,
                    b'(' => Kind::Tuple,
                    _ => Kind::Brace,
                };
                Value::Container(Box::new(Container { kind, items }))
            }
            None => raw(),
        },
        2 => {
            let group = sig.pop();
            let name = sig.pop();
            match (name, group) {
                (
                    Some(Piece::Text(a, b)),
                    Some(Piece::Group {
                        open, items, start, ..
                    }),
                ) => {
                    let name = src[a..b].trim();
                    let kind = match open {
                        // `Some(1)`: no space before the parenthesis.
                        b'(' if b == start && is_name(name) => Kind::TupleStruct(name.into()),
                        b'{' if is_name(name) => Kind::Struct(name.into()),
                        _ => return raw(),
                    };
                    Value::Container(Box::new(Container { kind, items }))
                }
                _ => raw(),
            }
        }
        _ => raw(),
    }
}

// ============================================================================
// Layout (rich/pretty.py: Node, _Line)
// ============================================================================

/// A node of the repr tree, as in Rich.
#[derive(Debug, Default)]
struct Node {
    key_repr: String,
    value_repr: String,
    open_brace: String,
    close_brace: String,
    empty: String,
    is_tuple: bool,
    children: Option<Vec<Node>>,
}

impl Node {
    fn atom(repr: String) -> Self {
        Node {
            value_repr: repr,
            ..Node::default()
        }
    }

    /// Feed this node's tokens to `f` until it returns `false` (Rich's
    /// `Node.iter_tokens`). Returns `false` if stopped early.
    fn walk(&self, f: &mut dyn FnMut(&str) -> bool) -> bool {
        if !self.key_repr.is_empty() && !(f(&self.key_repr) && f(": ")) {
            return false;
        }
        if !self.value_repr.is_empty() {
            return f(&self.value_repr);
        }
        let Some(children) = &self.children else {
            return true;
        };
        if children.is_empty() {
            return f(&self.empty);
        }
        if !f(&self.open_brace) {
            return false;
        }
        if self.is_tuple && children.len() == 1 {
            if !(children[0].walk(f) && f(",")) {
                return false;
            }
        } else {
            for (i, child) in children.iter().enumerate() {
                if !child.walk(f) || (i + 1 < children.len() && !f(", ")) {
                    return false;
                }
            }
        }
        f(&self.close_brace)
    }

    /// The node on one line.
    fn inline(&self) -> String {
        let mut out = String::new();
        self.walk(&mut |token| {
            out.push_str(token);
            true
        });
        out
    }

    /// Whether the node fits in `max_width` after `start` cells (Rich's
    /// `Node.check_length`).
    fn fits(&self, start: usize, max_width: usize) -> bool {
        let mut total = start;
        self.walk(&mut |token| {
            total += UnicodeWidthStr::width(token);
            total <= max_width
        })
    }

    fn is_expandable(&self) -> bool {
        self.children.as_ref().is_some_and(|c| !c.is_empty())
    }
}

/// Rich's `to_repr` for strings: crop to `max_string` characters.
fn crop_string(literal: &str, max_string: Option<usize>) -> String {
    let Some(max) = max_string else {
        return literal.to_string();
    };
    let inner = &literal[1..literal.len().saturating_sub(1).max(1)];
    // One character per plain char or escape sequence (`\n`, `\u{301}`).
    let mut count = 0;
    let mut cut = None;
    let mut chars = inner.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if count == max && cut.is_none() {
            cut = Some(i);
        }
        count += 1;
        if c == '\\' {
            match chars.next() {
                Some((_, 'u')) if chars.peek().map(|&(_, c)| c) == Some('{') => {
                    for (_, c) in chars.by_ref() {
                        if c == '}' {
                            break;
                        }
                    }
                }
                Some((_, 'x')) => {
                    chars.next();
                    chars.next();
                }
                _ => {}
            }
        }
    }
    match cut {
        Some(cut) => {
            let quote = &literal[..1];
            format!("{quote}{}{quote}+{}", &inner[..cut], count - max)
        }
        None => literal.to_string(),
    }
}

/// Rich's `traverse`: build nodes, applying the length/string/depth limits.
fn traverse(value: &Value, depth: usize, options: &PrettyOptions) -> Node {
    match value {
        Value::Atom(s) => Node::atom(s.clone()),
        Value::Str(s) => Node::atom(crop_string(s, options.max_string)),
        Value::Container(container) => {
            let (open, close, empty) = match &container.kind {
                Kind::List => ("[".into(), "]".into(), "[]".into()),
                Kind::Tuple => ("(".into(), ")".into(), "()".into()),
                Kind::Brace => ("{".into(), "}".into(), "{}".into()),
                Kind::Struct(name) => (format!("{name} {{ "), " }".into(), format!("{name} {{}}")),
                Kind::TupleStruct(name) => (format!("{name}("), ")".into(), format!("{name}()")),
            };
            if options.max_depth.is_some_and(|max| depth >= max) {
                return Node::atom(format!("{open}...{close}"));
            }
            if container.items.is_empty() {
                return Node {
                    empty,
                    children: Some(Vec::new()),
                    ..Node::default()
                };
            }
            let abbreviate = matches!(container.kind, Kind::List | Kind::Tuple | Kind::Brace);
            let limit = match options.max_length {
                Some(max) if abbreviate => max,
                _ => usize::MAX,
            };
            let mut children: Vec<Node> = container
                .items
                .iter()
                .take(limit)
                .map(|item| {
                    let mut node = traverse(&item.value, depth + 1, options);
                    if let Some(key) = &item.key {
                        node.key_repr = key_repr(key, options);
                    }
                    node
                })
                .collect();
            if container.items.len() > limit {
                children.push(Node::atom(format!(
                    "... +{}",
                    container.items.len() - limit
                )));
            }
            Node {
                open_brace: open,
                close_brace: close,
                is_tuple: container.kind == Kind::Tuple,
                children: Some(children),
                ..Node::default()
            }
        }
    }
}

/// Rich's `to_repr(key)`: keys are shown on one line, and only a string key
/// is cropped.
fn key_repr(key: &Value, options: &PrettyOptions) -> String {
    match key {
        Value::Str(s) => crop_string(s, options.max_string),
        other => traverse(other, 0, &PrettyOptions::default()).inline(),
    }
}

/// One output line being laid out (Rich's `_Line`).
struct Line<'a> {
    node: Option<&'a Node>,
    text: String,
    suffix: &'static str,
    indent: usize,
    last: bool,
}

/// Rich's `Node.render`. Lines are processed depth first with an explicit
/// stack, which gives the same order as Rich's in-place list expansion.
fn render_node(root: &Node, options: &PrettyOptions) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut stack = vec![Line {
        node: Some(root),
        text: String::new(),
        suffix: "",
        indent: 0,
        last: false,
    }];
    while let Some(line) = stack.pop() {
        if let Some(node) = line.node.filter(|n| n.is_expandable()) {
            let start = line.indent
                + UnicodeWidthStr::width(line.text.as_str())
                + UnicodeWidthStr::width(line.suffix);
            if options.expand_all || !node.fits(start, options.max_width) {
                let children = node.children.as_deref().unwrap_or_default();
                let open = node.open_brace.trim_end();
                let mut opening = " ".repeat(line.indent);
                if !node.key_repr.is_empty() {
                    opening.push_str(&node.key_repr);
                    opening.push_str(": ");
                }
                opening.push_str(open);
                out.push(opening);

                stack.push(Line {
                    node: None,
                    text: node.close_brace.trim_start().to_string(),
                    suffix: line.suffix,
                    indent: line.indent,
                    last: line.last,
                });
                let tuple_of_one = node.is_tuple && children.len() == 1;
                let separator = if tuple_of_one { "," } else { ", " };
                for (i, child) in children.iter().enumerate().rev() {
                    stack.push(Line {
                        node: Some(child),
                        text: String::new(),
                        suffix: separator,
                        indent: line.indent + options.indent_size,
                        last: i + 1 == children.len() && !tuple_of_one,
                    });
                }
                continue;
            }
        }
        let mut s = " ".repeat(line.indent);
        s.push_str(&line.text);
        if let Some(node) = line.node {
            s.push_str(&node.inline());
        }
        if !line.last {
            s.push_str(line.suffix.trim_end());
        }
        out.push(s);
    }
    out.join("\n")
}

fn layout(value: &Value, options: &PrettyOptions) -> String {
    render_node(&traverse(value, 0, options), options)
}

/// Lay out a value's `Debug` output to fit `options.max_width` (Rich's
/// `pretty_repr`). The result has no styles; see [`Pretty`] for
/// highlighted output.
///
/// ```
/// use fast_rich::pretty::{pretty_repr, PrettyOptions};
/// use std::collections::BTreeMap;
///
/// let mut map = BTreeMap::new();
/// map.insert("numbers", vec![1, 2, 3]);
/// assert_eq!(pretty_repr(&map, &PrettyOptions::default()), "{\"numbers\": [1, 2, 3]}");
/// assert_eq!(
///     pretty_repr(&map, &PrettyOptions::default().max_width(12)),
///     "{\n    \"numbers\": [\n        1,\n        2,\n        3\n    ]\n}"
/// );
/// ```
pub fn pretty_repr<T: Debug + ?Sized>(value: &T, options: &PrettyOptions) -> String {
    layout(&parse(&format!("{value:?}")), options)
}

/// Pretty print a value to stdout at the terminal width (Rich's `pprint`).
///
/// `options.max_width` is ignored: the console width is used. Lines are
/// not wrapped (Rich prints with `soft_wrap=True`). Rich's `pprint` turns
/// indent guides on by default; here they follow
/// `options.indent_guides`.
///
/// ```no_run
/// use fast_rich::pretty::{pprint, PrettyOptions};
///
/// pprint(&vec![1, 2, 3], PrettyOptions::default().indent_guides(true));
/// ```
pub fn pprint<T: Debug + ?Sized>(value: &T, options: PrettyOptions) {
    let console = crate::Console::new();
    let pretty = Pretty::new(value)
        .options(options)
        .overflow(Overflow::Visible);
    console.print_renderable(&pretty);
}

// ============================================================================
// Pretty renderable
// ============================================================================

/// A renderable that pretty prints a value's `Debug` output (Rich's
/// `Pretty`).
///
/// It is laid out to the render width, highlighted with
/// [`ReprHighlighter`], and can draw indent guides (Rich's default guide
/// style `repr.indent`, dim green).
///
/// ```
/// use fast_rich::pretty::Pretty;
/// use fast_rich::Console;
///
/// let console = Console::capture().width(20);
/// let pretty = Pretty::new(&vec!["alpha", "beta", "gamma"]);
/// assert_eq!(
///     console.export_text(&pretty),
///     "[\n    \"alpha\",\n    \"beta\",\n    \"gamma\"\n]\n"
/// );
/// ```
#[derive(Clone)]
pub struct Pretty {
    value: Value,
    type_name: &'static str,
    options: PrettyOptions,
    highlighter: Option<Arc<dyn Highlighter + Send + Sync>>,
    highlight: bool,
    theme: Option<Theme>,
    overflow: Option<Overflow>,
}

impl std::fmt::Debug for Pretty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pretty")
            .field("type_name", &self.type_name)
            .field("options", &self.options)
            .field("highlight", &self.highlight)
            .finish_non_exhaustive()
    }
}

impl Pretty {
    /// Pretty print `value` (its `Debug` output is captured now).
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let pretty = Pretty::new(&Some((1, "a")));
    /// assert_eq!(pretty.text(80).plain_text(), "Some((1, \"a\"))");
    /// ```
    pub fn new<T: Debug + ?Sized>(value: &T) -> Self {
        Pretty {
            value: parse(&format!("{value:?}")),
            type_name: std::any::type_name::<T>(),
            options: PrettyOptions::default(),
            highlighter: None,
            highlight: true,
            theme: None,
            overflow: None,
        }
    }

    /// Replace all layout options at once (`max_width` is ignored: the
    /// render width is used).
    ///
    /// ```
    /// use fast_rich::pretty::{Pretty, PrettyOptions};
    /// let pretty = Pretty::new(&[1, 2, 3]).options(PrettyOptions::new().max_length(1));
    /// assert_eq!(pretty.text(80).plain_text(), "[1, ... +2]");
    /// ```
    pub fn options(mut self, options: PrettyOptions) -> Self {
        self.options = options;
        self
    }

    /// Spaces per indent level (default 4).
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&[1]).indent_size(2).text(2);
    /// assert_eq!(text.plain_text(), "[\n  1\n]");
    /// ```
    pub fn indent_size(mut self, size: usize) -> Self {
        self.options.indent_size = size;
        self
    }

    /// Show at most `length` children per container, then `... +K`.
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&[1, 2, 3]).max_length(2).text(80);
    /// assert_eq!(text.plain_text(), "[1, 2, ... +1]");
    /// ```
    pub fn max_length(mut self, length: usize) -> Self {
        self.options.max_length = Some(length);
        self
    }

    /// Crop strings longer than `length` characters (`"ab"+3`).
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&["hello"]).max_string(2).text(80);
    /// assert_eq!(text.plain_text(), "[\"he\"+3]");
    /// ```
    pub fn max_string(mut self, length: usize) -> Self {
        self.options.max_string = Some(length);
        self
    }

    /// Show containers at depth `depth` or deeper as `[...]`.
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&vec![vec![1]]).max_depth(1).text(80);
    /// assert_eq!(text.plain_text(), "[[...]]");
    /// ```
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.options.max_depth = Some(depth);
        self
    }

    /// Expand every container, even ones that fit.
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&[1]).expand_all(true).text(80);
    /// assert_eq!(text.plain_text(), "[\n    1\n]");
    /// ```
    pub fn expand_all(mut self, expand: bool) -> Self {
        self.options.expand_all = expand;
        self
    }

    /// Draw indent guides (`│`) in the `repr.indent` style.
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&vec![vec![1]]).indent_guides(true).text(0);
    /// assert_eq!(text.plain_text(), "[\n│   [\n│   │   1\n│   ]\n]");
    /// ```
    pub fn indent_guides(mut self, guides: bool) -> Self {
        self.options.indent_guides = guides;
        self
    }

    /// Turn highlighting off or on (default on).
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&[1]).highlight(false).text(80);
    /// assert!(text.spans.iter().all(|s| s.style.is_empty()));
    /// ```
    pub fn highlight(mut self, enabled: bool) -> Self {
        self.highlight = enabled;
        self
    }

    /// Use a custom highlighter instead of [`ReprHighlighter`].
    ///
    /// ```
    /// use fast_rich::highlighter::ReprHighlighter;
    /// use fast_rich::pretty::Pretty;
    /// let pretty = Pretty::new(&[true]).highlighter(ReprHighlighter::new().rich_compatible());
    /// assert_eq!(pretty.text(80).plain_text(), "[true]");
    /// ```
    pub fn highlighter(mut self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self {
        self.highlighter = Some(Arc::new(highlighter));
        self
    }

    /// Resolve the `repr.*` highlight styles and the `repr.indent` guide
    /// style from `theme`.
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// use fast_rich::style::{Color, Style};
    /// use fast_rich::theme::Theme;
    ///
    /// let mut theme = Theme::new();
    /// theme.add_style("repr.number", Style::new().foreground(Color::Red));
    /// let text = Pretty::new(&[7]).theme(theme).text(80);
    /// let seven = text.spans.iter().find(|s| s.text == "7").unwrap();
    /// assert_eq!(seven.style.foreground, Some(Color::Red));
    /// ```
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// How lines wider than the render width are handled (default: wrap).
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// use fast_rich::text::Overflow;
    /// let pretty = Pretty::new(&"a long string").overflow(Overflow::Visible);
    /// assert_eq!(pretty.text(4).overflow, Overflow::Visible);
    /// ```
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// The laid-out string for `width` cells (no styles).
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// assert_eq!(Pretty::new(&(1,)).repr(2), "(\n    1,\n)");
    /// ```
    pub fn repr(&self, width: usize) -> String {
        let options = PrettyOptions {
            max_width: width,
            ..self.options.clone()
        };
        layout(&self.value, &options)
    }

    /// The highlighted text for `width` cells, with indent guides if
    /// enabled.
    ///
    /// ```
    /// use fast_rich::pretty::Pretty;
    /// let text = Pretty::new(&vec![1, 2]).text(80);
    /// assert_eq!(text.plain_text(), "[1, 2]");
    /// ```
    pub fn text(&self, width: usize) -> Text {
        let repr = self.repr(width);
        let mut text = if repr.is_empty() {
            Text::styled(
                format!("{}'s Debug output is empty", self.type_name),
                Style::new().dim().italic(),
            )
        } else if !self.highlight {
            Text::plain(repr)
        } else {
            let spans = match (&self.highlighter, &self.theme) {
                (Some(custom), _) => custom.highlight(&repr),
                (None, Some(theme)) => ReprHighlighter::new().theme(theme.clone()).highlight(&repr),
                (None, None) => ReprHighlighter::new().highlight(&repr),
            };
            Text::from_spans(spans)
        };
        if self.options.indent_guides && self.options.indent_size > 0 {
            let style = self
                .theme
                .as_ref()
                .and_then(|t| t.lookup_style("repr.indent"))
                .or_else(|| crate::theme::builtin_style("repr.indent"))
                .unwrap_or_default();
            text = text.with_indent_guides(Some(self.options.indent_size), '│', style);
        }
        if let Some(overflow) = self.overflow {
            text.overflow = overflow;
        }
        text
    }
}

fn widest_line(s: &str) -> usize {
    s.split('\n').map(UnicodeWidthStr::width).max().unwrap_or(0)
}

impl Renderable for Pretty {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        self.text(context.width).render(context)
    }

    /// The widest line when every container is expanded.
    fn min_width(&self) -> usize {
        widest_line(&self.repr(0)).max(1)
    }

    /// The width of the value on one line.
    fn max_width(&self) -> usize {
        widest_line(&self.repr(usize::MAX))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    fn repr_str(debug: &str, options: &PrettyOptions) -> String {
        layout(&parse(debug), options)
    }

    fn width(w: usize) -> PrettyOptions {
        PrettyOptions::default().max_width(w)
    }

    #[test]
    fn fits_on_one_line() {
        assert_eq!(pretty_repr(&vec![1, 2, 3], &width(80)), "[1, 2, 3]");
        assert_eq!(pretty_repr(&vec![1, 2, 3], &width(9)), "[1, 2, 3]");
        assert_eq!(
            pretty_repr(&vec![1, 2, 3], &width(8)),
            "[\n    1,\n    2,\n    3\n]"
        );
    }

    #[test]
    fn empty_containers() {
        for w in [0, 1, 80] {
            assert_eq!(pretty_repr(&Vec::<i32>::new(), &width(w)), "[]");
            assert_eq!(pretty_repr(&(), &width(w)), "()");
            assert_eq!(pretty_repr(&BTreeMap::<i32, i32>::new(), &width(w)), "{}");
            assert_eq!(pretty_repr(&BTreeSet::<i32>::new(), &width(w)), "{}");
        }
    }

    #[test]
    fn nested_empties() {
        let v: Vec<Vec<Vec<i32>>> = vec![vec![], vec![vec![]]];
        assert_eq!(pretty_repr(&v, &width(80)), "[[], [[]]]");
        assert_eq!(
            pretty_repr(&v, &width(0)),
            "[\n    [],\n    [\n        []\n    ]\n]"
        );
    }

    #[test]
    fn width_zero_and_one() {
        let v = vec![(1,), (2,)];
        let expected = "[\n    (\n        1,\n    ),\n    (\n        2,\n    )\n]";
        assert_eq!(pretty_repr(&v, &width(0)), expected);
        assert_eq!(pretty_repr(&v, &width(1)), expected);
        assert_eq!(pretty_repr(&42, &width(0)), "42");
    }

    #[test]
    fn tuple_of_one_keeps_comma() {
        assert_eq!(pretty_repr(&(1,), &width(80)), "(1,)");
        assert_eq!(pretty_repr(&(1,), &width(0)), "(\n    1,\n)");
        assert_eq!(pretty_repr(&(1, 2), &width(80).max_length(0)), "(... +2,)");
    }

    #[test]
    fn maps_and_keys() {
        let mut m = BTreeMap::new();
        m.insert("a", vec![1, 2]);
        assert_eq!(
            pretty_repr(&m, &width(5)),
            "{\n    \"a\": [\n        1,\n        2\n    ]\n}"
        );
        let mut t = BTreeMap::new();
        t.insert((1, 2), "x");
        assert_eq!(pretty_repr(&t, &width(0)), "{\n    (1, 2): \"x\"\n}");
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Wrapper(Vec<u8>, &'static str);

    #[derive(Debug)]
    struct Unit;

    #[test]
    fn structs_keep_rust_syntax() {
        let p = Point { x: 1, y: -2 };
        assert_eq!(pretty_repr(&p, &width(80)), "Point { x: 1, y: -2 }");
        assert_eq!(
            pretty_repr(&p, &width(10)),
            "Point {\n    x: 1,\n    y: -2\n}"
        );
        assert_eq!(pretty_repr(&p, &width(80).max_depth(0)), "Point { ... }");
        let w = Wrapper(vec![1, 2], "s");
        assert_eq!(pretty_repr(&w, &width(80)), "Wrapper([1, 2], \"s\")");
        assert_eq!(
            pretty_repr(&w, &width(12)),
            "Wrapper(\n    [1, 2],\n    \"s\"\n)"
        );
        assert_eq!(pretty_repr(&w, &width(80).max_depth(0)), "Wrapper(...)");
        assert_eq!(pretty_repr(&Unit, &width(0)), "Unit");
        assert_eq!(pretty_repr(&Some(Unit), &width(0)), "Some(\n    Unit\n)");
        assert_eq!(pretty_repr(&None::<i32>, &width(0)), "None");
        let r: Result<i32, String> = Err("bad".into());
        assert_eq!(pretty_repr(&r, &width(80)), "Err(\"bad\")");
    }

    #[test]
    fn struct_fields_not_abbreviated() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(
            pretty_repr(&p, &width(80).max_length(0)),
            "Point { x: 1, y: 2 }"
        );
    }

    #[test]
    fn non_exhaustive_struct() {
        let out = repr_str("Panel { title: None, .. }", &width(0));
        assert_eq!(out, "Panel {\n    title: None,\n    ..\n}");
    }

    #[test]
    fn multibyte_strings() {
        let v = vec!["é日本🎉", "x"];
        // "é日本🎉" is 7 cells wide plus quotes.
        assert_eq!(pretty_repr(&v, &width(16)), "[\"é日本🎉\", \"x\"]");
        assert_eq!(
            pretty_repr(&v, &width(15)),
            "[\n    \"é日本🎉\",\n    \"x\"\n]"
        );
        assert_eq!(
            pretty_repr(&v, &width(80).max_string(2)),
            "[\"é日\"+2, \"x\"]"
        );
    }

    #[test]
    fn strings_with_brackets_quotes_and_escapes() {
        let v = vec![
            "[a, {b}]",
            "say \"hi\"",
            "tab\there",
            "back\\slash",
            "(",
            "x: y",
        ];
        let out = pretty_repr(&v, &width(0));
        let expected = format!(
            "[\n{}\n]",
            v.iter()
                .map(|s| format!("    {s:?}"))
                .collect::<Vec<_>>()
                .join(",\n")
        );
        assert_eq!(out, expected);
        // An escape counts as one character.
        assert_eq!(
            pretty_repr(&"a\nbc", &width(80).max_string(2)),
            "\"a\\n\"+2"
        );
        assert_eq!(
            pretty_repr(&"\u{301}xy", &width(80).max_string(1)),
            "\"\\u{301}\"+2"
        );
    }

    #[test]
    fn chars() {
        let v = vec!['a', '\'', '[', '"', '\n', '日'];
        assert_eq!(pretty_repr(&v, &width(80)), format!("{v:?}"));
        assert_eq!(
            pretty_repr(&v, &width(0)).lines().count(),
            8,
            "one char per line"
        );
    }

    #[test]
    fn max_limits_at_zero() {
        let v = vec![vec![1, 2], vec![3]];
        assert_eq!(pretty_repr(&v, &width(80).max_length(0)), "[... +2]");
        assert_eq!(pretty_repr(&v, &width(80).max_depth(0)), "[...]");
        assert_eq!(
            pretty_repr(&vec!["abc"], &width(80).max_string(0)),
            "[\"\"+3]"
        );
        let mut m = BTreeMap::new();
        m.insert("abcdef", "xyz");
        assert_eq!(
            pretty_repr(&m, &width(80).max_string(2)),
            "{\"ab\"+4: \"xy\"+1}"
        );
        assert_eq!(pretty_repr(&m, &width(80).max_length(0)), "{... +1}");
        assert_eq!(
            pretty_repr(&vec![Vec::<i32>::new()], &width(80).max_depth(1)),
            "[[...]]"
        );
        assert_eq!(pretty_repr(&"abc", &width(80).max_string(3)), "\"abc\"");
    }

    #[test]
    fn indent_size_zero() {
        assert_eq!(
            pretty_repr(&[1, 2], &width(1).indent_size(0)),
            "[\n1,\n2\n]"
        );
    }

    #[test]
    fn very_deep_nesting_does_not_overflow() {
        let n = 10_000;
        let debug = format!("{}{}", "[".repeat(n), "]".repeat(n));
        let out = repr_str(&debug, &width(80));
        assert!(out.starts_with('['));
        assert_eq!(out.replace(['\n', ' '], ""), debug);
        let out = repr_str(&debug, &width(80).max_depth(3));
        assert_eq!(out, "[[[[...]]]]");
        // Malformed deep input is one atom.
        let broken = "[".repeat(n);
        assert_eq!(repr_str(&broken, &width(80)), broken);
    }

    #[test]
    fn deep_nesting_from_debug() {
        // A real deeply nested value, built iteratively.
        #[derive(Debug)]
        #[allow(dead_code)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        let mut list = List::Nil;
        for i in 0..2_000 {
            list = List::Cons(i, Box::new(list));
        }
        // Debug-formatting this needs a big stack; do it on a thread.
        let out = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let out = pretty_repr(&list, &width(80));
                // Drop iteratively to avoid a recursive drop overflow.
                let mut cur = list;
                while let List::Cons(_, next) = cur {
                    cur = *next;
                }
                out
            })
            .unwrap()
            .join()
            .unwrap();
        assert!(out.starts_with("Cons(\n    1999,\n"));
    }

    #[test]
    fn opaque_atoms() {
        for s in [
            "1.5s",
            "PhantomData<(i32, u8)>",
            "a, b",
            "Error: bad",
            "<unknown>",
            "foo [1]",
            "x'y",
            "",
            "\"unterminated",
            "[1, 2",
            "1, 2]",
            "[,]",
        ] {
            let out = repr_str(s, &width(0));
            assert_eq!(out, s, "input {s:?}");
        }
        assert_eq!(repr_str("[1..5, 2s]", &width(0)), "[\n    1..5,\n    2s\n]");
        assert_eq!(repr_str("[a: b]", &width(0)), "[\n    a: b\n]");
        // A key separator with no key is plain text.
        assert_eq!(repr_str("{: 1}", &width(0)), "{\n    : 1\n}");
    }

    #[test]
    fn fuzz_never_panics() {
        // Simple xorshift so the test is deterministic without a dependency.
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let alphabet: Vec<char> = "[](){},: \"'\\ab1é日🎉.-_:<>u{}x\n".chars().collect();
        for _ in 0..20_000 {
            let len = (next() % 24) as usize;
            let s: String = (0..len)
                .map(|_| alphabet[(next() % alphabet.len() as u64) as usize])
                .collect();
            for w in [0, 3, 80] {
                let options = width(w)
                    .max_string((next() % 3) as usize)
                    .max_length((next() % 3) as usize)
                    .max_depth((next() % 3) as usize);
                let _ = repr_str(&s, &options);
                let _ = repr_str(&s, &width(w));
            }
        }
    }

    #[test]
    fn round_trips_debug_when_wide() {
        let mut m = BTreeMap::new();
        m.insert("k", (vec![Some(1.5), None], 'c', "s\"q"));
        let p = Point { x: 3, y: 4 };
        assert_eq!(pretty_repr(&m, &width(usize::MAX)), format!("{m:?}"));
        assert_eq!(pretty_repr(&p, &width(usize::MAX)), format!("{p:?}"));
    }

    #[test]
    fn pretty_text_highlights_and_guides() {
        let pretty = Pretty::new(&vec![vec![1, 2]]).indent_guides(true);
        let text = pretty.text(0);
        assert_eq!(
            text.plain_text(),
            "[\n│   [\n│   │   1,\n│   │   2\n│   ]\n]"
        );
        let guide = text.spans.iter().find(|s| s.text.contains('│')).unwrap();
        assert_eq!(
            guide.style,
            crate::theme::builtin_style("repr.indent").unwrap()
        );
        let one = text.spans.iter().find(|s| s.text == "1").unwrap();
        assert!(one.style.bold, "numbers are highlighted");
    }

    #[test]
    fn pretty_renders_at_console_width() {
        let console = crate::Console::capture().width(10);
        assert_eq!(
            console.export_text(&Pretty::new(&vec![100, 200, 300])),
            "[\n    100,\n    200,\n    300\n]\n"
        );
    }

    #[test]
    fn pretty_empty_debug() {
        struct Empty;
        impl Debug for Empty {
            fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Ok(())
            }
        }
        let text = Pretty::new(&Empty).text(80);
        assert!(text.plain_text().ends_with("Debug output is empty"));
    }

    #[test]
    fn measure_widths() {
        let pretty = Pretty::new(&vec![vec![1, 2, 3]]);
        assert_eq!(pretty.max_width(), 11, "[[1, 2, 3]]");
        assert_eq!(pretty.min_width(), 10, "        1,");
    }
}
