//! Rich's `Text` API on top of fast-rich's chunked [`Text`].
//!
//! Python Rich stores one plain string plus `(start, end, style)` ranges;
//! fast-rich stores a list of [`Span`]s that each own their piece of text.
//! The methods here give `Text` the same editing API as Rich (styling
//! ranges, dividing, cropping, padding, joining, ...) by splitting and
//! re-merging spans as needed.
//!
//! # Offsets and widths
//!
//! * **Offsets** (`stylize`, `divide`, `get_style_at_offset`, ...) are
//!   **byte offsets** into [`Text::plain_text`], the same numbers that
//!   `str::find` returns. An offset that is not on a `char` boundary is
//!   snapped *down* to the start of that character, so no method panics on
//!   a bad offset. Offsets past the end are clamped to the end.
//! * **Widths** (`truncate`, `set_length`, `fit`, `rstrip_end`,
//!   `expand_tabs`) are terminal cells, measured with `unicode-width` like
//!   the rest of the crate, except that control characters such as `\n`
//!   and `\t` count as 0 cells (as in Rich's `cell_len`).
//! * **Counts** (`right_crop`, `pad*`) are numbers of characters, as in Rich.
//!
//! Styles are layered with [`Style::combine`]: `stylize` puts the new style
//! on top of what is there, `stylize_before` puts it underneath. The
//! whole-text [`Text::style`](struct@Text) field is the base layer, under
//! every span (as in Rich); [`Text::get_style_at_offset`] returns the full
//! stack for one character.

use super::{Overflow, Span, Text};
use crate::style::Style;
use std::borrow::{Borrow, Cow};
use std::ops::Range;
use unicode_width::UnicodeWidthChar;

/// Python's `str.isspace()`: Rust's `char::is_whitespace` plus the four
/// ASCII information separators (U+001C..U+001F).
fn is_py_space(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

pub(crate) fn char_cells(c: char) -> usize {
    UnicodeWidthChar::width(c).unwrap_or(0)
}

/// Cells of a string as Rich's `cell_len` measures it: control characters
/// (`\n`, `\t`, ...) and other zero-width characters take no cells
/// (`UnicodeWidthStr` would count control characters as 1), and a
/// character after a zero-width joiner is part of the joined grapheme.
pub(crate) fn str_cells(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        width += char_cells(c);
        if c == '\u{200d}' {
            chars.next();
        }
    }
    width
}

/// Crop `text` to exactly `total` cells (Rich's `set_cell_size` for text
/// wider than `total`). As in Rich, zero-width characters stay with the
/// character before them, and a double-width character that straddles the
/// edge is replaced by a space so the result is exactly `total` cells.
pub(crate) fn crop_cells(text: &str, total: usize) -> String {
    let mut out = String::new();
    if total == 0 {
        return out;
    }
    let mut width = 0;
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        let w = char_cells(c);
        if w > 0 {
            if width == total {
                return out;
            }
            if width + w > total {
                out.push_str(&" ".repeat(total - width));
                return out;
            }
            width += w;
        }
        out.push(c);
        if c == '\u{200d}' {
            out.extend(chars.next());
        }
    }
    out
}

/// Builds a span list, merging neighbours that share style and link.
#[derive(Default)]
struct SpanBuilder {
    spans: Vec<Span>,
}

impl SpanBuilder {
    fn push_str(&mut self, text: &str, style: Style, link: Option<&str>) {
        if text.is_empty() {
            return;
        }
        if let Some(last) = self.spans.last_mut() {
            if last.style == style && last.link.as_deref() == link {
                last.text.to_mut().push_str(text);
                return;
            }
        }
        self.spans.push(Span {
            text: Cow::Owned(text.to_string()),
            style,
            link: link.map(str::to_string),
        });
    }

    fn push_span(&mut self, span: Span) {
        if span.text.is_empty() {
            return;
        }
        if let Some(last) = self.spans.last_mut() {
            if last.style == span.style && last.link == span.link {
                last.text.to_mut().push_str(&span.text);
                return;
            }
        }
        self.spans.push(span);
    }
}

