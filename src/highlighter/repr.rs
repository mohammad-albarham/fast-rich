//! [`ReprHighlighter`]: Rich's default highlighter for printed strings.
//!
//! The patterns below are a one-to-one transcription of
//! `rich.highlighter.ReprHighlighter` (Rich 15). Each is applied as a
//! separate pass in Rich's order, and later passes layer their style over
//! earlier ones, exactly as Rich's `Text.highlight_regex` spans do.

use super::engine::{
    alt, ch, class, find_iter, group, is_decimal, is_word, lazy, lit, opt, rep, seq, Guard, Node,
    Pattern, INF,
};
use super::Highlighter;
use crate::style::Style;
use crate::text::Span;
use crate::theme::Theme;
use std::sync::LazyLock;

// --- character classes -----------------------------------------------------

fn any(_: char) -> bool {
    true
}
fn not_newline(c: char) -> bool {
    c != '\n'
}
fn digit(c: char) -> bool {
    c.is_ascii_digit()
}
fn hex(c: char) -> bool {
    c.is_ascii_hexdigit()
}
/// `[-\w.:|]`
fn tag_name_char(c: char) -> bool {
    is_word(c) || matches!(c, '-' | '.' | ':' | '|')
}
/// `[\w.]`
fn call_char(c: char) -> bool {
    is_word(c) || c == '.'
}
/// `[-\w._+]`
fn path_char(c: char) -> bool {
    is_word(c) || matches!(c, '-' | '.' | '_' | '+')
}
/// ``[-0-9a-zA-Z$_+!`(),.?/;:&=%#~@]``
fn url_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "-$_+!`(),.?/;:&=%#~@".contains(c)
}
fn sign(c: char) -> bool {
    c == '-' || c == '+'
}
fn backslash(c: char) -> bool {
    c == '\\'
}
fn word_or_backslash(c: char) -> bool {
    is_word(c) || c == '\\'
}
fn brace_char(c: char) -> bool {
    matches!(c, '[' | ']' | '{' | '}' | '(' | ')')
}
fn combined_start(c: char) -> bool {
    is_word(c) || matches!(c, '.' | '-' | '/' | '\'' | '"' | '(')
}
fn is_lt(c: char) -> bool {
    c == '<'
}
/// Every char a complex number body can contain before its `j`.
fn complex_body_char(c: char) -> bool {
    is_decimal(c) || matches!(c, '.' | 'e' | '+' | '-')
}
fn is_j(c: char) -> bool {
    c == 'j'
}
fn is_open_paren(c: char) -> bool {
    c == '('
}

/// `[class]{n}`
fn exactly(f: fn(char) -> bool, n: usize) -> Node {
    rep(class(f), n, n)
}

// --- the passes ------------------------------------------------------------

/// `(?P<tag_start><)(?P<tag_name>[-\w.:|]*)(?P<tag_contents>[\w\W]*)(?P<tag_end>>)`
fn tag_pattern() -> Pattern {
    Pattern {
        root: seq(vec![
            // `>` must appear later, or the greedy body scans in vain.
            Node::Guard(0, Guard::CharLater('>')),
            group(0, ch('<')),
            group(1, rep(class(tag_name_char), 0, INF)),
            group(2, rep(class(any), 0, INF)),
            group(3, ch('>')),
        ]),
        names: vec!["tag_start", "tag_name", "tag_contents", "tag_end"],
        may_start: is_lt,
    }
}

/// `(?P<attrib_name>[\w_]{1,50})=(?P<attrib_value>"?[\w_]+"?)?`
fn attrib_pattern() -> Pattern {
    Pattern {
        root: seq(vec![
            group(0, rep(class(is_word), 1, 50)),
            ch('='),
            opt(group(
                1,
                seq(vec![
                    opt(ch('"')),
                    rep(class(is_word), 1, INF),
                    opt(ch('"')),
                ]),
            )),
        ]),
        names: vec!["attrib_name", "attrib_value"],
        may_start: is_word,
    }
}

/// `(?P<brace>[][{}()])`
fn brace_pattern() -> Pattern {
    Pattern {
        root: group(0, class(brace_char)),
        names: vec!["brace"],
        may_start: brace_char,
    }
}

const IPV4: usize = 0;
const IPV6: usize = 1;
const EUI64: usize = 2;
const EUI48: usize = 3;
const UUID: usize = 4;
const CALL: usize = 5;
const BOOL_TRUE: usize = 6;
const BOOL_FALSE: usize = 7;
const NONE: usize = 8;
const ELLIPSIS: usize = 9;
const NUMBER_COMPLEX: usize = 10;
const NUMBER: usize = 11;
const PATH: usize = 12;
const FILENAME: usize = 13;
const STR: usize = 14;
const URL: usize = 15;

