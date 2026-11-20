//! Rich's `Segment` toolkit (`rich.segment`): helpers that split, crop,
//! pad, shape, divide and restyle rendered output.
//!
//! In fast-rich the piece of styled text that Rich calls a `Segment` is a
//! [`Span`] (text, style, optional link), and a rendered line is a
//! `Vec<Span>` (see [`segments_to_lines`](crate::renderable::segments_to_lines)).
//! Rich's `Segment` class methods are associated functions on
//! [`Segment`] that take spans or lines; Rich's per-segment methods
//! (`cell_length`, `is_control`, `split_cells`) are methods on [`Span`].
//!
//! Widths are terminal cells measured like Rich's `cell_len` (control
//! characters count 0).
//!
//! # Control spans
//!
//! Rich marks control segments (cursor moves, bell, title, ...) with a
//! `control` field; they take no cells and are never cropped or styled.
//! A fast-rich [`Span`] has no such field: a span is a control span when
//! its text starts with ESC, BEL or `\r`, which is exactly the text that
//! [`Control`](crate::control::Control) renders (see
//! [`Span::is_control`]). Normal text never starts with those characters.
//!
//! ```
//! use fast_rich::renderable::Segment;
//! use fast_rich::style::Style;
//! use fast_rich::text::Span;
//!
//! let spans = vec![Span::raw("Hello\nWorld, this is long")];
//! let lines = Segment::split_and_crop_lines(&spans, 8, Style::new(), true, false);
//! let plain: Vec<String> = lines.iter().map(|l| Segment::plain(l)).collect();
//! assert_eq!(plain, ["Hello   ", "World, t"]);
//! ```

use crate::renderable::Segment;
use crate::style::Style;
use crate::text::{char_cells, crop_cells, str_cells, Span};

fn with_text(span: &Span, text: String) -> Span {
    Span {
        text: text.into(),
        style: span.style,
        link: span.link.clone(),
    }
}

impl Span {
    /// A span holding a single `\n` (Rich's `Segment.line()`).
    ///
    /// ```
    /// use fast_rich::text::Span;
    /// assert_eq!(Span::line().text, "\n");
    /// ```
    pub fn line() -> Span {
        Span::raw("\n")
    }

    /// Whether this is a control span: its text starts with ESC, BEL or
    /// `\r`, as everything [`Control`](crate::control::Control) renders
    /// does (Rich's `Segment.is_control`; see the
    /// [`segment`](crate::segment) module).
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// use fast_rich::text::Span;
    /// assert!(Span::raw(Control::home().to_string()).is_control());
    /// assert!(!Span::raw("home").is_control());
    /// ```
    pub fn is_control(&self) -> bool {
        self.text.starts_with(['\x1b', '\x07', '\r'])
    }

    /// Cells the span takes in the terminal (Rich's
    /// `Segment.cell_length`): 0 for a control span, otherwise the cell
    /// width with control characters counted as 0.
    ///
    /// ```
    /// use fast_rich::text::Span;
    /// assert_eq!(Span::raw("日本\n").cell_length(), 4);
    /// assert_eq!(Span::raw("\x1b[H").cell_length(), 0);
    /// ```
    pub fn cell_length(&self) -> usize {
        if self.is_control() {
            0
        } else {
            str_cells(&self.text)
        }
    }

