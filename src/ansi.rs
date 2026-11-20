//! Decode text containing ANSI escape sequences into styled [`Text`]
//! (Rich's `rich.ansi`: `AnsiDecoder` and `Text.from_ansi`).
//!
//! Use it to embed the colored output of another program (`ls --color`,
//! `cargo`, `git diff --color`) in a panel, table or layout: the escapes
//! become styles and links, so widths are measured on the visible text
//! only.
//!
//! What is decoded, as in Rich 15:
//!
//! * SGR (`ESC [ ... m`): all 13 attributes and their "off" codes (`22`
//!   clears bold and dim, `54` clears frame and encircle), the 16 standard
//!   colors (`30`-`37`, `90`-`97` and backgrounds), 256 colors
//!   (`38;5;n`), truecolor (`38;2;r;g;b`), the default colors (`39`,
//!   `49`) and reset (`0`, or an empty `ESC [ m`). Unknown codes are
//!   ignored; numbers above 255 are clamped to 255.
//! * OSC 8 hyperlinks (`ESC ] 8 ; params ; url ST`) set
//!   [`Span::link`](crate::text::Span::link); an empty url ends the link.
//!   SGR `0` ends a link too (Rich keeps the link in the style).
//! * Every other escape sequence (cursor movement, `ESC [ ? 25 l`,
//!   character-set selection `ESC ( B`, other OSC strings) is removed.
//! * Each line keeps only the text after its last `\r` (what a terminal
//!   would show after a carriage return), so a `\r\n` line ending leaves an
//!   empty line, exactly as in Rich. The control characters bell,
//!   backspace, vertical tab and form feed are removed.
//!
//! One deliberate difference: Rich only recognizes OSC strings ended by
//! `ESC \`; an OSC 8 link ended by BEL (`\x07`, which most programs and
//! fast-rich's own console write) comes out of Rich as stray text such as
//! `8;;https://...`. fast-rich accepts both terminators, so its own output
//! decodes back to the same links.
//!
//! ```
//! use fast_rich::style::Color;
//! use fast_rich::text::Text;
//!
//! let text = Text::from_ansi("\x1b[1;31merror\x1b[0m: oops");
//! assert_eq!(text.plain_text(), "error: oops");
//! assert!(text.spans[0].style.bold);
//! assert_eq!(text.spans[0].style.foreground, Some(Color::Red));
//! ```

use crate::style::{Color, Style};
use crate::text::{Span, Text};

