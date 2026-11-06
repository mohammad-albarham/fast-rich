//! Markup parser for Rich-style markup syntax.
//!
//! Parses text like `[bold red]Hello[/]` into styled spans.
//!
//! # Syntax
//!
//! - `[style]text[/]` - Apply style to text
//! - `[style]text[/style]` - Explicit close tag
//! - `[bold red on blue]` - Multiple styles
//! - `:emoji_name:` - Emoji shortcode (only known shortcodes are replaced)
//! - `\[` - Escaped `[` (see [`escape`]); like Rich, only `[` followed by
//!   `a-z`, `#`, `/` or `@` starts a tag, so `[1, 2]` and `[INFO]` print as-is

use crate::style::{Color, Style};
use crate::text::{Span, Text};

/// A parsed token from markup text.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkupToken {
    /// Plain text
    Text(String),
    /// Opening style tag with optional hyperlink
    OpenTag(Style, Option<String>),
    /// Closing tag
    CloseTag,
    /// Emoji shortcode like :smile:
    Emoji(String),
}

/// Whether a markup word is not a known attribute/color and may be a
/// theme-custom color name.
fn style_is_unknown_word(word: &str) -> bool {
    !matches!(
        word,
        "bold"
            | "b"
            | "dim"
            | "italic"
            | "i"
            | "underline"
            | "u"
            | "blink"
            | "reverse"
            | "hidden"
            | "strike"
            | "strikethrough"
            | "s"
            | "on"
            | "not"
    ) && Color::parse(word).is_none()
        && crate::style::Style::parse(word) == crate::style::Style::new()
}

/// Parse markup text into tokens.
pub fn tokenize(input: &str) -> Vec<MarkupToken> {
    tokenize_with(input, true)
}

/// Parse markup text into tokens, optionally translating emoji shortcodes.
pub fn tokenize_with(input: &str, translate_emoji: bool) -> Vec<MarkupToken> {
    tokenize_with_theme(input, translate_emoji, &crate::theme::Theme::new())
}

/// Parse markup text into tokens with theme-aware style resolution.
pub fn tokenize_with_theme(
    input: &str,
    translate_emoji: bool,
    theme: &crate::theme::Theme,
) -> Vec<MarkupToken> {
    tokenize_impl(input, translate_emoji, theme)
}

/// Length in bytes of the markup tag starting at `input[at..]`, if any.
///
/// Follows Rich's tag grammar: `[` + a first character in `a-z#/@` + any
/// characters except `[` up to the first `]`. So `[bold]`, `[/]` and
/// `[#ff0000]` are tags, while `[1, 2]`, `[]` and `[INFO]` are plain text.
fn tag_len_at(input: &str, at: usize) -> Option<usize> {
    let rest = input.get(at..)?;
    let mut chars = rest.char_indices();
    match chars.next() {
        Some((_, '[')) => {}
        _ => return None,
    }
    match chars.next() {
        Some((_, c)) if c.is_ascii_lowercase() || matches!(c, '#' | '/' | '@') => {}
        _ => return None,
    }
    for (i, c) in chars {
        match c {
            ']' => return Some(i + 1),
            '[' => return None,
            _ => {}
        }
    }
    None
}

/// Convert the inside of a tag (between the brackets) into a token.
fn tag_token(tag_content: &str, theme: &crate::theme::Theme) -> MarkupToken {
    if tag_content.starts_with('/') {
        return MarkupToken::CloseTag;
    }

    let mut link: Option<String> = None;
    let mut style_parts = Vec::new();
    for part in tag_content.split_whitespace() {
        if let Some(url) = part.strip_prefix("link=") {
            link = Some(url.to_string());
        } else {
            style_parts.push(part);
        }
    }

    // A theme style name (e.g. `[warning]`) is looked up as a whole first.
    if link.is_none() {
        if let Some(style) = theme.lookup_style(tag_content.trim()) {
            return MarkupToken::OpenTag(style, None);
        }
    }

    let mut style = Style::parse(&style_parts.join(" "));
    // Resolve words that aren't built-in styles/colors against the theme:
    // named styles first, then custom colors (e.g. a color named "brand"
    // enables `[brand]text[/]`).
    for part in &style_parts {
        let lower = part.to_lowercase();
        if style_is_unknown_word(&lower) {
            if let Some(named) = theme.lookup_style(&lower) {
                style = style.combine(&named);
            } else if let Some(color) = theme.custom.get(&lower) {
                style = style.combine(&Style::new().foreground(*color));
            }
        }
    }
    // Like Rich, a tag with an unknown style is consumed (it renders
    // unstyled) rather than printed.
    MarkupToken::OpenTag(style, link)
}