    /// Split the span in two at cell `cut` (Rich's `Segment.split_cells`).
    /// A double-width character cut in half becomes a space on each side,
    /// so the two halves keep the total width. Both halves keep the style
    /// and link.
    ///
    /// ```
    /// use fast_rich::text::Span;
    /// let (a, b) = Span::raw("ab日c").split_cells(3);
    /// assert_eq!((a.text.as_ref(), b.text.as_ref()), ("ab ", " c"));
    /// ```
    pub fn split_cells(&self, cut: usize) -> (Span, Span) {
        let cell_length = self.cell_length();
        if cut >= cell_length {
            return (self.clone(), with_text(self, String::new()));
        }
        let chars: Vec<char> = self.text.chars().collect();
        let collect = |range: &[char]| range.iter().collect::<String>();
        // Same search as Rich's `Segment._split_cells`: start from the
        // proportional position and step one character at a time.
        let mut pos = ((cut as f64 / cell_length as f64) * chars.len() as f64) as usize;
        loop {
            let pos_c = pos.min(chars.len());
            let cell_pos = str_cells(&collect(&chars[..pos_c]));
            if cell_pos == cut {
                return (
                    with_text(self, collect(&chars[..pos_c])),
                    with_text(self, collect(&chars[pos_c..])),
                );
            }
            if cell_pos + 1 == cut && chars.get(pos_c).is_some_and(|&c| char_cells(c) == 2) {
                return (
                    with_text(self, collect(&chars[..pos_c]) + " "),
                    with_text(self, " ".to_string() + &collect(&chars[pos_c + 1..])),
                );
            }
            if cell_pos == cut + 1 && pos_c > 0 && char_cells(chars[pos_c - 1]) == 2 {
                return (
                    with_text(self, collect(&chars[..pos_c - 1]) + " "),
                    with_text(self, " ".to_string() + &collect(&chars[pos_c..])),
                );
            }
            if cell_pos < cut {
                pos = pos_c + 1;
            } else {
                pos = pos_c.saturating_sub(1);
            }
        }
    }
}

impl Segment {
    /// Plain text of a line of spans.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// assert_eq!(Segment::plain(&[Span::raw("a"), Span::raw("b")]), "ab");
    /// ```
    pub fn plain(spans: &[Span]) -> String {
        spans.iter().map(|s| s.text.as_ref()).collect()
    }

    /// Restyle spans (Rich's `Segment.apply_style`): each span's style
    /// becomes `style + span style + post_style`. An empty style changes
    /// nothing; control spans get an empty style when either is set.
    /// Links are kept.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let out = Segment::apply_style(&[Span::raw("x")], Style::new().bold(), Style::new());
    /// assert!(out[0].style.bold);
    /// ```
    pub fn apply_style(spans: &[Span], style: Style, post_style: Style) -> Vec<Span> {
        let mut out = spans.to_vec();
        if !style.is_empty() {
            for span in &mut out {
                span.style = if span.is_control() {
                    Style::new()
                } else {
                    style.combine(&span.style)
                };
            }
        }
        if !post_style.is_empty() {
            for span in &mut out {
                span.style = if span.is_control() {
                    Style::new()
                } else {
                    span.style.combine(&post_style)
                };
            }
        }
        out
    }

    /// Keep only control spans (`is_control = true`) or only the others
    /// (Rich's `Segment.filter_control`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// let spans = [Span::raw("a"), Span::raw("\x07")];
    /// assert_eq!(Segment::filter_control(&spans, false), [Span::raw("a")]);
    /// ```
    pub fn filter_control(spans: &[Span], is_control: bool) -> Vec<Span> {
        spans
            .iter()
            .filter(|s| s.is_control() == is_control)
            .cloned()
            .collect()
    }

    /// Split spans into lines at `\n` (Rich's `Segment.split_lines`). The
    /// `\n` characters are dropped; a trailing `\n` does not start an
    /// empty last line.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// let lines = Segment::split_lines(&[Span::raw("a\nb\n")]);
    /// assert_eq!(lines, [vec![Span::raw("a")], vec![Span::raw("b")]]);
    /// ```
    pub fn split_lines(spans: &[Span]) -> Vec<Vec<Span>> {
        Self::split_lines_terminator(spans)
            .into_iter()
            .map(|(line, _)| line)
            .collect()
    }

    /// [`split_lines`](Self::split_lines), also telling whether each line
    /// ended with a `\n` (Rich's `Segment.split_lines_terminator`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// let lines = Segment::split_lines_terminator(&[Span::raw("a\nb")]);
    /// assert_eq!(lines[0].1, true);
    /// assert_eq!(lines[1].1, false);
    /// ```
    pub fn split_lines_terminator(spans: &[Span]) -> Vec<(Vec<Span>, bool)> {
        let mut lines = Vec::new();
        let mut line: Vec<Span> = Vec::new();
        for span in spans {
            if span.text.contains('\n') && !span.is_control() {
                let mut parts = span.text.split('\n').peekable();
                while let Some(part) = parts.next() {
                    if !part.is_empty() {
                        line.push(with_text(span, part.to_string()));
                    }
                    if parts.peek().is_some() {
                        lines.push((std::mem::take(&mut line), true));
                    }
                }
            } else {
                line.push(span.clone());
            }
        }
        if !line.is_empty() {
            lines.push((line, false));
        }
        lines
    }