/// A decoded piece of the input: plain text, an SGR parameter string or an
/// OSC payload (Rich's `_AnsiToken`).
#[derive(Debug, PartialEq)]
enum Token<'a> {
    Plain(&'a str),
    Sgr(&'a str),
    Osc(&'a str),
}

/// Split `text` into plain text and escape sequences, matching Rich's
/// `re_ansi` regex and `_ansi_tokenize` (plus BEL-terminated OSC strings).
fn tokenize(text: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    // `position`: end of the last consumed input; `search`: where the next
    // escape is looked for (they differ after `ESC (`, which also skips
    // the following character, as in Rich).
    let mut position = 0;
    let mut search = 0;
    while let Some(offset) = text[search..].find('\x1b') {
        let start = search + offset;
        let Some((end, token)) = match_escape(text, start) else {
            // Not an escape sequence Rich recognizes: the ESC stays text.
            search = start + 1;
            continue;
        };
        if start > position {
            tokens.push(Token::Plain(&text[position..start]));
        }
        search = end;
        position = end;
        match token {
            Escape::Charset => {
                // Skip the charset designator (one character).
                position = text[end..]
                    .chars()
                    .next()
                    .map_or(end + 1, |c| end + c.len_utf8());
            }
            Escape::Sgr(params) => tokens.push(Token::Sgr(params)),
            Escape::Osc(payload) => tokens.push(Token::Osc(payload)),
            Escape::Other => {}
        }
    }
    if position < text.len() {
        tokens.push(Token::Plain(&text[position..]));
    }
    tokens
}

enum Escape<'a> {
    Charset,
    Sgr(&'a str),
    Osc(&'a str),
    Other,
}

/// Match one escape sequence at `start` (which holds an ESC). Returns the
/// end offset and what was matched, or `None` if no alternative of Rich's
/// regex matches here.
fn match_escape(text: &str, start: usize) -> Option<(usize, Escape<'_>)> {
    let bytes = text.as_bytes();
    let next = *bytes.get(start + 1)?;
    // (?:\x1b[0-?])
    if (b'0'..=b'?').contains(&next) {
        return Some((start + 2, Escape::Other));
    }
    // (?:\x1b\](.*?)\x1b\\)  — `.` does not match `\n`. BEL is accepted as
    // a terminator too (see the module docs).
    if next == b']' {
        let body = start + 2;
        for (i, &b) in bytes[body..].iter().enumerate() {
            match b {
                b'\n' => break,
                0x07 => return Some((body + i + 1, Escape::Osc(&text[body..body + i]))),
                0x1b if bytes.get(body + i + 1) == Some(&b'\\') => {
                    return Some((body + i + 2, Escape::Osc(&text[body..body + i])));
                }
                _ => {}
            }
        }
    }
    // (?:\x1b([(@-Z\\-_]|\[[0-?]*[ -/]*[@-~]))
    match next {
        b'(' => Some((start + 2, Escape::Charset)),
        b'@'..=b'Z' | b'\\'..=b'_' => Some((start + 2, Escape::Other)),
        b'[' => {
            let mut i = start + 2;
            while bytes.get(i).is_some_and(|b| (b'0'..=b'?').contains(b)) {
                i += 1;
            }
            while bytes.get(i).is_some_and(|b| (b' '..=b'/').contains(b)) {
                i += 1;
            }
            let last = *bytes.get(i)?;
            if !(b'@'..=b'~').contains(&last) {
                return None;
            }
            let end = i + 1;
            if last == b'm' {
                Some((end, Escape::Sgr(&text[start + 2..i])))
            } else {
                Some((end, Escape::Other))
            }
        }
        _ => None,
    }
}

/// The 16 standard colors and the 256-color palette, as Rich's
/// `Color.from_ansi` gives them (0-15 are the named colors).
fn ansi_color(number: u8) -> Color {
    if number < 16 {
        Color::from_ansi_standard_code(number)
    } else {
        Color::Ansi256(number)
    }
}

/// Translates ANSI escape sequences into styled [`Text`] (Rich's
/// `AnsiDecoder`).
///
/// The decoder keeps the current style and link between calls, so a style
/// opened on one line carries on to the next, as in a terminal.
///
/// ```
/// use fast_rich::ansi::AnsiDecoder;
///
/// let mut decoder = AnsiDecoder::new();
/// let lines = decoder.decode("\x1b[1mone\ntwo\x1b[0m\nthree");
/// let plain: Vec<String> = lines.iter().map(|t| t.plain_text()).collect();
/// assert_eq!(plain, ["one", "two", "three"]);
/// assert!(lines[1].spans[0].style.bold);
/// assert!(lines[2].spans[0].style.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct AnsiDecoder {
    style: Style,
    link: Option<String>,
}

impl AnsiDecoder {
    /// Create a decoder with no current style.
    ///
    /// ```
    /// use fast_rich::ansi::AnsiDecoder;
    /// let text = AnsiDecoder::new().decode_line("plain");
    /// assert_eq!(text.plain_text(), "plain");
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode terminal output into one [`Text`] per line. A trailing
    /// newline gives a final empty line, as in Rich.
    ///
    /// ```
    /// use fast_rich::ansi::AnsiDecoder;
    /// let lines = AnsiDecoder::new().decode("a\nb\n");
    /// assert_eq!(lines.len(), 3);
    /// ```
    pub fn decode(&mut self, terminal_text: &str) -> Vec<Text> {
        terminal_text
            .split('\n')
            .map(|line| self.decode_line(line))
            .collect()
    }

    /// Decode one line (without its `\n`) into a [`Text`].
    ///
    /// Only the text after the last `\r` is kept, and escapes before it
    /// are dropped with it (Rich does the same).
    ///
    /// ```
    /// use fast_rich::ansi::AnsiDecoder;
    /// let text = AnsiDecoder::new().decode_line("50%\r\x1b[32m100%\x1b[0m");
    /// assert_eq!(text.plain_text(), "100%");
    /// ```
    pub fn decode_line(&mut self, line: &str) -> Text {
        let line = line.rsplit('\r').next().unwrap_or(line);
        let mut spans: Vec<Span> = Vec::new();
        for token in tokenize(line) {
            match token {
                Token::Plain(plain) => {
                    let plain = crate::control::strip_control_codes(plain);
                    if plain.is_empty() {
                        continue;
                    }
                    match spans.last_mut() {
                        Some(last) if last.style == self.style && last.link == self.link => {
                            last.text.to_mut().push_str(&plain);
                        }
                        _ => spans.push(Span {
                            text: plain.into(),
                            style: self.style,
                            link: self.link.clone(),
                        }),
                    }
                }
                Token::Osc(osc) => {
                    if let Some(rest) = osc.strip_prefix("8;") {
                        if let Some((_params, link)) = rest.split_once(';') {
                            self.link = (!link.is_empty()).then(|| link.to_string());
                        }
                    }
                }
                Token::Sgr(sgr) => self.apply_sgr(sgr),
            }
        }
        Text::from_spans(spans)
    }

    /// Apply the parameters of one SGR sequence to the current style.
    fn apply_sgr(&mut self, sgr: &str) {
        // Lenient like Rich: drop anything that is not a number (such as
        // `4:3` sub-parameters); an empty parameter means 0.
        let mut codes = sgr
            .split(';')
            .filter(|code| code.bytes().all(|b| b.is_ascii_digit()))
            .map(|code| {
                if code.is_empty() {
                    0
                } else {
                    code.parse::<u64>().map_or(255, |n| n.min(255) as u8)
                }
            });
        let style = &mut self.style;
        while let Some(code) = codes.next() {
            match code {
                0 => {
                    *style = Style::new();
                    self.link = None;
                }
                1 => style.bold = true,
                2 => style.dim = true,
                3 => style.italic = true,
                4 => style.underline = true,
                5 => style.blink = true,
                6 => style.blink2 = true,
                7 => style.reverse = true,
                8 => style.hidden = true,
                9 => style.strikethrough = true,
                21 => style.underline2 = true,
                22 => {
                    style.dim = false;
                    style.bold = false;
                }
                23 => style.italic = false,
                24 => style.underline = false,
                25 => style.blink = false,
                26 => style.blink2 = false,
                27 => style.reverse = false,
                28 => style.hidden = false,
                29 => style.strikethrough = false,
                30..=37 => style.foreground = Some(ansi_color(code - 30)),
                39 => style.foreground = Some(Color::Default),
                40..=47 => style.background = Some(ansi_color(code - 40)),
                49 => style.background = Some(Color::Default),
                51 => style.frame = true,
                52 => style.encircle = true,
                53 => style.overline = true,
                54 => {
                    style.frame = false;
                    style.encircle = false;
                }
                55 => style.overline = false,
                90..=97 => style.foreground = Some(ansi_color(code - 90 + 8)),
                100..=107 => style.background = Some(ansi_color(code - 100 + 8)),
                38 | 48 => {
                    let color = match codes.next() {
                        Some(5) => codes.next().map(ansi_color),
                        Some(2) => match (codes.next(), codes.next(), codes.next()) {
                            (Some(r), Some(g), Some(b)) => Some(Color::Rgb { r, g, b }),
                            _ => None,
                        },
                        _ => None,
                    };
                    if let Some(color) = color {
                        if code == 38 {
                            style.foreground = Some(color);
                        } else {
                            style.background = Some(color);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl Text {
    /// Create a [`Text`] from a string containing ANSI escape codes (Rich's
    /// `Text.from_ansi`). Lines are decoded with one [`AnsiDecoder`] and
    /// joined with `\n`; see the [`ansi`](crate::ansi) module for exactly
    /// what is decoded.
    ///
    /// ```
    /// use fast_rich::text::Text;
    ///
    /// let text = Text::from_ansi("\x1b]8;;https://example.com\x1b\\site\x1b]8;;\x1b\\");
    /// assert_eq!(text.plain_text(), "site");
    /// assert_eq!(text.spans[0].link.as_deref(), Some("https://example.com"));
    /// ```
    pub fn from_ansi(text: &str) -> Text {
        let mut decoder = AnsiDecoder::new();
        Text::plain("\n").join(decoder.decode(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_like_rich() {
        assert_eq!(
            tokenize("a\x1b[1;31mb\x1b[?25lc"),
            [
                Token::Plain("a"),
                Token::Sgr("1;31"),
                Token::Plain("b"),
                Token::Plain("c")
            ]
        );
        // ESC ( skips the charset character.
        assert_eq!(tokenize("\x1b(Bxy"), [Token::Plain("xy")]);
        // An unfinished CSI is not an escape: it stays text.
        assert_eq!(tokenize("\x1b[1;2"), [Token::Plain("\x1b[1;2")]);
        assert_eq!(
            tokenize("\x1b]0;t\x1b\\x"),
            [Token::Osc("0;t"), Token::Plain("x")]
        );
    }

    #[test]
    fn sgr_off_codes() {
        let t = Text::from_ansi("\x1b[1;2;3mA\x1b[22mB\x1b[23mC");
        assert!(t.spans[0].style.bold && t.spans[0].style.dim);
        assert!(!t.spans[1].style.bold && t.spans[1].style.italic);
        assert!(t.spans[2].style.is_empty());
    }

    #[test]
    fn colors() {
        let t = Text::from_ansi("\x1b[38;5;200;48;2;1;2;3mx\x1b[39;49my\x1b[38;2;1mz");
        assert_eq!(t.spans[0].style.foreground, Some(Color::Ansi256(200)));
        assert_eq!(t.spans[0].style.background, Some(Color::rgb(1, 2, 3)));
        assert_eq!(t.spans[1].style.foreground, Some(Color::Default));
        // Incomplete truecolor is ignored, so `z` keeps `y`'s style.
        assert_eq!(t.spans[1].text, "yz");
    }

    #[test]
    fn links_and_bel() {
        let t = Text::from_ansi("\x1b]8;;http://x\x07a\x1b]8;;\x07b");
        assert_eq!(t.spans[0].link.as_deref(), Some("http://x"));
        assert_eq!(t.spans[1].link, None);
        assert_eq!(t.plain_text(), "ab");
    }

    #[test]
    fn carriage_returns() {
        assert_eq!(Text::from_ansi("a\r\nb").plain_text(), "\nb");
        assert_eq!(Text::from_ansi("x\x07y\x08z").plain_text(), "xyz");
    }
}