/// `\b(?P<name>word)\b`
fn keyword(index: usize, word: &'static str) -> Node {
    seq(vec![
        Node::WordBoundary,
        group(index, lit(word)),
        Node::WordBoundary,
    ])
}

/// `(?:H{n}-){count}H{n}` style EUI address.
fn eui(sep: char, part: usize, count: usize) -> Node {
    seq(vec![
        rep(seq(vec![rep(class(hex), 1, part), ch(sep)]), count, count),
        rep(class(hex), 1, part),
    ])
}

/// `(?:H{4}\.){count}H{4}` dotted EUI address.
fn eui_dotted(count: usize) -> Node {
    seq(vec![
        rep(seq(vec![exactly(hex, 4), ch('.')]), count, count),
        exactly(hex, 4),
    ])
}

/// `b?Q.*?(?<!\\)Q` for a quote string `Q`.
fn quoted(q: &'static str) -> Node {
    seq(vec![
        opt(ch('b')),
        lit(q),
        lazy(class(not_newline), 0, INF),
        Node::NotAfter(backslash),
        lit(q),
    ])
}

/// `e[-+]?\d+?` (lazy) or `e[-+]?\d+` (greedy).
fn exponent(greedy: bool) -> Node {
    let digits = if greedy {
        rep(class(is_decimal), 1, INF)
    } else {
        lazy(class(is_decimal), 1, INF)
    };
    seq(vec![ch('e'), opt(class(sign)), digits])
}

/// `[0-9]+\.?[0-9]*`
fn decimal() -> Vec<Node> {
    vec![
        rep(class(digit), 1, INF),
        opt(ch('.')),
        rep(class(digit), 0, INF),
    ]
}

/// The big combined pattern. `rust_literals` adds `true` / `false`.
fn combined_pattern(rust_literals: bool) -> Pattern {
    let octet = || rep(class(digit), 1, 3);
    let mut alts = vec![
        // ipv4
        group(
            IPV4,
            seq(vec![
                octet(),
                ch('.'),
                octet(),
                ch('.'),
                octet(),
                ch('.'),
                octet(),
            ]),
        ),
        // ipv6: ([A-Fa-f0-9]{1,4}::?){1,7}[A-Fa-f0-9]{1,4}
        group(
            IPV6,
            seq(vec![
                rep(
                    seq(vec![rep(class(hex), 1, 4), ch(':'), opt(ch(':'))]),
                    1,
                    7,
                ),
                rep(class(hex), 1, 4),
            ]),
        ),
        group(
            EUI64,
            alt(vec![eui('-', 2, 7), eui(':', 2, 7), eui_dotted(3)]),
        ),
        group(
            EUI48,
            alt(vec![eui('-', 2, 5), eui(':', 2, 5), eui_dotted(2)]),
        ),
        group(
            UUID,
            seq(vec![
                exactly(hex, 8),
                ch('-'),
                exactly(hex, 4),
                ch('-'),
                exactly(hex, 4),
                ch('-'),
                exactly(hex, 4),
                ch('-'),
                exactly(hex, 12),
            ]),
        ),
        // (?P<call>[\w.]*?)\(
        // '(' isn't in [\w.], so the run of [\w.] here must end at a '('.
        seq(vec![
            Node::Guard(
                0,
                Guard::RunThen {
                    class: call_char,
                    then: is_open_paren,
                },
            ),
            group(CALL, lazy(class(call_char), 0, INF)),
            ch('('),
        ]),
        keyword(BOOL_TRUE, "True"),
        keyword(BOOL_FALSE, "False"),
        keyword(NONE, "None"),
    ];
    if rust_literals {
        alts.push(keyword(BOOL_TRUE, "true"));
        alts.push(keyword(BOOL_FALSE, "false"));
    }
    alts.extend([
        group(ELLIPSIS, lit("...")),
        // number_complex
        group(NUMBER_COMPLEX, {
            let mut parts = vec![
                Node::NotAfter(is_word),
                // The body uses only these chars, so its `j` ends the run.
                Node::Guard(
                    1,
                    Guard::RunThen {
                        class: complex_body_char,
                        then: is_j,
                    },
                ),
                opt(ch('-')),
            ];
            parts.extend(decimal());
            parts.push(opt(exponent(false)));
            let mut imag = vec![class(sign)];
            imag.extend(decimal());
            imag.push(opt(exponent(true)));
            parts.push(opt(seq(imag)));
            parts.push(ch('j'));
            seq(parts)
        }),
        // number
        group(
            NUMBER,
            alt(vec![
                {
                    let mut parts = vec![Node::NotAfter(is_word), opt(ch('-'))];
                    parts.extend(decimal());
                    parts.push(opt(exponent(false)));
                    parts.push(Node::WordBoundary);
                    seq(parts)
                },
                seq(vec![lit("0x"), rep(class(hex), 0, INF)]),
            ]),
        ),
        // (?P<path>\B(/[-\w._+]+)*\/)(?P<filename>[-\w._+]*)?
        seq(vec![
            group(
                PATH,
                seq(vec![
                    Node::NotWordBoundary,
                    // (/[-\w._+]+)*  ('/' is not a path_char)
                    Node::Segments {
                        sep: '/',
                        class: path_char,
                    },
                    ch('/'),
                ]),
            ),
            opt(group(FILENAME, rep(class(path_char), 0, INF))),
        ]),
        // (?<![\\\w])(?P<str>...)
        seq(vec![
            Node::NotAfter(word_or_backslash),
            group(
                STR,
                alt(vec![
                    quoted("'''"),
                    quoted("'"),
                    quoted("\"\"\""),
                    quoted("\""),
                ]),
            ),
        ]),
        group(
            URL,
            seq(vec![
                alt(vec![
                    lit("file"),
                    lit("https"),
                    lit("http"),
                    lit("ws"),
                    lit("wss"),
                ]),
                lit("://"),
                rep(class(url_char), 0, INF),
            ]),
        ),
    ]);
    Pattern {
        root: alt(alts),
        names: vec![
            "ipv4",
            "ipv6",
            "eui64",
            "eui48",
            "uuid",
            "call",
            "bool_true",
            "bool_false",
            "none",
            "ellipsis",
            "number_complex",
            "number",
            "path",
            "filename",
            "str",
            "url",
        ],
        may_start: combined_start,
    }
}