    /// Split spans into lines and make each exactly `length` cells:
    /// longer lines are cropped, shorter ones padded with spaces in
    /// `style` when `pad` is set (Rich's `Segment.split_and_crop_lines`).
    /// With `include_new_lines`, lines that ended in `\n` keep a
    /// [`Span::line`] at the end.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let lines = Segment::split_and_crop_lines(&[Span::raw("abc\nd")], 2, Style::new(), true, true);
    /// assert_eq!(Segment::plain(&lines[0]), "ab\n");
    /// assert_eq!(Segment::plain(&lines[1]), "d ");
    /// ```
    pub fn split_and_crop_lines(
        spans: &[Span],
        length: usize,
        style: Style,
        pad: bool,
        include_new_lines: bool,
    ) -> Vec<Vec<Span>> {
        Self::split_lines_terminator(spans)
            .into_iter()
            .map(|(line, new_line)| {
                let mut cropped = Self::adjust_line_length(&line, length, style, pad);
                if new_line && include_new_lines {
                    cropped.push(Span::line());
                }
                cropped
            })
            .collect()
    }

    /// Crop or pad one line to `length` cells (Rich's
    /// `Segment.adjust_line_length`). Padding is a span of spaces in
    /// `style`, added only when `pad` is set; a double-width character cut
    /// at the edge becomes a space. Control spans are never cropped.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let line = Segment::adjust_line_length(&[Span::raw("日本語")], 5, Style::new(), true);
    /// assert_eq!(Segment::plain(&line), "日本 ");
    /// ```
    pub fn adjust_line_length(line: &[Span], length: usize, style: Style, pad: bool) -> Vec<Span> {
        let line_length: usize = line.iter().map(Span::cell_length).sum();
        if line_length < length {
            let mut new_line = line.to_vec();
            if pad {
                new_line.push(Span::styled(" ".repeat(length - line_length), style));
            }
            return new_line;
        }
        if line_length == length {
            return line.to_vec();
        }
        let mut new_line = Vec::new();
        let mut used = 0;
        for span in line {
            let span_length = span.cell_length();
            if used + span_length < length || span.is_control() {
                new_line.push(span.clone());
                used += span_length;
            } else {
                new_line.push(with_text(span, crop_cells(&span.text, length - used)));
                break;
            }
        }
        new_line
    }

    /// Cells taken by a line, not counting control spans (Rich's
    /// `Segment.get_line_length`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// assert_eq!(Segment::get_line_length(&[Span::raw("ab"), Span::raw("日")]), 4);
    /// ```
    pub fn get_line_length(line: &[Span]) -> usize {
        line.iter().map(Span::cell_length).sum()
    }

    /// `(width, height)` of the rectangle enclosing the lines (Rich's
    /// `Segment.get_shape`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// let lines = vec![vec![Span::raw("abc")], vec![Span::raw("d")]];
    /// assert_eq!(Segment::get_shape(&lines), (3, 2));
    /// ```
    pub fn get_shape(lines: &[Vec<Span>]) -> (usize, usize) {
        let width = lines
            .iter()
            .map(|l| Self::get_line_length(l))
            .max()
            .unwrap_or(0);
        (width, lines.len())
    }

    /// Make every line `width` cells and add blank lines up to `height`
    /// (Rich's `Segment.set_shape`). `None` (or 0) keeps the number of
    /// lines. As in Rich, lines beyond `height` are kept, not removed.
    /// Blank lines are one span of spaces in `style`, ending in `\n` when
    /// `new_lines` is set.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let lines = Segment::set_shape(&[vec![Span::raw("ab")]], 3, Some(2), Style::new(), false);
    /// assert_eq!(Segment::get_shape(&lines), (3, 2));
    /// ```
    pub fn set_shape(
        lines: &[Vec<Span>],
        width: usize,
        height: Option<usize>,
        style: Style,
        new_lines: bool,
    ) -> Vec<Vec<Span>> {
        let height = height.filter(|&h| h > 0).unwrap_or(lines.len());
        let mut shaped: Vec<Vec<Span>> = lines
            .iter()
            .map(|line| Self::adjust_line_length(line, width, style, true))
            .collect();
        if shaped.len() < height {
            let blank = vec![Self::blank(width, style, new_lines)];
            shaped.resize(height, blank);
        }
        shaped
    }