impl Text {
    /// Length of the text in **bytes** (the unit of every offset in this
    /// API). Use [`Text::width`] for terminal cells.
    ///
    /// ```
    /// use fast_rich::text::Text;
    /// assert_eq!(Text::plain("日本").len(), 6);
    /// assert_eq!(Text::plain("日本").width(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.spans.iter().map(|s| s.text.len()).sum()
    }

    // ----------------------------------------------------------------
    // Internal helpers
    // ----------------------------------------------------------------

    /// Clamp `offset` to the text and snap it down to a char boundary.
    fn snap(&self, offset: usize) -> usize {
        let mut pos = 0;
        for span in &self.spans {
            let len = span.text.len();
            if offset < pos + len {
                let mut local = offset - pos;
                while !span.text.is_char_boundary(local) {
                    local -= 1;
                }
                return pos + local;
            }
            pos += len;
        }
        pos
    }

    /// Make sure a span boundary falls at `offset` (a char boundary).
    fn split_spans_at(&mut self, offset: usize) {
        let mut pos = 0;
        for i in 0..self.spans.len() {
            let len = self.spans[i].text.len();
            if offset > pos && offset < pos + len {
                let local = offset - pos;
                let span = &mut self.spans[i];
                let right = span.text[local..].to_string();
                span.text.to_mut().truncate(local);
                let new = Span {
                    text: Cow::Owned(right),
                    style: span.style,
                    link: span.link.clone(),
                };
                self.spans.insert(i + 1, new);
                return;
            }
            pos += len;
        }
    }

    /// Drop empty spans and merge neighbours with equal style and link.
    fn normalize(&mut self) {
        let mut builder = SpanBuilder::default();
        for span in std::mem::take(&mut self.spans) {
            builder.push_span(span);
        }
        self.spans = builder.spans;
    }

    /// Apply `f` to every span inside `start..end` (byte offsets, snapped),
    /// splitting spans at the edges first.
    fn apply_range(&mut self, start: usize, end: usize, mut f: impl FnMut(&mut Span)) {
        let start = self.snap(start);
        let end = self.snap(end);
        if start >= end {
            return;
        }
        self.split_spans_at(start);
        self.split_spans_at(end);
        let mut pos = 0;
        for span in &mut self.spans {
            let len = span.text.len();
            if pos >= start && pos + len <= end && len > 0 {
                f(span);
            }
            pos += len;
        }
        self.normalize();
    }

    /// The spans covering `start..end` (char-boundary byte offsets).
    fn slice_spans(&self, start: usize, end: usize) -> Vec<Span> {
        let mut out = Vec::new();
        let mut pos = 0;
        for span in &self.spans {
            let len = span.text.len();
            let (s, e) = (start.max(pos), end.min(pos + len));
            if s < e {
                out.push(Span {
                    text: Cow::Owned(span.text[s - pos..e - pos].to_string()),
                    style: span.style,
                    link: span.link.clone(),
                });
            }
            pos += len;
        }
        out
    }

    /// Cut the text at byte offset `at` (a char boundary).
    fn truncate_bytes(&mut self, at: usize) {
        let mut pos = 0;
        let mut keep = 0;
        for (i, span) in self.spans.iter_mut().enumerate() {
            let len = span.text.len();
            if pos + len <= at {
                pos += len;
                keep = i + 1;
                continue;
            }
            let local = at - pos;
            if local > 0 {
                span.text.to_mut().truncate(local);
                keep = i + 1;
            }
            break;
        }
        self.spans.truncate(keep);
    }

    /// Replace the plain text, keeping styles by character index (Rich's
    /// `plain` setter): character `i` of the new text gets the style and
    /// link of character `i` of the old text; extra characters are
    /// unstyled.
    fn replace_plain_by_index(&mut self, new_plain: &str) {
        let mut builder = SpanBuilder::default();
        {
            let mut old = self
                .spans
                .iter()
                .flat_map(|s| s.text.chars().map(move |_| (s.style, s.link.as_deref())));
            let mut buf = [0u8; 4];
            for c in new_plain.chars() {
                let (style, link) = old.next().unwrap_or((Style::new(), None));
                builder.push_str(c.encode_utf8(&mut buf), style, link);
            }
        }
        self.spans = builder.spans;
    }

    /// A copy with the same style, alignment and overflow but new spans.
    fn with_spans(&self, spans: Vec<Span>) -> Text {
        Text {
            spans,
            alignment: self.alignment,
            overflow: self.overflow,
            style: self.style,
        }
    }

    // ----------------------------------------------------------------
    // Styling ranges
    // ----------------------------------------------------------------

    /// Apply `style` to the bytes `start..end`, on top of the styles
    /// already there (Rich's `Text.stylize`).
    ///
    /// `end` is clamped to the text, so `usize::MAX` means "to the end".
    /// Offsets inside a multibyte character snap down to its start. An
    /// empty style or an empty range does nothing. Links are kept.
    ///
    /// ```
    /// use fast_rich::style::{Color, Style};
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("hello world");
    /// text.stylize(Style::new().bold(), 0, 5);
    /// assert!(text.get_style_at_offset(0).bold);
    /// assert!(!text.get_style_at_offset(6).bold);
    /// ```
    pub fn stylize(&mut self, style: Style, start: usize, end: usize) {
        if style.is_empty() || start >= self.len() {
            return;
        }
        self.apply_range(start, end, |span| span.style = span.style.combine(&style));
    }

    /// Like [`Text::stylize`], but the style goes *under* the existing
    /// styles, so they win where both set the same attribute (Rich's
    /// `Text.stylize_before`).
    ///
    /// ```
    /// use fast_rich::style::{Color, Style};
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::styled("hi", Style::new().foreground(Color::Red));
    /// text.stylize_before(Style::new().foreground(Color::Blue).bold(), 0, 2);
    /// let style = text.get_style_at_offset(0);
    /// assert_eq!(style.foreground, Some(Color::Red)); // existing red wins
    /// assert!(style.bold); // new attributes still apply
    /// ```
    pub fn stylize_before(&mut self, style: Style, start: usize, end: usize) {
        if style.is_empty() || start >= self.len() {
            return;
        }
        self.apply_range(start, end, |span| span.style = style.combine(&span.style));
    }

    /// Style every occurrence of any of `words` and return how many were
    /// found (Rich's `Text.highlight_words`).
    ///
    /// Matching works like Rich's regex alternation `word1|word2|...`:
    /// scanning left to right, the first word in `words` that matches at a
    /// position wins, and matches do not overlap. With
    /// `case_sensitive = false`, characters are compared by their simple
    /// lowercase form. Empty words are ignored (Rich would match the empty
    /// string at every position).
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("Error: disk error");
    /// let n = text.highlight_words(["error"], Style::new().bold(), false);
    /// assert_eq!(n, 2);
    /// assert!(text.get_style_at_offset(0).bold);
    /// ```
    pub fn highlight_words<I, S>(&mut self, words: I, style: Style, case_sensitive: bool) -> usize
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let words: Vec<S> = words.into_iter().collect();
        let words: Vec<&str> = words
            .iter()
            .map(AsRef::as_ref)
            .filter(|w| !w.is_empty())
            .collect();
        let plain = self.plain_text();
        let matches = find_words(&plain, &words, case_sensitive);
        for &(start, end) in &matches {
            self.stylize(style, start, end);
        }
        matches.len()
    }

    /// Style each byte range in `ranges` and return how many non-empty
    /// ranges were styled. This is the regex-free way to highlight matches
    /// you found yourself, e.g. with `str::match_indices`.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("a=1, b=2");
    /// let ranges = text.plain_text().match_indices('=').map(|(i, m)| i..i + m.len()).collect::<Vec<_>>();
    /// assert_eq!(text.highlight_matches(ranges, Style::new().bold()), 2);
    /// assert!(text.get_style_at_offset(1).bold);
    /// ```
    pub fn highlight_matches<I>(&mut self, ranges: I, style: Style) -> usize
    where
        I: IntoIterator<Item = Range<usize>>,
    {
        let len = self.len();
        let mut count = 0;
        for range in ranges {
            let (start, end) = (self.snap(range.start), self.snap(range.end.min(len)));
            if start < end {
                self.stylize(style, start, end);
                count += 1;
            }
        }
        count
    }

    /// Style every match of `re` and return the number of matches (Rich's
    /// `Text.highlight_regex` with a whole-match style). Empty matches are
    /// counted but style nothing. Rich's "named group = style name" feature
    /// is not supported (fast-rich has no theme lookup inside `Text`).
    ///
    /// Requires the `regex` feature.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let re = regex::Regex::new(r"\d+").unwrap();
    /// let mut text = Text::plain("port 8080 and 443");
    /// assert_eq!(text.highlight_regex(&re, Style::new().bold()), 2);
    /// assert!(text.get_style_at_offset(5).bold);
    /// ```
    #[cfg(feature = "regex")]
    pub fn highlight_regex(&mut self, re: &regex::Regex, style: Style) -> usize {
        let plain = self.plain_text();
        let ranges: Vec<Range<usize>> = re.find_iter(&plain).map(|m| m.range()).collect();
        let count = ranges.len();
        for range in ranges {
            if !range.is_empty() {
                self.stylize(style, range.start, range.end);
            }
        }
        count
    }

    /// Copy the span styles (and links) of `other` onto this text, by
    /// byte offset, on top of the existing styles (Rich's
    /// `Text.copy_styles`). `other` should have the same plain text; spans
    /// past the end of this text are clipped. `other.style` is not copied.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let mut styled = Text::plain("hello");
    /// styled.stylize(Style::new().italic(), 1, 3);
    /// let mut text = Text::plain("hello");
    /// text.copy_styles(&styled);
    /// assert!(text.get_style_at_offset(1).italic);
    /// ```
    pub fn copy_styles(&mut self, other: &Text) {
        let mut pos = 0;
        for span in &other.spans {
            let (start, end) = (pos, pos + span.text.len());
            pos = end;
            if span.style.is_empty() && span.link.is_none() {
                continue;
            }
            if start >= self.len() {
                break;
            }
            self.apply_range(start, end, |s| {
                s.style = s.style.combine(&span.style);
                if span.link.is_some() {
                    s.link = span.link.clone();
                }
            });
        }
    }

    /// The full style of the character at byte `offset`: the text's base
    /// [`style`](struct@Text) with the span style layered on top (Rich's
    /// `Text.get_style_at_offset`). Past the end, only the base style.
    ///
    /// ```
    /// use fast_rich::style::{Color, Style};
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("ab").style(Style::new().italic());
    /// text.stylize(Style::new().foreground(Color::Red), 1, 2);
    /// let s = text.get_style_at_offset(1);
    /// assert!(s.italic && s.foreground == Some(Color::Red));
    /// assert_eq!(text.get_style_at_offset(99), Style::new().italic());
    /// ```
    pub fn get_style_at_offset(&self, offset: usize) -> Style {
        let offset = self.snap(offset);
        let mut pos = 0;
        for span in &self.spans {
            let len = span.text.len();
            if offset < pos + len {
                return self.style.combine(&span.style);
            }
            pos += len;
        }
        self.style
    }

    // ----------------------------------------------------------------
    // Building
    // ----------------------------------------------------------------

    /// Build a text from parts: anything that converts into [`Text`]
    /// (`&str`, `String`, [`Span`], `Text`) (Rich's `Text.assemble`). Each
    /// part's base style is kept under its spans. For `(str, style)` pairs
    /// use [`Text::append_tokens`].
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::{Span, Text};
    ///
    /// let text = Text::assemble([
    ///     Text::from(Span::styled("Error", Style::new().bold())),
    ///     Text::from(": disk full"),
    /// ]);
    /// assert_eq!(text.plain_text(), "Error: disk full");
    /// assert!(text.get_style_at_offset(0).bold);
    /// ```
    pub fn assemble<I>(parts: I) -> Text
    where
        I: IntoIterator,
        I::Item: Into<Text>,
    {
        let mut text = Text::new();
        for part in parts {
            text.append_text(&part.into());
        }
        text
    }

    /// Append another text, keeping its spans and links; its base `style`
    /// is applied under its spans (Rich's `Text.append_text`). Returns
    /// `self` for chaining.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("a ");
    /// text.append_text(&Text::styled("b", Style::new().bold()))
    ///     .append_text(&Text::plain("c"));
    /// assert_eq!(text.plain_text(), "a bc");
    /// assert!(text.get_style_at_offset(2).bold);
    /// ```
    pub fn append_text(&mut self, text: &Text) -> &mut Self {
        let mut builder = SpanBuilder {
            spans: std::mem::take(&mut self.spans),
        };
        for span in &text.spans {
            builder.push_str(
                &span.text,
                text.style.combine(&span.style),
                span.link.as_deref(),
            );
        }
        self.spans = builder.spans;
        self
    }

    /// Append `(text, style)` pairs (Rich's `Text.append_tokens`). Use
    /// `Style::new()` for unstyled text. Returns `self` for chaining.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::new();
    /// text.append_tokens([("key", Style::new().bold()), ("=", Style::new()), ("1", Style::new())]);
    /// assert_eq!(text.plain_text(), "key=1");
    /// ```
    pub fn append_tokens<I, S>(&mut self, tokens: I) -> &mut Self
    where
        I: IntoIterator<Item = (S, Style)>,
        S: Into<Cow<'static, str>>,
    {
        let mut builder = SpanBuilder {
            spans: std::mem::take(&mut self.spans),
        };
        for (content, style) in tokens {
            builder.push_span(Span::styled(content, style));
        }
        self.spans = builder.spans;
        self
    }

    /// Join `texts` with this text as the separator (Rich's `Text.join`).
    ///
    /// The result takes the separator's base style, alignment and overflow.
    /// Can also be called as `Text::join(&separator, texts)`.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let sep = Text::plain(", ");
    /// let joined = sep.join([Text::plain("a"), Text::plain("b"), Text::plain("c")]);
    /// assert_eq!(joined.plain_text(), "a, b, c");
    /// ```
    pub fn join<I>(&self, texts: I) -> Text
    where
        I: IntoIterator,
        I::Item: Borrow<Text>,
    {
        let mut out = self.with_spans(Vec::new());
        let with_separator = !self.is_empty();
        for (i, text) in texts.into_iter().enumerate() {
            if i > 0 && with_separator {
                out.append_text(self);
            }
            out.append_text(text.borrow());
        }
        out
    }

    // ----------------------------------------------------------------
    // Dividing
    // ----------------------------------------------------------------

    /// Cut the text at the given byte offsets and return the pieces
    /// (Rich's `Text.divide`). `n` offsets give `n + 1` pieces; each piece
    /// keeps the styles, links, base style, alignment and overflow.
    ///
    /// Offsets are snapped to char boundaries and clamped to the end. They
    /// should be ascending; a piece whose end is before its start is empty
    /// (as Python slicing would give). No offsets returns a copy.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let parts = Text::plain("abcdef").divide([2, 4]);
    /// let plain: Vec<String> = parts.iter().map(|t| t.plain_text()).collect();
    /// assert_eq!(plain, ["ab", "cd", "ef"]);
    /// ```
    pub fn divide<I>(&self, offsets: I) -> Vec<Text>
    where
        I: IntoIterator<Item = usize>,
    {
        let offsets: Vec<usize> = offsets.into_iter().map(|o| self.snap(o)).collect();
        if offsets.is_empty() {
            return vec![self.clone()];
        }
        let bounds: Vec<usize> = std::iter::once(0)
            .chain(offsets)
            .chain(std::iter::once(self.len()))
            .collect();
        bounds
            .windows(2)
            .map(|w| self.with_spans(self.slice_spans(w[0], w[1])))
            .collect()
    }

    /// Split the text at every occurrence of `separator` (Rich's
    /// `Text.split`). With `include_separator`, each piece keeps the
    /// separator that ended it. A trailing separator does not produce a
    /// final empty piece; an empty separator returns a copy.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let lines = Text::plain("one\ntwo\n").split("\n", false);
    /// let plain: Vec<String> = lines.iter().map(|t| t.plain_text()).collect();
    /// assert_eq!(plain, ["one", "two"]);
    /// ```
    pub fn split(&self, separator: &str, include_separator: bool) -> Vec<Text> {
        self.split_impl(separator, include_separator, false)
    }

    fn split_impl(&self, separator: &str, include_separator: bool, allow_blank: bool) -> Vec<Text> {
        let plain = self.plain_text();
        if separator.is_empty() || !plain.contains(separator) {
            return vec![self.clone()];
        }
        let mut lines = if include_separator {
            self.divide(plain.match_indices(separator).map(|(i, m)| i + m.len()))
        } else {
            self.divide(
                plain
                    .match_indices(separator)
                    .flat_map(|(i, m)| [i, i + m.len()]),
            )
            .into_iter()
            .filter(|line| line.plain_text() != separator)
            .collect()
        };
        if !allow_blank && plain.ends_with(separator) {
            lines.pop();
        }
        lines
    }

    /// Split into lines and set each to exactly `width` cells, cropping or
    /// padding with spaces (Rich's `Text.fit`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let lines = Text::plain("hello\nhi").fit(3);
    /// let plain: Vec<String> = lines.iter().map(|t| t.plain_text()).collect();
    /// assert_eq!(plain, ["hel", "hi "]);
    /// ```
    pub fn fit(&self, width: usize) -> Vec<Text> {
        self.split("\n", false)
            .into_iter()
            .map(|mut line| {
                line.set_length(width);
                line
            })
            .collect()
    }

    // ----------------------------------------------------------------
    // Cropping and padding
    // ----------------------------------------------------------------

    /// Remove `amount` characters from the end (Rich's `Text.right_crop`).
    /// Removing more characters than there are leaves an empty text;
    /// `0` does nothing.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("café!");
    /// text.right_crop(2);
    /// assert_eq!(text.plain_text(), "caf");
    /// ```
    pub fn right_crop(&mut self, amount: usize) {
        if amount == 0 {
            return;
        }
        let plain = self.plain_text();
        let at = plain
            .char_indices()
            .rev()
            .nth(amount - 1)
            .map_or(0, |(i, _)| i);
        self.truncate_bytes(at);
    }

    /// Crop the text to `max_width` cells if it is wider (Rich's
    /// `Text.truncate`).
    ///
    /// `overflow` defaults to the text's own [`Overflow`]. `Ellipsis` and
    /// `WrapEllipsis` end the cropped text with `…`; `Visible` leaves the
    /// text alone; the others crop. A double-width character cut in half
    /// becomes a space. Kept characters keep their styles; the ellipsis
    /// takes the style of the first removed character. With `pad`, a
    /// narrower text is padded with unstyled spaces to `max_width`.
    ///
    /// ```
    /// use fast_rich::text::{Overflow, Text};
    ///
    /// let mut text = Text::plain("Hello, World!");
    /// text.truncate(8, Some(Overflow::Ellipsis), false);
    /// assert_eq!(text.plain_text(), "Hello, …");
    ///
    /// let mut short = Text::plain("hi");
    /// short.truncate(4, None, true);
    /// assert_eq!(short.plain_text(), "hi  ");
    /// ```
    pub fn truncate(&mut self, max_width: usize, overflow: Option<Overflow>, pad: bool) {
        let overflow = overflow.unwrap_or(self.overflow);
        if overflow == Overflow::Visible {
            return;
        }
        let plain = self.plain_text();
        let length = str_cells(&plain);
        if length > max_width {
            let cropped = match overflow {
                Overflow::Ellipsis | Overflow::WrapEllipsis if max_width > 0 => {
                    crop_cells(&plain, max_width - 1) + "…"
                }
                _ => crop_cells(&plain, max_width),
            };
            self.replace_plain_by_index(&cropped);
        }
        if pad && length < max_width {
            self.pad_right(max_width - length, ' ');
        }
    }

    /// Add `count` copies of `character` to both ends, unstyled (Rich's
    /// `Text.pad`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("x");
    /// text.pad(2, '.');
    /// assert_eq!(text.plain_text(), "..x..");
    /// ```
    pub fn pad(&mut self, count: usize, character: char) {
        self.pad_left(count, character);
        self.pad_right(count, character);
    }

    /// Add `count` copies of `character` at the start, unstyled (Rich's
    /// `Text.pad_left`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("7");
    /// text.pad_left(2, '0');
    /// assert_eq!(text.plain_text(), "007");
    /// ```
    pub fn pad_left(&mut self, count: usize, character: char) {
        if count == 0 {
            return;
        }
        let pad = Span::raw(character.to_string().repeat(count));
        self.spans.insert(0, pad);
        self.normalize();
    }

    /// Add `count` copies of `character` at the end, unstyled (Rich's
    /// `Text.pad_right`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("ab");
    /// text.pad_right(2, '-');
    /// assert_eq!(text.plain_text(), "ab--");
    /// ```
    pub fn pad_right(&mut self, count: usize, character: char) {
        if count == 0 {
            return;
        }
        let mut builder = SpanBuilder {
            spans: std::mem::take(&mut self.spans),
        };
        builder.push_span(Span::raw(character.to_string().repeat(count)));
        self.spans = builder.spans;
    }

    /// Make the text exactly `width` cells wide: pad the end with spaces or
    /// crop it (Rich's `Text.set_length`). Unlike Rich, which counts
    /// characters, this counts cells, so double-width text lines up; a
    /// double-width character cut in half becomes a space.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("日本語");
    /// text.set_length(5);
    /// assert_eq!(text.plain_text(), "日本 ");
    /// ```
    pub fn set_length(&mut self, width: usize) {
        let plain = self.plain_text();
        let length = str_cells(&plain);
        if length < width {
            self.pad_right(width - length, ' ');
        } else if length > width {
            let cropped = crop_cells(&plain, width);
            self.replace_plain_by_index(&cropped);
        }
    }

    /// Remove trailing whitespace (Rich's `Text.rstrip`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("done  \n");
    /// text.rstrip();
    /// assert_eq!(text.plain_text(), "done");
    /// ```
    pub fn rstrip(&mut self) {
        let plain = self.plain_text();
        self.truncate_bytes(plain.trim_end_matches(is_py_space).len());
    }

    /// If the text is wider than `size` cells, remove trailing whitespace
    /// until it fits, but never any other characters (Rich's
    /// `Text.rstrip_end`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("ab    ");
    /// text.rstrip_end(4);
    /// assert_eq!(text.plain_text(), "ab  ");
    /// ```
    pub fn rstrip_end(&mut self, size: usize) {
        let plain = self.plain_text();
        let width = str_cells(&plain);
        if width <= size {
            return;
        }
        let excess = width - size;
        let mut removed = 0;
        let mut at = plain.len();
        for (i, c) in plain.char_indices().rev() {
            if removed >= excess || !is_py_space(c) {
                break;
            }
            removed += char_cells(c);
            at = i;
        }
        self.truncate_bytes(at);
    }

    /// Remove `suffix` from the end if the text ends with it (Rich's
    /// `Text.remove_suffix`). An empty suffix does nothing.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("report.txt");
    /// text.remove_suffix(".txt");
    /// assert_eq!(text.plain_text(), "report");
    /// ```
    pub fn remove_suffix(&mut self, suffix: &str) {
        let plain = self.plain_text();
        if !suffix.is_empty() && plain.ends_with(suffix) {
            self.truncate_bytes(plain.len() - suffix.len());
        }
    }

    // ----------------------------------------------------------------
    // Tabs and indentation
    // ----------------------------------------------------------------

    /// Replace tabs with spaces up to the next multiple of `tab_size`
    /// cells, column counting restarting after each `\n` (Rich's
    /// `Text.expand_tabs`). The spaces keep the tab's style. A `tab_size`
    /// of 0 is treated as 1.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let mut text = Text::plain("a\tbc\td");
    /// text.expand_tabs(4);
    /// assert_eq!(text.plain_text(), "a   bc  d");
    /// ```
    pub fn expand_tabs(&mut self, tab_size: usize) {
        if !self.spans.iter().any(|s| s.text.contains('\t')) {
            return;
        }
        let tab_size = tab_size.max(1);
        let mut column = 0;
        for span in &mut self.spans {
            if !span.text.contains('\t') {
                for c in span.text.chars() {
                    column = if c == '\n' { 0 } else { column + char_cells(c) };
                }
                continue;
            }
            let mut out = String::with_capacity(span.text.len());
            for c in span.text.chars() {
                match c {
                    '\t' => {
                        let spaces = tab_size - column % tab_size;
                        out.push_str(&" ".repeat(spaces));
                        column += spaces;
                    }
                    '\n' => {
                        out.push(c);
                        column = 0;
                    }
                    _ => {
                        out.push(c);
                        column += char_cells(c);
                    }
                }
            }
            span.text = Cow::Owned(out);
        }
    }

    /// Guess the indent size of code: the greatest common divisor of the
    /// even leading-space counts of all lines, or 1 (Rich's
    /// `Text.detect_indentation`).
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let code = Text::plain("fn main() {\n    if x {\n        y();\n    }\n}");
    /// assert_eq!(code.detect_indentation(), 4);
    /// ```
    pub fn detect_indentation(&self) -> usize {
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let plain = self.plain_text();
        let indent = plain
            .split('\n')
            .map(|line| line.len() - line.trim_start_matches(' ').len())
            .filter(|n| n % 2 == 0)
            .fold(0, gcd);
        indent.max(1)
    }

    /// Return a copy with indentation drawn as guide lines (Rich's
    /// `Text.with_indent_guides`), for code display.
    ///
    /// Tabs are expanded (8 columns) first. Every full indent level becomes
    /// `character` followed by spaces, styled with `style` on top of what
    /// was there. `indent_size` defaults to
    /// [`Text::detect_indentation`]; `0` is treated as 1. As in Rich,
    /// blank (empty or all-space) lines take the indent of the next
    /// non-blank line, and trailing blank lines become empty.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Text;
    ///
    /// let code = Text::plain("if x:\n    y()\n        z()");
    /// let guided = code.with_indent_guides(Some(4), '│', Style::new().dim());
    /// assert_eq!(guided.plain_text(), "if x:\n│   y()\n│   │   z()");
    /// ```
    pub fn with_indent_guides(
        &self,
        indent_size: Option<usize>,
        character: char,
        style: Style,
    ) -> Text {
        let size = indent_size
            .unwrap_or_else(|| self.detect_indentation())
            .max(1);
        let mut text = self.clone();
        text.expand_tabs(8);
        let indent_line = format!("{character}{}", " ".repeat(size - 1));

        let mut new_lines: Vec<Text> = Vec::new();
        let mut blank_lines = 0;
        for mut line in text.split_impl("\n", false, true) {
            let plain = line.plain_text();
            let rest = plain.trim_start_matches(' ');
            if rest.is_empty() {
                blank_lines += 1;
                continue;
            }
            let indent = plain.len() - rest.len();
            let new_indent = format!(
                "{}{}",
                indent_line.repeat(indent / size),
                " ".repeat(indent % size)
            );
            line.replace_plain_by_index(&format!("{new_indent}{rest}"));
            line.stylize(style, 0, new_indent.len());
            for _ in 0..blank_lines {
                new_lines.push(Text::plain(new_indent.clone()).style(style));
            }
            blank_lines = 0;
            new_lines.push(line);
        }
        for _ in 0..blank_lines {
            new_lines.push(Text::new().style(style));
        }

        let separator = text.with_spans(vec![Span::raw("\n")]);
        separator.join(new_lines)
    }
}