fn tokenize_impl(
    input: &str,
    translate_emoji: bool,
    theme: &crate::theme::Theme,
) -> Vec<MarkupToken> {
    let mut tokens = Vec::new();
    let mut current_text = String::new();
    let mut i = 0;

    while let Some(c) = input[i..].chars().next() {
        match c {
            '\\' => {
                // A run of backslashes before a tag: each `\\` pair is one
                // literal backslash; an odd one escapes the tag (Rich's
                // `\[bold]`). Backslashes elsewhere are kept verbatim.
                let run_start = i;
                while input[i..].starts_with('\\') {
                    i += 1;
                }
                let run = i - run_start;
                if let Some(len) = tag_len_at(input, i) {
                    current_text.push_str(&"\\".repeat(run / 2));
                    if run % 2 == 1 {
                        current_text.push_str(&input[i..i + len]);
                        i += len;
                    }
                } else {
                    current_text.push_str(&input[run_start..i]);
                }
            }
            '[' => {
                if let Some(len) = tag_len_at(input, i) {
                    if !current_text.is_empty() {
                        tokens.push(MarkupToken::Text(std::mem::take(&mut current_text)));
                    }
                    tokens.push(tag_token(&input[i + 1..i + len - 1], theme));
                    i += len;
                } else {
                    current_text.push('[');
                    i += 1;
                }
            }
            ':' => {
                // Scan a candidate emoji shortcode: :name:
                i += 1;
                let name_len = input[i..]
                    .find(|ch: char| !(ch.is_alphanumeric() || matches!(ch, '_' | '-' | '+')))
                    .unwrap_or(input.len() - i);
                let emoji_name = &input[i..i + name_len];
                let found_close = input[i + name_len..].starts_with(':');

                // Only treat it as an emoji when enabled and the shortcode is
                // known; otherwise keep the text verbatim (so "12:30:45"
                // survives).
                if translate_emoji
                    && found_close
                    && !emoji_name.is_empty()
                    && crate::emoji::get_emoji(emoji_name).is_some()
                {
                    if !current_text.is_empty() {
                        tokens.push(MarkupToken::Text(std::mem::take(&mut current_text)));
                    }
                    tokens.push(MarkupToken::Emoji(emoji_name.to_string()));
                    i += name_len + 1;
                } else {
                    // Keep ':' and the name; a closing ':' is re-scanned as
                    // the possible start of the next shortcode.
                    current_text.push(':');
                    current_text.push_str(emoji_name);
                    i += name_len;
                }
            }
            _ => {
                current_text.push(c);
                i += c.len_utf8();
            }
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        tokens.push(MarkupToken::Text(current_text));
    }

    tokens
}

/// Escape text so it prints literally when parsed as markup.
///
/// Mirrors Rich's `rich.markup.escape`: every `[` that would start a tag
/// gets a backslash (and backslashes right before it are doubled).
///
/// ```
/// use fast_rich::markup::{escape, render_plain};
/// assert_eq!(escape("[bold]x[/bold]"), "\\[bold]x\\[/bold]");
/// assert_eq!(render_plain(&escape("[bold]x[/bold]")), "[bold]x[/bold]");
/// ```
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while let Some(c) = text[i..].chars().next() {
        if c == '\\' || c == '[' {
            let run_start = i;
            while text[i..].starts_with('\\') {
                i += 1;
            }
            if tag_len_at(text, i).is_some() {
                // Double the backslashes, then escape the bracket.
                out.push_str(&text[run_start..i]);
                out.push_str(&text[run_start..i]);
                out.push('\\');
                out.push('[');
                i += 1;
            } else if i > run_start {
                out.push_str(&text[run_start..i]);
            } else {
                out.push('[');
                i += 1;
            }
        } else {
            out.push(c);
            i += c.len_utf8();
        }
    }
    // A trailing single backslash would escape whatever follows; double it.
    if out.ends_with('\\') && !out.ends_with("\\\\") {
        out.push('\\');
    }
    out
}

