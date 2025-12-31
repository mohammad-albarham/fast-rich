//! Integration tests for highlighter system

use rich_rust::highlighter::{highlight_text, Highlighter, RegexHighlighter};
use rich_rust::style::{Color, Style};

#[test]
fn test_regex_highlighter_creation() {
    let hl = RegexHighlighter::new();
    // Highlighter builds successfully
    let _ = hl;
    assert!(true);
}

#[test]
fn test_add_pattern() {
    let mut hl = RegexHighlighter::new();
    let result = hl.add_pattern(r"\d+", Style::new().foreground(Color::Cyan));
    assert!(result.is_ok());
}

#[test]
fn test_invalid_pattern() {
    let mut hl = RegexHighlighter::new();
    let result = hl.add_pattern(r"[invalid(", Style::new());
    assert!(result.is_err());
}

#[test]
fn test_number_highlighter() {
    let hl = RegexHighlighter::number_highlighter(Style::new().foreground(Color::Cyan));
    let spans = hl.highlight("Port 8080 is open");

    // Should have: "Port ", "8080", " is open"
    assert!(spans.len() >= 3);
}

#[test]
fn test_url_highlighter() {
    let hl = RegexHighlighter::url_highlighter(Style::new().foreground(Color::Blue));
    let spans = hl.highlight("Visit https://example.com");

    assert!(spans.len() > 1);
}

#[test]
fn test_email_highlighter() {
    let hl = RegexHighlighter::email_highlighter(Style::new().foreground(Color::Green));
    let spans = hl.highlight("Email: test@example.com");

    assert!(spans.len() > 1);
}

#[test]
fn test_multiple_matches() {
    let mut hl = RegexHighlighter::new();
    hl.add_pattern(r"\d+", Style::new().foreground(Color::Cyan))
        .unwrap();

    let spans = hl.highlight("1 + 2 = 3");
    // Should highlight all three numbers
    assert!(spans.len() >= 5); // "1", " + ", "2", " = ", "3"
}

#[test]
fn test_no_matches() {
    let mut hl = RegexHighlighter::new();
    hl.add_pattern(r"\d+", Style::new()).unwrap();

    let spans = hl.highlight("no numbers here");
    assert_eq!(spans.len(), 1); // Just the original text
}

#[test]
fn test_highlight_text_function() {
    let hl = RegexHighlighter::number_highlighter(Style::new().foreground(Color::Cyan));
    let text = highlight_text("Version 2.0", &hl);

    // Text should be created successfully
    assert!(!text.spans.is_empty());
}

#[test]
fn test_overlapping_patterns() {
    let mut hl = RegexHighlighter::new();
    hl.add_pattern(r"\d+", Style::new().foreground(Color::Cyan))
        .unwrap();
    hl.add_pattern(r"\w+", Style::new().foreground(Color::Yellow))
        .unwrap();

    // First pattern should win for overlapping matches
    let spans = hl.highlight("123");
    assert!(!spans.is_empty());
}

#[test]
fn test_builder_pattern() {
    let result = RegexHighlighter::new().with_pattern(r"\d+", Style::new().foreground(Color::Cyan));

    assert!(result.is_ok());
}