/// Non-overlapping leftmost matches of any of `words` (first word wins at
/// a position), as Python's `re.finditer("w1|w2|...")`.
fn find_words(plain: &str, words: &[&str], case_sensitive: bool) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if words.is_empty() {
        return out;
    }
    let mut i = 0;
    while i < plain.len() {
        let rest = &plain[i..];
        let found = words
            .iter()
            .find_map(|w| match_word(rest, w, case_sensitive));
        match found {
            Some(n) => {
                out.push((i, i + n));
                i += n;
            }
            None => i += rest.chars().next().map_or(1, char::len_utf8),
        }
    }
    out
}

/// If `text` starts with `word`, the number of bytes of `text` matched.
fn match_word(text: &str, word: &str, case_sensitive: bool) -> Option<usize> {
    if case_sensitive {
        return text.starts_with(word).then_some(word.len());
    }
    fn fold(c: char) -> char {
        let mut lower = c.to_lowercase();
        match (lower.next(), lower.next()) {
            (Some(l), None) => l,
            _ => c,
        }
    }
    let mut chars = text.char_indices();
    let mut end = 0;
    for w in word.chars() {
        let (i, c) = chars.next()?;
        if c != w && fold(c) != fold(w) {
            return None;
        }
        end = i + c.len_utf8();
    }
    Some(end)
}

