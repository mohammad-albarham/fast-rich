//! Text and Span types for styled text with wrapping and alignment.
//!
//! This module provides `Text` - a container for styled text that supports
//! word wrapping, alignment, and combining multiple styled spans.

use crate::style::Style;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A styled region of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    /// The text content
    pub text: Cow<'static, str>,
    /// The style applied to this span
    pub style: Style,
    /// Optional hyperlink URL (OSC 8 terminal links)
    pub link: Option<String>,
}

impl Span {
    /// Create a new span with no style.
    pub fn raw<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Span {
            text: text.into(),
            style: Style::new(),
            link: None,
        }
    }

    /// Create a new span with a style.
    pub fn styled<S: Into<Cow<'static, str>>>(text: S, style: Style) -> Self {
        Span {
            text: text.into(),
            style,
            link: None,
        }
    }

    /// Create a new span with a style and hyperlink.
    pub fn linked<S: Into<Cow<'static, str>>>(text: S, style: Style, url: String) -> Self {
        Span {
            text: text.into(),
            style,
            link: Some(url),
        }
    }

    /// Get the display width of this span.
    pub fn width(&self) -> usize {
        UnicodeWidthStr::width(self.text.as_ref())
    }

    /// Check if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Span {
    fn from(text: S) -> Self {
        Span::raw(text)
    }
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Left-aligned (default)
    #[default]
    Left,
    /// Center-aligned
    Center,
    /// Right-aligned
    Right,
}

/// Overflow behavior when text exceeds available width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Wrap to next line (default)
    #[default]
    Wrap,
    /// Truncate with ellipsis
    Ellipsis,
    /// Hard truncate without indicator
    Truncate,
    /// Allow overflow
    Visible,
}

/// A text container with multiple styled spans.
#[derive(Debug, Clone, Default)]
pub struct Text {
    /// The spans that make up this text
    pub spans: Vec<Span>,
    /// Text alignment
    pub alignment: Alignment,
    /// Overflow behavior
    pub overflow: Overflow,
    /// Optional style applied to the whole text
    pub style: Style,
}

impl Text {
    /// Create a new empty text.
    pub fn new() -> Self {
        Text::default()
    }

    /// Create text from a plain string.
    pub fn plain<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Text {
            spans: vec![Span::raw(text)],
            ..Default::default()
        }
    }

    /// Create text from a styled string.
    pub fn styled<S: Into<Cow<'static, str>>>(text: S, style: Style) -> Self {
        Text {
            spans: vec![Span::styled(text, style)],
            style,
            ..Default::default()
        }
    }

    /// Create text from multiple spans.
    pub fn from_spans<I: IntoIterator<Item = Span>>(spans: I) -> Self {
        Text {
            spans: spans.into_iter().collect(),
            ..Default::default()
        }
    }

    /// Add a span to the text.
    pub fn push_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    /// Append plain text.
    pub fn push<S: Into<Cow<'static, str>>>(&mut self, text: S) {
        self.spans.push(Span::raw(text));
    }

    /// Append styled text.
    pub fn push_styled<S: Into<Cow<'static, str>>>(&mut self, text: S, style: Style) {
        self.spans.push(Span::styled(text, style));
    }

    /// Set the alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the overflow behavior.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    /// Disable word wrapping (equivalent to `overflow(Overflow::Visible)`).
    ///
    /// When set, text will extend beyond the available width instead of wrapping.
    pub fn no_wrap(self) -> Self {
        self.overflow(Overflow::Visible)
    }

    /// Set the overall style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get the total display width (without line breaks).
    pub fn width(&self) -> usize {
        self.spans.iter().map(|s| s.width()).sum()
    }

    /// Get the plain text content without styling.
    pub fn plain_text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_ref()).collect()
    }

    /// Check if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|s| s.is_empty())
    }

    /// Split the text into lines, wrapping at the given width.
    ///
    /// Embedded `\n` characters are treated as hard line breaks; the
    /// remaining text is then laid out according to the overflow mode.
    pub fn wrap(&self, width: usize) -> Vec<Vec<Span>> {
        if width == 0 {
            return vec![];
        }

        let logical_lines = split_on_newlines(&self.spans);

        match self.overflow {
            Overflow::Visible => logical_lines,
            Overflow::Truncate | Overflow::Ellipsis => logical_lines
                .into_iter()
                .map(|line| truncate_line(&line, width, self.overflow == Overflow::Ellipsis))
                .collect(),
            Overflow::Wrap => logical_lines
                .into_iter()
                .flat_map(|line| wrap_line(&line, width))
                .collect(),
        }
    }

    /// Apply alignment to a line, returning padded spans.
    pub fn align_line(&self, line: Vec<Span>, width: usize) -> Vec<Span> {
        let line_width: usize = line.iter().map(|s| s.width()).sum();

        if line_width >= width {
            return line;
        }

        let padding = width - line_width;

        match self.alignment {
            Alignment::Left => {
                // Don't add padding for left alignment (professional behavior)
                // This prevents visual artifacts and matches Python rich
                line
            }
            Alignment::Right => {
                let mut result = vec![Span::raw(" ".repeat(padding))];
                result.extend(line);
                result
            }
            Alignment::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                let mut result = vec![Span::raw(" ".repeat(left_pad))];
                result.extend(line);
                result.push(Span::raw(" ".repeat(right_pad)));
                result
            }
        }
    }
}