    fn blank(width: usize, style: Style, new_lines: bool) -> Span {
        let mut text = " ".repeat(width);
        if new_lines {
            text.push('\n');
        }
        Span::styled(text, style)
    }

    /// Shared body of the `align_*` functions: keep at most `height`
    /// lines and add `top` / bottom blank lines.
    fn align(
        lines: &[Vec<Span>],
        width: usize,
        height: usize,
        style: Style,
        new_lines: bool,
        top: impl Fn(usize) -> usize,
    ) -> Vec<Vec<Span>> {
        let kept = &lines[..lines.len().min(height)];
        let extra = height.saturating_sub(lines.len());
        if extra == 0 {
            return kept.to_vec();
        }
        let blank = vec![Self::blank(width, style, new_lines)];
        let above = top(extra);
        let mut out = vec![blank.clone(); above];
        out.extend_from_slice(kept);
        out.extend(std::iter::repeat_n(blank, extra - above));
        out
    }

    /// Fit lines to `height` keeping them at the top: blank lines are
    /// added below, extra lines removed (Rich's `Segment.align_top`).
    /// Line widths are not changed; blanks are `width` spaces in `style`.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let lines = Segment::align_top(&[vec![Span::raw("x")]], 1, 3, Style::new(), false);
    /// assert_eq!(lines.len(), 3);
    /// assert_eq!(Segment::plain(&lines[0]), "x");
    /// ```
    pub fn align_top(
        lines: &[Vec<Span>],
        width: usize,
        height: usize,
        style: Style,
        new_lines: bool,
    ) -> Vec<Vec<Span>> {
        Self::align(lines, width, height, style, new_lines, |_| 0)
    }

    /// Like [`align_top`](Self::align_top) but the blank lines go above
    /// (Rich's `Segment.align_bottom`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let lines = Segment::align_bottom(&[vec![Span::raw("x")]], 1, 3, Style::new(), false);
    /// assert_eq!(Segment::plain(&lines[2]), "x");
    /// ```
    pub fn align_bottom(
        lines: &[Vec<Span>],
        width: usize,
        height: usize,
        style: Style,
        new_lines: bool,
    ) -> Vec<Vec<Span>> {
        Self::align(lines, width, height, style, new_lines, |extra| extra)
    }

    /// Like [`align_top`](Self::align_top) but the blank lines are split
    /// above and below, the extra one going below (Rich's
    /// `Segment.align_middle`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let lines = Segment::align_middle(&[vec![Span::raw("x")]], 1, 4, Style::new(), false);
    /// assert_eq!(Segment::plain(&lines[1]), "x");
    /// ```
    pub fn align_middle(
        lines: &[Vec<Span>],
        width: usize,
        height: usize,
        style: Style,
        new_lines: bool,
    ) -> Vec<Vec<Span>> {
        Self::align(lines, width, height, style, new_lines, |extra| extra / 2)
    }

    /// Merge neighbouring spans with the same style and link (Rich's
    /// `Segment.simplify`). A control span is never merged into the span
    /// before it.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// let out = Segment::simplify(&[Span::raw("a"), Span::raw("b")]);
    /// assert_eq!(out, [Span::raw("ab")]);
    /// ```
    pub fn simplify(spans: &[Span]) -> Vec<Span> {
        let mut out: Vec<Span> = Vec::with_capacity(spans.len());
        for span in spans {
            match out.last_mut() {
                Some(last)
                    if last.style == span.style && last.link == span.link && !span.is_control() =>
                {
                    last.text.to_mut().push_str(&span.text);
                }
                _ => out.push(span.clone()),
            }
        }
        out
    }

    /// Remove links, keeping styles (Rich's `Segment.strip_links`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let span = Span::linked("x", Style::new(), "https://a".into());
    /// assert_eq!(Segment::strip_links(&[span])[0].link, None);
    /// ```
    pub fn strip_links(spans: &[Span]) -> Vec<Span> {
        spans
            .iter()
            .map(|s| Span {
                link: if s.is_control() { s.link.clone() } else { None },
                ..s.clone()
            })
            .collect()
    }