impl From<Span> for Text {
    /// A text made of this one span (no base style).
    fn from(span: Span) -> Self {
        Text::from_spans([span])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn red() -> Style {
        Style::new().foreground(Color::Red)
    }
    fn blue() -> Style {
        Style::new().foreground(Color::Blue)
    }
    fn bold() -> Style {
        Style::new().bold()
    }

    /// (text, style, link) of every span.
    fn spans(t: &Text) -> Vec<(String, Style, Option<String>)> {
        t.spans
            .iter()
            .map(|s| (s.text.to_string(), s.style, s.link.clone()))
            .collect()
    }

    fn plains(texts: &[Text]) -> Vec<String> {
        texts.iter().map(Text::plain_text).collect()
    }

    /// A multibyte sample: "é日本🎉" = 2 + 3 + 3 + 4 bytes.
    const MB: &str = "é日本🎉";

    // ---- len / snap ----

    #[test]
    fn len_counts_bytes() {
        assert_eq!(Text::new().len(), 0);
        assert_eq!(Text::plain(MB).len(), 12);
        assert!(!Text::plain(MB).is_empty());
    }

    // ---- stylize ----

    #[test]
    fn stylize_splits_spans() {
        let mut t = Text::plain("hello world");
        t.stylize(bold(), 2, 7);
        assert_eq!(
            spans(&t),
            [
                ("he".into(), Style::new(), None),
                ("llo w".into(), bold(), None),
                ("orld".into(), Style::new(), None),
            ]
        );
    }

    #[test]
    fn stylize_layers_over_existing() {
        let mut t = Text::from_spans([Span::styled("ab", red()), Span::raw("cd")]);
        t.stylize(blue().bold(), 1, 3);
        // "a" red, "bc" blue (later color wins) + bold, "d" unstyled.
        assert_eq!(
            spans(&t),
            [
                ("a".into(), red(), None),
                ("bc".into(), blue().bold(), None),
                ("d".into(), Style::new(), None),
            ]
        );
    }

    #[test]
    fn stylize_before_keeps_existing_on_top() {
        let mut t = Text::from_spans([Span::styled("ab", red()), Span::raw("cd")]);
        t.stylize_before(blue(), 0, 4);
        let s: Vec<Style> = t.spans.iter().map(|s| s.style).collect();
        assert_eq!(s, [red(), blue()]);
    }

    #[test]
    fn stylize_edge_cases() {
        let mut empty = Text::new();
        empty.stylize(bold(), 0, 10);
        assert!(empty.spans.is_empty());

        let mut t = Text::plain("abc");
        t.stylize(bold(), 5, 10); // start past the end
        t.stylize(bold(), 2, 1); // inverted
        t.stylize(Style::new(), 0, 3); // empty style
        assert_eq!(spans(&t), [("abc".into(), Style::new(), None)]);

        t.stylize(bold(), 1, usize::MAX); // end clamped
        assert_eq!(t.spans[1].text, "bc");
        assert!(t.spans[1].style.bold);
    }

    #[test]
    fn stylize_multibyte_snaps_down() {
        let mut t = Text::plain(MB);
        // 3 is inside "日" (bytes 2..5) -> snaps to 2; 6 is inside "本" -> 5.
        t.stylize(bold(), 3, 6);
        assert_eq!(t.spans[1].text, "日");
        assert!(t.spans[1].style.bold);
        // An end inside the first char snaps to 0: nothing to do.
        let mut u = Text::plain(MB);
        u.stylize(bold(), 0, 1);
        assert_eq!(u.spans.len(), 1);
        assert!(!u.spans[0].style.bold);
        // Whole emoji.
        u.stylize(red(), 8, 12);
        assert_eq!(u.spans.last().unwrap().text, "🎉");
    }

    #[test]
    fn stylize_keeps_links() {
        let mut t = Text::from_spans([Span::linked("click", Style::new(), "https://x.io".into())]);
        t.stylize(bold(), 0, 2);
        assert_eq!(
            spans(&t),
            [
                ("cl".into(), bold(), Some("https://x.io".into())),
                ("ick".into(), Style::new(), Some("https://x.io".into())),
            ]
        );
    }

    #[test]
    fn stylize_merges_back_equal_neighbours() {
        let mut t = Text::plain("abcdef");
        t.stylize(bold(), 0, 2);
        t.stylize(bold(), 2, 4);
        assert_eq!(t.spans.len(), 2);
        assert_eq!(t.spans[0].text, "abcd");
    }

    // ---- highlight ----

    #[test]
    fn highlight_words_basic_and_case() {
        let mut t = Text::plain("Error error ERROR");
        assert_eq!(t.highlight_words(["error"], bold(), true), 1);
        let mut t = Text::plain("Error error ERROR");
        assert_eq!(t.highlight_words(["error"], bold(), false), 3);
        assert!(t.get_style_at_offset(12).bold);
        assert!(!t.get_style_at_offset(5).bold);
    }

    #[test]
    fn highlight_words_alternation_order_and_overlap() {
        // Like regex "ab|abc": the first alternative wins at a position.
        let mut t = Text::plain("abcabc");
        assert_eq!(t.highlight_words(["ab", "abc"], bold(), true), 2);
        assert!(!t.get_style_at_offset(2).bold);
        // Non-overlapping: "aaa" with "aa" matches once.
        let mut t = Text::plain("aaa");
        assert_eq!(t.highlight_words(["aa"], bold(), true), 1);
    }

    #[test]
    fn highlight_words_edge_cases() {
        let mut t = Text::new();
        assert_eq!(t.highlight_words(["x"], bold(), true), 0);
        let mut t = Text::plain("abc");
        assert_eq!(t.highlight_words(Vec::<&str>::new(), bold(), true), 0);
        assert_eq!(t.highlight_words([""], bold(), true), 0);
        let mut t = Text::plain("Été 日本 été");
        assert_eq!(t.highlight_words(["été", "日本"], bold(), false), 3);
        assert!(t.get_style_at_offset(0).bold);
    }

    #[test]
    fn highlight_matches_counts_nonempty() {
        let mut t = Text::plain("abcdef");
        let n = t.highlight_matches([0..1, 2..2, 4..100, 10..12], bold());
        assert_eq!(n, 2);
        assert_eq!(t.plain_text(), "abcdef");
        assert!(t.get_style_at_offset(5).bold);
        let mut m = Text::plain(MB);
        assert_eq!(m.highlight_matches(std::iter::once(0..1), bold()), 0);
    }

    #[cfg(feature = "regex")]
    #[test]
    fn highlight_regex_counts_matches() {
        let mut t = Text::plain("a1b22c");
        let re = regex::Regex::new(r"\d*").unwrap();
        // Empty matches are counted (as in Rich) but style nothing.
        let n = t.highlight_regex(&re, bold());
        assert!(n >= 2);
        assert!(t.get_style_at_offset(1).bold);
        assert!(t.get_style_at_offset(3).bold);
        assert!(!t.get_style_at_offset(0).bold);
    }

    // ---- assemble / append / join ----

    #[test]
    fn assemble_parts() {
        let t = Text::assemble([
            Text::from("a"),
            Text::from(Span::styled("b", bold())),
            Text::plain("c").style(red()),
            Text::new(),
        ]);
        assert_eq!(t.plain_text(), "abc");
        assert_eq!(t.get_style_at_offset(1), bold());
        assert_eq!(t.get_style_at_offset(2), red());
        assert_eq!(t.style, Style::new());
    }

    #[test]
    fn append_text_bakes_base_style_under_spans() {
        let mut other = Text::plain("xy").style(red());
        other.stylize(blue(), 1, 2);
        let mut t = Text::plain("a");
        t.append_text(&other);
        assert_eq!(t.get_style_at_offset(1), red());
        assert_eq!(t.get_style_at_offset(2), blue());
    }

    #[test]
    fn append_text_keeps_links_and_merges() {
        let mut t = Text::plain("a");
        t.append_text(&Text::plain("b"));
        assert_eq!(t.spans.len(), 1);
        t.append_text(&Text::from_spans([Span::linked(
            "c",
            Style::new(),
            "u".into(),
        )]));
        assert_eq!(t.spans.len(), 2);
        assert_eq!(t.spans[1].link.as_deref(), Some("u"));
    }

    #[test]
    fn append_tokens_works() {
        let mut t = Text::new();
        t.append_tokens([("é", bold()), ("", red()), ("日", Style::new())]);
        assert_eq!(t.plain_text(), "é日");
        assert_eq!(t.spans.len(), 2);
    }

    #[test]
    fn join_with_and_without_separator() {
        let parts = [Text::plain("a"), Text::styled("b", bold()), Text::new()];
        let sep = Text::styled(", ", red());
        let j = sep.join(&parts);
        assert_eq!(j.plain_text(), "a, b, ");
        assert_eq!(j.style, red());
        assert_eq!(j.get_style_at_offset(1), red());
        assert_eq!(Text::new().join(&parts).plain_text(), "ab");
        assert_eq!(sep.join(Vec::<Text>::new()).plain_text(), "");
        assert_eq!(Text::join(&sep, [Text::plain("x")]).plain_text(), "x");
    }

    // ---- divide / split / fit ----

    #[test]
    fn divide_keeps_styles() {
        let mut t = Text::plain("abcdef").alignment(super::super::Alignment::Right);
        t.stylize(bold(), 1, 5);
        let parts = t.divide([2, 4]);
        assert_eq!(plains(&parts), ["ab", "cd", "ef"]);
        assert_eq!(parts[0].spans[1].text, "b");
        assert!(parts[1].spans[0].style.bold);
        assert_eq!(parts[2].spans[0].text, "e");
        assert_eq!(parts[2].alignment, super::super::Alignment::Right);
    }

    #[test]
    fn divide_edge_cases() {
        assert_eq!(plains(&Text::plain("abc").divide([])), ["abc"]);
        assert_eq!(plains(&Text::new().divide([0])), ["", ""]);
        assert_eq!(plains(&Text::plain("abc").divide([10])), ["abc", ""]);
        assert_eq!(
            plains(&Text::plain("abcdef").divide([4, 2])),
            ["abcd", "", "cdef"]
        );
        // Multibyte: 3 snaps to 2.
        assert_eq!(plains(&Text::plain(MB).divide([3, 8])), ["é", "日本", "🎉"]);
    }

    #[test]
    fn split_like_rich() {
        let t = Text::plain("a\n\nb\n");
        assert_eq!(plains(&t.split("\n", false)), ["a", "", "b"]);
        assert_eq!(plains(&t.split("\n", true)), ["a\n", "\n", "b\n"]);
        assert_eq!(plains(&t.split_impl("\n", false, true)), ["a", "", "b", ""]);
        assert_eq!(plains(&Text::plain("abc").split(",", false)), ["abc"]);
        assert_eq!(plains(&Text::plain("abc").split("", false)), ["abc"]);
        assert_eq!(plains(&Text::new().split("\n", false)), [""]);
        assert_eq!(
            plains(&Text::plain("日,本").split(",", false)),
            ["日", "本"]
        );
    }

    #[test]
    fn split_keeps_links_across_pieces() {
        let t = Text::from_spans([Span::linked("a b", bold(), "u".into())]);
        let parts = t.split(" ", false);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].spans[0].link.as_deref(), Some("u"));
        assert!(parts[1].spans[0].style.bold);
    }

    #[test]
    fn fit_pads_and_crops_cells() {
        let t = Text::plain("hello\n日本語\n");
        assert_eq!(plains(&t.fit(4)), ["hell", "日本"]);
        assert_eq!(plains(&t.fit(5)), ["hello", "日本 "]);
        assert_eq!(plains(&Text::new().fit(2)), ["  "]);
    }

    // ---- crop / truncate / pad ----

    #[test]
    fn right_crop_chars() {
        let mut t = Text::plain(MB);
        t.right_crop(1);
        assert_eq!(t.plain_text(), "é日本");
        t.right_crop(0);
        assert_eq!(t.plain_text(), "é日本");
        t.right_crop(100);
        assert!(t.spans.is_empty());
        let mut e = Text::new();
        e.right_crop(3);
        assert!(e.is_empty());
    }

    #[test]
    fn truncate_modes() {
        let mut t = Text::plain("hello world");
        t.stylize(bold(), 0, 5);
        t.truncate(5, Some(Overflow::Ellipsis), false);
        assert_eq!(t.plain_text(), "hell…");
        // The ellipsis takes the style of the first removed char ('o').
        assert!(t.get_style_at_offset(4).bold);

        let mut c = Text::plain("hello");
        c.truncate(3, Some(Overflow::Truncate), false);
        assert_eq!(c.plain_text(), "hel");

        let mut v = Text::plain("hello");
        v.truncate(3, Some(Overflow::Visible), true);
        assert_eq!(v.plain_text(), "hello");

        let mut d = Text::plain("hello").overflow(Overflow::Ellipsis);
        d.truncate(2, None, false);
        assert_eq!(d.plain_text(), "h…");

        let mut z = Text::plain("hello");
        z.truncate(0, Some(Overflow::Ellipsis), false);
        assert_eq!(z.plain_text(), "");
    }

    #[test]
    fn truncate_wide_chars_and_pad() {
        let mut t = Text::plain("日本語");
        t.truncate(4, Some(Overflow::Ellipsis), false);
        assert_eq!(t.plain_text(), "日 …");
        let mut t = Text::plain("日本語");
        t.truncate(3, Some(Overflow::Truncate), false);
        assert_eq!(t.plain_text(), "日 ");
        let mut p = Text::styled("é", bold());
        p.truncate(3, None, true);
        assert_eq!(p.plain_text(), "é  ");
        assert!(!p.spans.last().unwrap().style.bold);
        let mut e = Text::new();
        e.truncate(2, None, true);
        assert_eq!(e.plain_text(), "  ");
    }

    #[test]
    fn pad_family() {
        let mut t = Text::styled("x", bold());
        t.pad(1, ' ');
        assert_eq!(t.plain_text(), " x ");
        assert_eq!(
            t.spans.iter().map(|s| s.style.bold).collect::<Vec<_>>(),
            [false, true, false]
        );
        t.pad_left(0, '-');
        t.pad_right(0, '-');
        assert_eq!(t.plain_text(), " x ");
        let mut e = Text::new();
        e.pad_left(2, '日');
        assert_eq!(e.plain_text(), "日日");
    }

    #[test]
    fn set_length_cells() {
        let mut t = Text::plain("abc");
        t.set_length(5);
        assert_eq!(t.plain_text(), "abc  ");
        t.set_length(2);
        assert_eq!(t.plain_text(), "ab");
        t.set_length(2);
        assert_eq!(t.plain_text(), "ab");
        let mut w = Text::plain("🎉🎉");
        w.set_length(3);
        assert_eq!(w.plain_text(), "🎉 ");
        let mut z = Text::plain("abc");
        z.set_length(0);
        assert!(z.is_empty());
    }

    #[test]
    fn rstrip_family() {
        let mut t = Text::from_spans([Span::raw("ab"), Span::styled("  \t\n", bold())]);
        t.rstrip();
        assert_eq!(spans(&t), [("ab".into(), Style::new(), None)]);

        let mut e = Text::plain("   ");
        e.rstrip();
        assert!(e.is_empty());

        let mut r = Text::plain("ab    ");
        r.rstrip_end(10);
        assert_eq!(r.plain_text(), "ab    ");
        r.rstrip_end(3);
        assert_eq!(r.plain_text(), "ab ");
        r.rstrip_end(0);
        assert_eq!(r.plain_text(), "ab");
        let mut f = Text::plain("日本  ");
        f.rstrip_end(5);
        assert_eq!(f.plain_text(), "日本 ");
    }

    #[test]
    fn remove_suffix_cases() {
        let mut t = Text::plain("file.rs");
        t.remove_suffix(".py");
        assert_eq!(t.plain_text(), "file.rs");
        t.remove_suffix("");
        assert_eq!(t.plain_text(), "file.rs");
        t.remove_suffix(".rs");
        assert_eq!(t.plain_text(), "file");
        let mut m = Text::plain(MB);
        m.remove_suffix("本🎉");
        assert_eq!(m.plain_text(), "é日");
        let mut e = Text::new();
        e.remove_suffix("x");
        assert!(e.is_empty());
    }

    // ---- copy_styles / get_style_at_offset ----

    #[test]
    fn copy_styles_layers_and_clips() {
        let mut src = Text::plain("abcdef");
        src.stylize(bold(), 1, 3);
        src.spans.push(Span::linked("ghi", red(), "u".into()));
        let mut t = Text::plain("abcd");
        t.copy_styles(&src);
        assert_eq!(t.plain_text(), "abcd");
        assert!(t.get_style_at_offset(1).bold);
        assert!(!t.get_style_at_offset(3).bold);
        assert!(t.spans.iter().all(|s| s.link.is_none()));

        let mut long = Text::plain("abcdefghij");
        long.copy_styles(&src);
        assert_eq!(long.get_style_at_offset(7), red());
        let ghi = long.spans.iter().find(|s| s.text == "ghi").unwrap();
        assert_eq!(ghi.link.as_deref(), Some("u"));
    }

    #[test]
    fn get_style_at_offset_cases() {
        assert_eq!(Text::new().get_style_at_offset(0), Style::new());
        let t = Text::from_spans([Span::raw("é"), Span::styled("日", bold())]).style(red());
        assert_eq!(t.get_style_at_offset(0), red());
        assert_eq!(t.get_style_at_offset(3), red().bold()); // inside "日"
        assert_eq!(t.get_style_at_offset(5), red());
    }

    // ---- tabs / indentation ----

    #[test]
    fn expand_tabs_columns_and_styles() {
        let mut t = Text::plain("a\tbc\t日\tx");
        t.stylize(red(), 1, 2);
        t.expand_tabs(4);
        assert_eq!(t.plain_text(), "a   bc  日  x");
        assert_eq!(t.spans[1].text, "   ");
        assert_eq!(t.spans[1].style, red());

        let mut nl = Text::from_spans([Span::raw("ab\n"), Span::raw("\tc")]);
        nl.expand_tabs(4);
        assert_eq!(nl.plain_text(), "ab\n    c");

        let mut z = Text::plain("a\tb");
        z.expand_tabs(0);
        assert_eq!(z.plain_text(), "a b");

        let mut none = Text::plain("abc");
        none.expand_tabs(4);
        assert_eq!(none.plain_text(), "abc");
    }

    #[test]
    fn detect_indentation_cases() {
        assert_eq!(Text::new().detect_indentation(), 1);
        assert_eq!(Text::plain("a\n  b\n    c").detect_indentation(), 2);
        assert_eq!(Text::plain("a\n   b").detect_indentation(), 1); // odd ignored
        assert_eq!(Text::plain("a\n    b\n        c").detect_indentation(), 4);
        assert_eq!(Text::plain("a\n\tb").detect_indentation(), 1);
    }

    #[test]
    fn with_indent_guides_blank_lines() {
        let code = Text::plain("a:\n    b\n\n    c\n\n");
        let g = code.with_indent_guides(Some(4), '│', bold());
        assert_eq!(g.plain_text(), "a:\n│   b\n│   \n│   c\n\n");
        // Guide column is styled, the code is not.
        assert!(g.get_style_at_offset(3).bold);
        assert!(!g.get_style_at_offset(3 + "│   ".len()).bold);
        assert_eq!(
            Text::new()
                .with_indent_guides(None, '|', bold())
                .plain_text(),
            ""
        );
    }

    #[test]
    fn with_indent_guides_keeps_code_styles_and_tabs() {
        let mut code = Text::plain("x\n\ty");
        code.stylize(red(), 3, 4);
        let g = code.with_indent_guides(Some(4), '|', bold());
        assert_eq!(g.plain_text(), "x\n|   |   y");
        assert_eq!(g.get_style_at_offset(10), red());
    }
}