static TAG: LazyLock<Pattern> = LazyLock::new(tag_pattern);
static ATTRIB: LazyLock<Pattern> = LazyLock::new(attrib_pattern);
static BRACE: LazyLock<Pattern> = LazyLock::new(brace_pattern);
static COMBINED_RICH: LazyLock<Pattern> = LazyLock::new(|| combined_pattern(false));
static COMBINED_RUST: LazyLock<Pattern> = LazyLock::new(|| combined_pattern(true));

/// Attributes a Rich default style explicitly turns *off* (`bold=False`,
/// `italic=False`), clearing them from styles layered underneath.
fn unset_flags(name: &str) -> (bool, bool) {
    match name {
        "str" | "url" => (true, true),
        "attrib_name" | "attrib_value" | "number" | "number_complex" => (false, true),
        "uuid" => (true, false),
        _ => (false, false),
    }
}

/// One highlighted region: byte range plus the `repr.*` style name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HighlightRegion {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// Style name without the `repr.` prefix, e.g. `"number"`.
    pub name: &'static str,
}

/// Run every pass and return the regions in application order.
fn regions(text: &str, rust_literals: bool) -> Vec<HighlightRegion> {
    let chars: Vec<char> = text.chars().collect();
    // char index -> byte offset (one extra entry for the end).
    let mut offsets: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    offsets.push(text.len());

    let combined: &Pattern = if rust_literals {
        &COMBINED_RUST
    } else {
        &COMBINED_RICH
    };
    let mut out = Vec::new();
    for pattern in [&*TAG, &*ATTRIB, &*BRACE, combined] {
        find_iter(pattern, &chars, &mut |g| {
            out.push(HighlightRegion {
                start: offsets[g.start],
                end: offsets[g.end],
                name: pattern.names[g.group],
            })
        });
    }
    out
}