    /// Remove styles and links (Rich's `Segment.strip_styles`; Rich keeps
    /// links in the style).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let out = Segment::strip_styles(&[Span::styled("x", Style::new().bold())]);
    /// assert_eq!(out, [Span::raw("x")]);
    /// ```
    pub fn strip_styles(spans: &[Span]) -> Vec<Span> {
        spans.iter().map(|s| Span::raw(s.text.clone())).collect()
    }

    /// Remove foreground and background colors, keeping attributes and
    /// links (Rich's `Segment.remove_color`).
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::style::Style;
    /// use fast_rich::text::Span;
    /// let out = Segment::remove_color(&[Span::styled("x", Style::parse("bold red on blue"))]);
    /// assert_eq!(out[0].style, Style::new().bold());
    /// ```
    pub fn remove_color(spans: &[Span]) -> Vec<Span> {
        spans
            .iter()
            .map(|s| {
                let mut s = s.clone();
                s.style.foreground = None;
                s.style.background = None;
                s
            })
            .collect()
    }

    /// Cut spans at the given cell positions (Rich's `Segment.divide`).
    /// Cuts should be ascending; output stops at the last cut, so pass the
    /// total width as the last cut to keep everything. A double-width
    /// character cut in half becomes two spaces. Leading zero cuts give
    /// empty pieces.
    ///
    /// ```
    /// use fast_rich::renderable::Segment;
    /// use fast_rich::text::Span;
    /// let parts = Segment::divide(&[Span::raw("abcdef")], [2, 6]);
    /// let plain: Vec<String> = parts.iter().map(|p| Segment::plain(p)).collect();
    /// assert_eq!(plain, ["ab", "cdef"]);
    /// ```
    pub fn divide(spans: &[Span], cuts: impl IntoIterator<Item = usize>) -> Vec<Vec<Span>> {
        let mut out = Vec::new();
        let mut cuts = cuts.into_iter();
        let mut cut = loop {
            match cuts.next() {
                None => return out,
                Some(0) => out.push(Vec::new()),
                Some(cut) => break cut,
            }
        };
        let mut split: Vec<Span> = Vec::new();
        let mut pos = 0;
        for span in spans {
            let mut span = span.clone();
            while !span.text.is_empty() {
                let end_pos = if span.is_control() {
                    pos
                } else {
                    pos + str_cells(&span.text)
                };
                if end_pos < cut {
                    split.push(span);
                    pos = end_pos;
                    break;
                }
                if end_pos == cut {
                    split.push(span);
                    out.push(std::mem::take(&mut split));
                    pos = end_pos;
                    match cuts.next() {
                        Some(next) => cut = next,
                        None => return out,
                    }
                    break;
                }
                let (before, after) = span.split_cells(cut - pos);
                split.push(before);
                out.push(std::mem::take(&mut split));
                span = after;
                pos = cut;
                match cuts.next() {
                    Some(next) => cut = next,
                    None => return out,
                }
            }
        }
        out.push(split);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_cells_wide() {
        let (a, b) = Span::raw("日本").split_cells(1);
        assert_eq!((a.text.as_ref(), b.text.as_ref()), (" ", " 本"));
        let (a, b) = Span::raw("abc").split_cells(5);
        assert_eq!((a.text.as_ref(), b.text.as_ref()), ("abc", ""));
    }

    #[test]
    fn divide_stops_at_last_cut() {
        let parts = Segment::divide(&[Span::raw("abcdef")], [0, 2, 4]);
        let plain: Vec<String> = parts.iter().map(|p| Segment::plain(p)).collect();
        assert_eq!(plain, ["", "ab", "cd"]);
    }

    #[test]
    fn control_spans_take_no_cells() {
        let line = [Span::raw("ab"), Span::raw("\x1b[2J"), Span::raw("cd")];
        assert_eq!(Segment::get_line_length(&line), 4);
        let cropped = Segment::adjust_line_length(&line, 3, Style::new(), true);
        assert_eq!(Segment::plain(&cropped), "ab\x1b[2Jc");
    }
}
