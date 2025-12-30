//! Markup parser for Rich-style markup syntax.
//!
//! Parses text like `[bold red]Hello[/]` into styled spans.
//!
//! # Syntax
//!
//! - `[style]text[/]` - Apply style to text
//! - `[style]text[/style]` - Explicit close tag
//! - `[bold red on blue]` - Multiple styles
//! - `[@emoji_name]` - Emoji shortcode (handled separately)
//! - `[[` - Escaped `[`
//! - `]]` - Escaped `]`

use crate::style::Style;
use crate::text::{Span, Text};

/// A parsed token from markup text.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkupToken {
    /// Plain text
    Text(String),
    /// Opening style tag
    OpenTag(Style),
    /// Closing tag
    CloseTag,
    /// Emoji shortcode like :smile:
    Emoji(String),
}

/// Parse markup text into tokens.
pub fn tokenize(input: &str) -> Vec<MarkupToken> {
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
                        // Check for escape sequence ]]
                        chars.next();
                        if chars.peek() == Some(&']') {
                            chars.next();
                            tag_content.push(']');
                        } else {
                            found_close = true;
                            break;
                        }
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
                    // Style tag
                    let style = Style::parse(tag_content);
                    tokens.push(MarkupToken::OpenTag(style));
                }
            }
            ':' => {
                // Check for emoji
                let mut emoji_name = String::new();
                let mut found_close = false;

                while let Some(&c) = chars.peek() {
                    if c == ':' {
                        chars.next();
                        found_close = true;
                        break;
                    } else if c.is_alphanumeric() || c == '_' || c == '-' {
                        emoji_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if found_close && !emoji_name.is_empty() {
                    // Flush current text
                    if !current_text.is_empty() {
                        tokens.push(MarkupToken::Text(std::mem::take(&mut current_text)));
                    }
                    tokens.push(MarkupToken::Emoji(emoji_name));
                } else {
                    // Not an emoji, treat as regular text
                    current_text.push(':');
                    current_text.push_str(&emoji_name);
                    if found_close {
                        current_text.push(':');
                    }
                }
            }
            ']' => {
                // Check for escape sequence ]]
                if chars.peek() == Some(&']') {
                    chars.next();
                    current_text.push(']');
                } else {
                    current_text.push(']');
                }
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
    let tokens = tokenize(input);
    let mut spans = Vec::new();
    let mut style_stack: Vec<Style> = Vec::new();

    for token in tokens {
        match token {
            MarkupToken::Text(text) => {
                let style = style_stack.last().cloned().unwrap_or_default();
                spans.push(Span::styled(text, style));
            }
            MarkupToken::OpenTag(style) => {
                let combined = if let Some(current) = style_stack.last() {
                    current.combine(&style)
                } else {
                    style
                };
                style_stack.push(combined);
            }
            MarkupToken::CloseTag => {
                style_stack.pop();
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
        assert!(matches!(tokens[0], MarkupToken::OpenTag(_)));
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
}