/// Parse markup text into styled Text.
pub fn parse(input: &str) -> Text {
    parse_with(input, true)
}

/// Parse markup text into styled Text, optionally translating emoji
/// shortcodes (`:name:`).
pub fn parse_with(input: &str, translate_emoji: bool) -> Text {
    parse_with_theme(input, translate_emoji, &crate::theme::Theme::new())
}

/// Parse markup text into styled Text, resolving custom theme colors.
///
/// Style words that aren't built-in colors are looked up in
/// [`Theme::custom`](crate::theme::Theme::custom), so a theme color named
/// `"brand"` enables `[brand]text[/]`.
pub fn parse_with_theme(input: &str, translate_emoji: bool, theme: &crate::theme::Theme) -> Text {
    let tokens = tokenize_with_theme(input, translate_emoji, theme);
    let mut spans = Vec::new();
    let mut style_stack: Vec<Style> = Vec::new();
    let mut link_stack: Vec<Option<String>> = Vec::new();

    for token in tokens {
        match token {
            MarkupToken::Text(text) => {
                let style = style_stack.last().cloned().unwrap_or_default();
                let link = link_stack.iter().rev().find_map(|l| l.clone());
                if let Some(url) = link {
                    spans.push(Span::linked(text, style, url));
                } else {
                    spans.push(Span::styled(text, style));
                }
            }
            MarkupToken::OpenTag(style, link) => {
                let combined = if let Some(current) = style_stack.last() {
                    current.combine(&style)
                } else {
                    style
                };
                style_stack.push(combined);
                link_stack.push(link);
            }
            MarkupToken::CloseTag => {
                style_stack.pop();
                link_stack.pop();
            }
            MarkupToken::Emoji(name) => {
                let emoji = crate::emoji::get_emoji(&name).unwrap_or(&name);
                let style = style_stack.last().cloned().unwrap_or_default();
                spans.push(Span::styled(emoji.to_string(), style));
            }
        }
    }

    Text::from_spans(spans)
}