/// Flatten layered regions into non-overlapping styled byte ranges.
pub(crate) fn styled_ranges(
    text: &str,
    theme: &Theme,
    rust_literals: bool,
) -> Vec<(usize, usize, Style)> {
    let regions = regions(text, rust_literals);
    if regions.is_empty() {
        return Vec::new();
    }
    let resolved: Vec<Style> = regions
        .iter()
        .map(|r| {
            theme
                .lookup_style(&format!("repr.{}", r.name))
                .unwrap_or_default()
        })
        .collect();

    // Sweep over boundaries, keeping the regions active in each window in
    // application order (lower index = applied earlier).
    let mut events: Vec<(usize, bool, usize)> = Vec::with_capacity(regions.len() * 2);
    for (i, r) in regions.iter().enumerate() {
        events.push((r.start, true, i));
        events.push((r.end, false, i));
    }
    events.sort_unstable_by_key(|&(pos, _, _)| pos);

    let mut active = std::collections::BTreeSet::new();
    let mut out: Vec<(usize, usize, Style)> = Vec::new();
    let mut e = 0;
    while e < events.len() {
        let start = events[e].0;
        while e < events.len() && events[e].0 == start {
            let (_, opens, i) = events[e];
            if opens {
                active.insert(i);
            } else {
                active.remove(&i);
            }
            e += 1;
        }
        let Some(&(end, _, _)) = events.get(e) else {
            break;
        };
        if active.is_empty() {
            continue;
        }
        let mut style = Style::new();
        for &i in &active {
            let (no_bold, no_italic) = unset_flags(regions[i].name);
            if no_bold {
                style.bold = false;
            }
            if no_italic {
                style.italic = false;
            }
            style = style.combine(&resolved[i]);
        }
        if style.is_empty() {
            continue;
        }
        match out.last_mut() {
            Some(last) if last.1 == start && last.2 == style => last.1 = end,
            _ => out.push((start, end, style)),
        }
    }
    out
}

/// Rich's default highlighter for printed strings.
///
/// Colors numbers, strings, booleans, `None`, IPv4/IPv6/MAC addresses,
/// UUIDs, URLs, paths, `key=value` pairs, `<tags>`, calls like `Some(` and
/// brackets — the same token rules as Python Rich's `ReprHighlighter`.
///
/// [`Console`](crate::Console) applies it to every `print`/`println` by
/// default; turn it off with [`Console::highlight`](crate::Console::highlight).
///
/// Styles come from the `repr.*` names in the [`Theme`], so they can be
/// restyled:
///
/// ```
/// use fast_rich::highlighter::{Highlighter, ReprHighlighter};
/// use fast_rich::style::{Color, Style};
/// use fast_rich::theme::Theme;
///
/// let mut theme = Theme::new();
/// theme.add_style("repr.number", Style::new().foreground(Color::Red));
/// let hl = ReprHighlighter::new().theme(theme);
///
/// let spans = hl.highlight("took 42 ms");
/// let number = spans.iter().find(|s| s.text == "42").unwrap();
/// assert_eq!(number.style.foreground, Some(Color::Red));
/// ```
///
/// Beyond Rich, it also colors Rust's `true` / `false`. Use
/// [`rich_compatible`](Self::rich_compatible) for byte-exact Rich behavior.
#[derive(Debug, Clone)]
pub struct ReprHighlighter {
    theme: Option<Theme>,
    rust_literals: bool,
}

impl Default for ReprHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl ReprHighlighter {
    /// Create a highlighter with Rich's default `repr.*` styles.
    pub fn new() -> Self {
        ReprHighlighter {
            theme: None,
            rust_literals: true,
        }
    }

    /// Resolve `repr.*` styles from `theme` (names it lacks use Rich's
    /// defaults).
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Match Python Rich exactly: don't highlight Rust's lowercase
    /// `true` / `false`.
    pub fn rich_compatible(mut self) -> Self {
        self.rust_literals = false;
        self
    }

    /// Whether lowercase `true` / `false` are highlighted (default: yes).
    pub fn rust_literals(mut self, enabled: bool) -> Self {
        self.rust_literals = enabled;
        self
    }

    /// The raw matches, in the order Rich applies them, with `repr.*`
    /// names. Later regions take precedence where they overlap.
    pub fn regions(&self, text: &str) -> Vec<HighlightRegion> {
        regions(text, self.rust_literals)
    }

    pub(crate) fn styled_ranges_with(
        &self,
        text: &str,
        fallback: &Theme,
    ) -> Vec<(usize, usize, Style)> {
        styled_ranges(
            text,
            self.theme.as_ref().unwrap_or(fallback),
            self.rust_literals,
        )
    }
}

/// Turn non-overlapping styled byte ranges into spans covering `text`.
pub(crate) fn ranges_to_spans(text: &str, ranges: &[(usize, usize, Style)]) -> Vec<Span> {
    let mut spans = Vec::with_capacity(ranges.len() * 2 + 1);
    let mut last = 0;
    for &(start, end, style) in ranges {
        if start > last {
            spans.push(Span::raw(text[last..start].to_string()));
        }
        spans.push(Span::styled(text[start..end].to_string(), style));
        last = end;
    }
    if last < text.len() || spans.is_empty() {
        spans.push(Span::raw(text[last..].to_string()));
    }
    spans
}