impl<S: Into<Cow<'static, str>>> From<S> for Text {
    fn from(text: S) -> Self {
        Text::plain(text)
    }
}

/// Truncate a string to a given display width.
fn truncate_str(s: &str, max_width: usize) -> &str {
    let mut width = 0;
    let mut end = 0;

    for grapheme in s.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if width + grapheme_width > max_width {
            break;
        }
        width += grapheme_width;
        end += grapheme.len();
    }

    &s[..end]
}

/// A whitespace-separated lexical unit produced by [`tokenize`].
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'a> {
    /// A run of one or more non-whitespace characters.
    Word(&'a str),
    /// A run of whitespace (excluding newlines), preserved verbatim.
    Spaces(&'a str),
    /// A hard line break (`\n`).
    Newline,
}

/// Split a string into words, whitespace runs, and hard line breaks.
fn tokenize(s: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut run: Option<(usize, TokenKind)> = None;

    for (i, c) in s.char_indices() {
        if c == '\n' {
            if let Some((start, kind)) = run.take() {
                tokens.push(make_token(&s[start..i], kind));
            }
            tokens.push(Token::Newline);
            continue;
        }
        let kind = if c.is_whitespace() {
            TokenKind::Spaces
        } else {
            TokenKind::Word
        };
        match run {
            Some((_, k)) if k == kind => {}
            Some((start, k)) => {
                tokens.push(make_token(&s[start..i], k));
                run = Some((i, kind));
            }
            None => run = Some((i, kind)),
        }
    }
    if let Some((start, kind)) = run {
        tokens.push(make_token(&s[start..], kind));
    }

    tokens
}

fn make_token(text: &str, kind: TokenKind) -> Token<'_> {
    match kind {
        TokenKind::Spaces => Token::Spaces(text),
        _ => Token::Word(text),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenKind {
    Word,
    Spaces,
}

/// Split spans into logical lines at embedded `\n` characters.
fn split_on_newlines(spans: &[Span]) -> Vec<Vec<Span>> {
    let mut lines: Vec<Vec<Span>> = vec![Vec::new()];

    for span in spans {
        let mut rest: &str = &span.text;
        loop {
            if let Some(pos) = rest.find('\n') {
                let (head, tail) = rest.split_at(pos);
                if !head.is_empty() {
                    lines
                        .last_mut()
                        .expect("lines always has a current line")
                        .push(Span {
                            text: head.to_string().into(),
                            style: span.style,
                            link: span.link.clone(),
                        });
                }
                lines.push(Vec::new());
                rest = &tail[1..];
            } else {
                if !rest.is_empty() {
                    lines
                        .last_mut()
                        .expect("lines always has a current line")
                        .push(Span {
                            text: rest.to_string().into(),
                            style: span.style,
                            link: span.link.clone(),
                        });
                }
                break;
            }
        }
    }

    lines
}

/// Truncate a single logical line to `width` display columns.
fn truncate_line(line: &[Span], width: usize, ellipsis: bool) -> Vec<Span> {
    let line_width: usize = line.iter().map(|s| s.width()).sum();
    let mut result = Vec::new();
    let mut remaining_width = if ellipsis && line_width > width {
        width.saturating_sub(1)
    } else {
        width
    };

    for span in line {
        if remaining_width == 0 {
            break;
        }

        let span_width = span.width();
        if span_width <= remaining_width {
            result.push(span.clone());
            remaining_width -= span_width;
        } else {
            // Truncate this span (keep its hyperlink)
            let truncated = truncate_str(&span.text, remaining_width);
            let mut truncated_span = Span::styled(truncated.to_string(), span.style);
            truncated_span.link = span.link.clone();
            result.push(truncated_span);
            remaining_width = 0;
        }
    }

    if ellipsis && line_width > width {
        result.push(Span::raw("…"));
    }

    result
}

/// Wrap a single logical line (no embedded newlines) at `max_width`.
///
/// Words are kept intact when possible, whitespace runs between words are
/// preserved, and a line never exceeds `max_width` display columns. Adjacent
/// fragments sharing the same style are merged into one span.
fn wrap_line(line: &[Span], max_width: usize) -> Vec<Vec<Span>> {
    let mut lines: Vec<Vec<Span>> = Vec::new();
    let mut current_line: Vec<Span> = Vec::new();
    let mut current_width = 0usize;
    // Whitespace run awaiting the next word; emitted (and counted) only if
    // a word actually follows on the same line, so lines never end up one
    // column over max_width from a trailing space. The spaces keep the style
    // of the span they came from (matches Rich: separators are not styled by
    // the following word).
    let mut pending_spaces: Option<(&str, Style)> = None;

    for span in line {
        for token in tokenize(&span.text) {
            match token {
                Token::Newline => { /* unreachable: handled by split_on_newlines */ }
                Token::Spaces(s) => {
                    if current_width > 0 {
                        pending_spaces = Some((s, span.style));
                    }
                    // Spaces at the start of a line are dropped (except the
                    // very first fragment, which is kept below via current_width == 0
                    // only when the line is brand new and nothing flushed yet).
                    if current_width == 0 && lines.is_empty() && current_line.is_empty() {
                        // Preserve leading indentation of the first line.
                        push_fragment(&mut current_line, s, span.style, span.link.as_deref());
                        current_width += UnicodeWidthStr::width(s);
                    }
                }
                Token::Word(word) => {
                    let word_width = UnicodeWidthStr::width(word);

                    if word_width > max_width {
                        // Word is too long for any line: start fresh, then break it.
                        if current_width > 0 {
                            lines.push(std::mem::take(&mut current_line));
                            current_width = 0;
                        }
                        pending_spaces = None;

                        let broken = break_word(word, max_width);
                        for part in broken.iter() {
                            if current_width + UnicodeWidthStr::width(part.as_str()) > max_width {
                                lines.push(std::mem::take(&mut current_line));
                                current_width = 0;
                            }
                            push_fragment(
                                &mut current_line,
                                part,
                                span.style,
                                span.link.as_deref(),
                            );
                            current_width += UnicodeWidthStr::width(part.as_str());
                        }
                    } else {
                        let spaces_width = pending_spaces
                            .map(|(s, _)| UnicodeWidthStr::width(s))
                            .unwrap_or(0);
                        if current_width + spaces_width + word_width > max_width
                            && current_width > 0
                        {
                            // Wrap: pending spaces are dropped at the line end.
                            lines.push(std::mem::take(&mut current_line));
                            current_width = 0;
                            pending_spaces = None;
                        } else if let Some((s, style)) = pending_spaces.take() {
                            push_fragment(&mut current_line, s, style, None);
                            current_width += spaces_width;
                        }

                        push_fragment(&mut current_line, word, span.style, span.link.as_deref());
                        current_width += word_width;
                    }
                }
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(Vec::new());
    }

    lines
}

/// Append a text fragment to a line, merging into the previous span when it
/// has the same style and neither span carries a hyperlink (keeps span
/// counts proportional to style changes, not to word counts).
fn push_fragment(line: &mut Vec<Span>, fragment: &str, style: Style, link: Option<&str>) {
    if let Some(last) = line.last_mut() {
        if last.style == style && last.link.is_none() && link.is_none() {
            last.text.to_mut().push_str(fragment);
            return;
        }
    }
    let mut span = Span::styled(fragment.to_string(), style);
    span.link = link.map(|s| s.to_string());
    line.push(span);
}

/// Break a long word into parts that fit within max_width.
fn break_word(word: &str, max_width: usize) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;

    for grapheme in word.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);

        if current_width + grapheme_width > max_width && !current.is_empty() {
            parts.push(std::mem::take(&mut current));
            current_width = 0;
        }

        current.push_str(grapheme);
        current_width += grapheme_width;
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_width() {
        assert_eq!(Span::raw("hello").width(), 5);
        assert_eq!(Span::raw("你好").width(), 4); // Chinese characters are double-width
        assert_eq!(Span::raw("").width(), 0);
    }

    #[test]
    fn test_text_plain() {
        let text = Text::plain("Hello, World!");
        assert_eq!(text.plain_text(), "Hello, World!");
        assert_eq!(text.width(), 13);
    }

    #[test]
    fn test_text_wrap_simple() {
        let text = Text::plain("hello world");
        let lines = text.wrap(6);
        assert_eq!(lines.len(), 2);
        let first: String = lines[0].iter().map(|s| s.text.as_ref()).collect();
        let second: String = lines[1].iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(first, "hello");
        assert_eq!(second, "world");
    }

    #[test]
    fn test_wrap_preserves_newlines() {
        let text = Text::plain("a\nb");
        let lines = text.wrap(20);
        let plain: Vec<String> = lines
            .iter()
            .map(|l| l.iter().map(|s| s.text.as_ref()).collect())
            .collect();
        assert_eq!(plain, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_wrap_multi_line_styled() {
        let mut text = Text::new();
        text.push_styled("red1", Style::new().foreground(crate::style::Color::Red));
        text.push("\nmore");
        let lines = text.wrap(40);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0][0].style.foreground, Some(crate::style::Color::Red));
        assert_eq!(lines[1][0].text, "more");
    }

    #[test]
    fn test_wrap_preserves_inner_spaces() {
        let text = Text::plain("a  b");
        let lines = text.wrap(20);
        let plain: String = lines[0].iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(plain, "a  b");
    }

    #[test]
    fn test_wrap_exact_fit_has_no_trailing_space() {
        // "one two" is exactly 7 columns; the line must not become 8 wide
        let text = Text::plain("one two three");
        let lines = text.wrap(7);
        assert_eq!(lines.len(), 2);
        let first: String = lines[0].iter().map(|s| s.text.as_ref()).collect();
        let second: String = lines[1].iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(first, "one two");
        assert_eq!(UnicodeWidthStr::width(first.as_str()), 7);
        assert_eq!(second, "three");
    }

    #[test]
    fn test_wrap_merges_adjacent_same_style_spans() {
        let text = Text::plain("the quick brown fox jumps over the lazy dog");
        let lines = text.wrap(80);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].len(), 1, "single style should merge into one span");
    }

    #[test]
    fn test_visible_overflow_splits_newlines() {
        let text = Text::plain("a\nb\nc").overflow(Overflow::Visible);
        let lines = text.wrap(80);
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_text_wrap_long_word() {
        let text = Text::plain("supercalifragilistic");
        let lines = text.wrap(10);
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_truncate_ellipsis() {
        let text = Text::plain("Hello, World!").overflow(Overflow::Ellipsis);
        let lines = text.wrap(8);
        let plain: String = lines[0].iter().map(|s| s.text.as_ref()).collect();
        assert!(plain.ends_with('…'));
        // Check display width, not byte length (ellipsis is 3 bytes but 1 char width)
        assert!(UnicodeWidthStr::width(plain.as_str()) <= 8);
    }

    #[test]
    fn test_alignment_left() {
        let text = Text::plain("hi").alignment(Alignment::Left);
        let lines = text.wrap(10);
        let aligned = text.align_line(lines[0].clone(), 10);
        let plain: String = aligned.iter().map(|s| s.text.as_ref()).collect();
        // Left alignment no longer adds padding (professional behavior)
        assert_eq!(plain, "hi");
    }

    #[test]
    fn test_alignment_right() {
        let text = Text::plain("hi").alignment(Alignment::Right);
        let lines = text.wrap(10);
        let aligned = text.align_line(lines[0].clone(), 10);
        let plain: String = aligned.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(plain, "        hi");
    }

    #[test]
    fn test_alignment_center() {
        let text = Text::plain("hi").alignment(Alignment::Center);
        let lines = text.wrap(10);
        let aligned = text.align_line(lines[0].clone(), 10);
        let plain: String = aligned.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(plain, "    hi    ");
    }
}