/// Render markup to a plain string (for testing/debugging).
pub fn render_plain(input: &str) -> String {
    parse(input).plain_text()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_tokenize_plain() {
        let tokens = tokenize("Hello, World!");
        assert_eq!(tokens, vec![MarkupToken::Text("Hello, World!".to_string())]);
    }

    #[test]
    fn test_tokenize_styled() {
        let tokens = tokenize("[bold]Hello[/]");
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], MarkupToken::OpenTag(_, _)));
        assert_eq!(tokens[1], MarkupToken::Text("Hello".to_string()));
        assert_eq!(tokens[2], MarkupToken::CloseTag);
    }

    #[test]
    fn test_tokenize_nested() {
        let tokens = tokenize("[bold][red]Hi[/][/]");
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn test_backslash_escapes_tag() {
        // Rich: `\[bold]` prints the tag literally.
        assert_eq!(render_plain("\\[bold]x[/bold]"), "[bold]x");
        // `\\[bold]` is a literal backslash followed by a real tag.
        let text = parse("\\\\[bold]x[/]");
        assert_eq!(text.plain_text(), "\\x");
        assert!(text.spans.iter().any(|s| s.style.bold));
        // Backslashes not before a tag are kept.
        assert_eq!(render_plain("C:\\dir\\file"), "C:\\dir\\file");
    }

    #[test]
    fn test_only_rich_tags_are_parsed() {
        // A tag must start with a-z, '#', '/' or '@' (Rich's grammar).
        assert_eq!(render_plain("[1, 2, 3]"), "[1, 2, 3]");
        assert_eq!(render_plain("[INFO] ok"), "[INFO] ok");
        assert_eq!(render_plain("list[]"), "list[]");
        assert_eq!(render_plain("[[1, 2], [3]]"), "[[1, 2], [3]]");
        // Unknown lowercase tags are consumed, like Rich.
        assert_eq!(render_plain("[nostyle]x[/nostyle]"), "x");
        assert!(parse("[#ff0000]red").spans[0].style.foreground.is_some());
    }

    #[test]
    fn test_escape_round_trips() {
        for s in ["[bold]literal[/bold]", "a\\[b]", "[1] [x]", "plain"] {
            assert_eq!(render_plain(&escape(s)), s, "escape({s:?})");
        }
        assert_eq!(escape("[bold]x"), "\\[bold]x");
        assert_eq!(escape("[1]"), "[1]");
        // Like Rich, a trailing backslash is doubled so concatenating
        // markup after it can't be escaped by accident.
        assert_eq!(escape("x\\"), "x\\\\");
    }

    #[test]
    fn test_theme_style_names() {
        let mut theme = crate::theme::Theme::new();
        theme.add_style("alert", Style::parse("bold red"));
        let text = parse_with_theme("[alert]x[/] [italic alert]y", true, &theme);
        assert!(text.spans[0].style.bold);
        let y = text.spans.iter().find(|s| s.text == "y").unwrap();
        assert!(y.style.bold && y.style.italic);
    }

    #[test]
    fn test_tokenize_emoji() {
        let tokens = tokenize(":smile:");
        assert_eq!(tokens, vec![MarkupToken::Emoji("smile".to_string())]);
    }

    #[test]
    fn test_unknown_shortcode_keeps_colons() {
        // Unknown shortcodes must stay verbatim — no dropped colons.
        assert_eq!(render_plain("12:30:45"), "12:30:45");
        assert_eq!(render_plain("ratio a:b:c"), "ratio a:b:c");
        assert_eq!(render_plain("see http://x:8080/"), "see http://x:8080/");
    }

    #[test]
    fn test_known_shortcode_replaced() {
        assert_ne!(render_plain("hi :smile:"), "hi :smile:");
    }

    #[test]
    fn test_plus_one_shortcode() {
        let tokens = tokenize(":+1:");
        assert_eq!(tokens, vec![MarkupToken::Emoji("+1".to_string())]);
    }

    #[test]
    fn test_tag_close_not_broken_by_double_bracket() {
        // "[bold]]" should be a bold tag followed by a literal ']'
        let text = parse("[bold]]x[/]");
        assert!(text.spans[0].style.bold);
        assert_eq!(text.plain_text(), "]x");
    }

    #[test]
    fn test_parse_plain() {
        let text = parse("Hello, World!");
        assert_eq!(text.plain_text(), "Hello, World!");
    }

    #[test]
    fn test_parse_styled() {
        let text = parse("[bold]Hello[/]");
        assert_eq!(text.plain_text(), "Hello");
        assert_eq!(text.spans.len(), 1);
        assert!(text.spans[0].style.bold);
    }

    #[test]
    fn test_parse_multiple_styles() {
        let text = parse("[bold red]Hello[/]");
        assert!(text.spans[0].style.bold);
        assert_eq!(text.spans[0].style.foreground, Some(Color::Red));
    }

    #[test]
    fn test_parse_nested() {
        let text = parse("[bold]Hello [italic]World[/][/]");
        assert_eq!(text.plain_text(), "Hello World");
        assert!(text.spans[0].style.bold);
        assert!(text.spans[1].style.bold);
        assert!(text.spans[1].style.italic);
    }

    #[test]
    fn test_parse_background() {
        let text = parse("[white on red]Alert[/]");
        assert_eq!(text.spans[0].style.foreground, Some(Color::White));
        assert_eq!(text.spans[0].style.background, Some(Color::Red));
    }

    #[test]
    fn test_parse_hyperlink() {
        let text = parse("[link=https://example.com]Click here[/]");
        assert_eq!(text.plain_text(), "Click here");
        assert_eq!(text.spans[0].link, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_parse_hyperlink_with_style() {
        let text = parse("[bold blue link=https://google.com]Google[/]");
        assert_eq!(text.plain_text(), "Google");
        assert!(text.spans[0].style.bold);
        assert_eq!(text.spans[0].style.foreground, Some(Color::Blue));
        assert_eq!(text.spans[0].link, Some("https://google.com".to_string()));
    }
}