impl Highlighter for ReprHighlighter {
    fn highlight(&self, text: &str) -> Vec<Span> {
        static DEFAULT_THEME: LazyLock<Theme> = LazyLock::new(Theme::new);
        let ranges = self.styled_ranges_with(text, &DEFAULT_THEME);
        ranges_to_spans(text, &ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn names(text: &str) -> Vec<(String, &'static str)> {
        ReprHighlighter::new()
            .regions(text)
            .into_iter()
            .map(|r| (text[r.start..r.end].to_string(), r.name))
            .collect()
    }

    fn has(text: &str, token: &str, name: &str) -> bool {
        names(text).iter().any(|(t, n)| t == token && *n == name)
    }

    #[test]
    fn numbers() {
        assert!(has("got 404 back", "404", "number"));
        assert!(has("x -3.25 y", "-3.25", "number"));
        assert!(has("v 1e10", "1e10", "number"));
        assert!(has("0xff", "0xff", "number"));
        assert!(has("z 3+4j", "3+4j", "number_complex"));
        // Python's backtracking quirk: "3.5s" highlights "3."
        assert!(has("3.5s", "3.", "number"));
        // Not inside identifiers
        assert!(!names("abc123").iter().any(|(_, n)| *n == "number"));
    }

    #[test]
    fn literals() {
        assert!(has("a True b", "True", "bool_true"));
        assert!(has("a False b", "False", "bool_false"));
        assert!(has("a None b", "None", "none"));
        assert!(has("ok true", "true", "bool_true"));
        assert!(has("ok false", "false", "bool_false"));
        assert!(!ReprHighlighter::new()
            .rich_compatible()
            .regions("ok true")
            .iter()
            .any(|r| r.name == "bool_true"));
        assert!(!has("untrue", "true", "bool_true"));
    }

    #[test]
    fn addresses_and_ids() {
        assert!(has("at 192.168.1.10 now", "192.168.1.10", "ipv4"));
        assert!(has("at fe80::1 now", "fe80::1", "ipv6"));
        assert!(has("mac 00-1A-2B-3C-4D-5E", "00-1A-2B-3C-4D-5E", "eui48"));
        // Rich tries ipv6 first, so colon-separated MACs come out as ipv6.
        assert!(has("mac 00:1A:2B:3C:4D:5E", "00:1A:2B:3C:4D:5E", "ipv6"));
        assert!(has(
            "id 550e8400-e29b-41d4-a716-446655440000",
            "550e8400-e29b-41d4-a716-446655440000",
            "uuid"
        ));
    }

    #[test]
    fn urls_paths_strings() {
        assert!(has(
            "see https://x.io/a?b=1 now",
            "https://x.io/a?b=1",
            "url"
        ));
        assert!(has("at /var/log/app.log", "/var/log/", "path"));
        assert!(has("at /var/log/app.log", "app.log", "filename"));
        assert!(has("say 'hi there'", "'hi there'", "str"));
        assert!(has("say \"hi\"", "\"hi\"", "str"));
        assert!(!has("don't stop", "'t stop", "str"));
    }

    #[test]
    fn structure() {
        assert!(has("retry=3", "retry", "attrib_name"));
        assert!(has("retry=3", "3", "attrib_value"));
        assert!(has("Some(1)", "Some", "call"));
        assert!(has("[1]", "[", "brace"));
        assert!(has("<Foo bar>", "Foo", "tag_name"));
        assert!(has("wait...", "...", "ellipsis"));
    }

    #[test]
    fn spans_cover_text_exactly() {
        let text = "x=1 [a, 'b'] Some(None) /tmp/f.txt é 🎉 192.168.0.1";
        let spans = ReprHighlighter::new().highlight(text);
        let joined: String = spans.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(joined, text);
    }

    #[test]
    fn string_clears_brace_bold() {
        // Rich: repr.str is bold=False, so '[' inside a string is not bold.
        let spans = ReprHighlighter::new().highlight("'[x]'");
        assert_eq!(spans.len(), 1);
        assert!(!spans[0].style.bold);
        assert_eq!(spans[0].style.foreground, Some(Color::Green));
    }

    #[test]
    fn plain_text_untouched() {
        let spans = ReprHighlighter::new().highlight("hello world");
        assert_eq!(spans, vec![Span::raw("hello world")]);
    }
}
