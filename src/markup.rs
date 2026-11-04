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
//! - `[[` - Escaped `[`
//! - `]]` - Escaped `]`

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

fn tokenize_impl(
    input: &str,
    translate_emoji: bool,
    theme: &crate::theme::Theme,
) -> Vec<MarkupToken> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_text = String::new();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                // Check for escape sequence [[
                if chars.peek() == Some(&'[') {
                    chars.next();
                    current_text.push('[');
                    continue;
                }

                // Flush current text
                if !current_text.is_empty() {
                    tokens.push(MarkupToken::Text(std::mem::take(&mut current_text)));
                }

                // Parse tag content
                let mut tag_content = String::new();
                let mut found_close = false;

                while let Some(&c) = chars.peek() {
                    if c == ']' {
                        // First ']' closes the tag; `]]` escapes are handled
                        // outside tags.
                        chars.next();
                        found_close = true;
                        break;
                    } else {
                        tag_content.push(chars.next().unwrap());
                    }
                }

                if !found_close {
                    // Unterminated tag, treat as text
                    current_text.push('[');
                    current_text.push_str(&tag_content);
                    continue;
                }

                // Parse the tag content
                let tag_content = tag_content.trim();

                if tag_content.is_empty() || tag_content == "/" {
                    // Close tag
                    tokens.push(MarkupToken::CloseTag);
                } else if tag_content.starts_with('/') {
                    // Explicit close tag like [/bold]
                    tokens.push(MarkupToken::CloseTag);
                } else {
                    // Check for link=URL attribute
                    let mut link: Option<String> = None;
                    let mut style_parts = Vec::new();

                    for part in tag_content.split_whitespace() {
                        if let Some(url) = part.strip_prefix("link=") {
                            link = Some(url.to_string());
                        } else {
                            style_parts.push(part);
                        }
                    }

                    let mut style = Style::parse(&style_parts.join(" "));
                    // Resolve words that aren't built-in styles/colors against
                    // the theme's custom colors (e.g. a color named "brand"
                    // enables `[brand]text[/]`).
                    let mut unknown_words = 0;
                    for part in &style_parts {
                        let lower = part.to_lowercase();
                        if style_is_unknown_word(&lower) {
                            if let Some(color) = theme.custom.get(&lower) {
                                style = style.combine(&Style::new().foreground(*color));
                            } else {
                                unknown_words += 1;
                            }
                        }
                    }

                    // Heuristic: If we parsed no style and no link, it's likely not a tag
                    // (e.g. "[1, 2, 3]" from debug output). Treat it as literal text.
                    if style.is_empty() && link.is_none() {
                        // Reconstruct the original text
                        let original = format!("[{}]", tag_content);
                        tokens.push(MarkupToken::Text(original));
                    } else {
                        let _ = unknown_words;
                        tokens.push(MarkupToken::OpenTag(style, link));
                    }
                }
            }
            ':' => {
                // Scan a candidate emoji shortcode: :name:
                let mut emoji_name = String::new();
                let mut found_close = false;

                while let Some(&c) = chars.peek() {
                    if c == ':' {
                        chars.next();
                        found_close = true;
                        break;
                    } else if c.is_alphanumeric() || c == '_' || c == '-' || c == '+' {
                        emoji_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                // Only treat it as an emoji when enabled and the shortcode is
                // known; otherwise keep the text verbatim (so "12:30:45"
                // survives).
                if translate_emoji
                    && found_close
                    && !emoji_name.is_empty()
                    && crate::emoji::get_emoji(&emoji_name).is_some()
                {
                    // Flush current text
                    if !current_text.is_empty() {
                        tokens.push(MarkupToken::Text(std::mem::take(&mut current_text)));
                    }
                    tokens.push(MarkupToken::Emoji(emoji_name));
                } else {
                    current_text.push(':');
                    current_text.push_str(&emoji_name);
                    if found_close {
                        current_text.push(':');
                    }
                }
            }
            ']' => {
                // Consume one following ']' as part of the `]]` escape; either
                // way a single ']' lands in the text.
                if chars.peek() == Some(&']') {
                    chars.next();
                }
                current_text.push(']');
            }
            _ => {
                current_text.push(c);
            }
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        tokens.push(MarkupToken::Text(current_text));
    }

    tokens
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
    fn test_tokenize_escape_brackets() {
        let tokens = tokenize("[[escaped]]");
        assert_eq!(tokens, vec![MarkupToken::Text("[escaped]".to_string())]);
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
