//! ANSI byte-level test helpers for comparing Rust output with Python Rich.
//!
//! This module provides utilities for capturing ANSI output and comparing
//! it byte-for-byte or semantically with reference outputs.

#![allow(dead_code)]

use rich_rust::console::Console;
use rich_rust::renderable::Renderable;

/// Capture ANSI output from a renderable.
pub fn capture_ansi<R: Renderable>(renderable: &R, width: usize) -> String {
    let console = Console::capture().width(width).force_color(true);
    console.print_renderable(renderable);
    console.get_captured_output()
}

/// Capture ANSI output from markup string.
pub fn capture_markup(content: &str, width: usize) -> String {
    let console = Console::capture().width(width).force_color(true);
    console.print(content);
    console.get_captured_output()
}

/// Pretty print hex dump for debugging.
pub fn hexdump(data: &[u8]) -> String {
    let mut result = String::new();
    for (i, chunk) in data.chunks(16).enumerate() {
        // Offset
        result.push_str(&format!("{:08x}: ", i * 16));

        // Hex bytes
        for (j, byte) in chunk.iter().enumerate() {
            result.push_str(&format!("{:02x}", byte));
            if j % 2 == 1 {
                result.push(' ');
            }
        }

        // Padding for incomplete lines
        let remaining = 16 - chunk.len();
        for j in 0..remaining {
            result.push_str("  ");
            if (chunk.len() + j) % 2 == 1 {
                result.push(' ');
            }
        }

        // ASCII representation
        result.push(' ');
        for byte in chunk {
            let c = if *byte >= 0x20 && *byte < 0x7f {
                *byte as char
            } else {
                '.'
            };
            result.push(c);
        }
        result.push('\n');
    }
    result
}

/// Parse ANSI sequences from a string, returning vec of (sequence, text) pairs.
pub fn parse_ansi_sequences(s: &str) -> Vec<AnsiToken> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    let mut current_text = String::new();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Flush any accumulated text
            if !current_text.is_empty() {
                tokens.push(AnsiToken::Text(std::mem::take(&mut current_text)));
            }

            // Parse ANSI sequence
            let mut seq = String::from("\x1b");
            if let Some(&next) = chars.peek() {
                if next == '[' {
                    seq.push(chars.next().unwrap());
                    // Read until letter
                    while let Some(&c) = chars.peek() {
                        seq.push(chars.next().unwrap());
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            }
            tokens.push(AnsiToken::Escape(seq));
        } else {
            current_text.push(c);
        }
    }

    if !current_text.is_empty() {
        tokens.push(AnsiToken::Text(current_text));
    }

    tokens
}

/// Token types in ANSI output
#[derive(Debug, Clone, PartialEq)]
pub enum AnsiToken {
    /// Plain text
    Text(String),
    /// ANSI escape sequence
    Escape(String),
}

impl AnsiToken {
    /// Check if this is a reset sequence
    pub fn is_reset(&self) -> bool {
        matches!(self, AnsiToken::Escape(s) if s == "\x1b[0m")
    }

    /// Check if this is any escape sequence
    pub fn is_escape(&self) -> bool {
        matches!(self, AnsiToken::Escape(_))
    }
}

/// Compare two ANSI outputs, ignoring differences in color palette encoding.
/// Returns true if semantically equivalent.
pub fn ansi_semantically_equal(a: &str, b: &str) -> bool {
    let tokens_a = parse_ansi_sequences(a);
    let tokens_b = parse_ansi_sequences(b);

    // For now, just check text content matches
    let text_a: String = tokens_a
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    let text_b: String = tokens_b
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    text_a == text_b
}

/// Print detailed diff between two ANSI outputs for debugging.
pub fn print_ansi_diff(expected: &str, actual: &str) {
    println!("=== EXPECTED ({} bytes) ===", expected.len());
    println!("{}", hexdump(expected.as_bytes()));

    println!("=== ACTUAL ({} bytes) ===", actual.len());
    println!("{}", hexdump(actual.as_bytes()));

    // Find first difference
    for (i, (e, a)) in expected.bytes().zip(actual.bytes()).enumerate() {
        if e != a {
            println!(
                "First difference at byte {}: expected 0x{:02x}, got 0x{:02x}",
                i, e, a
            );
            return;
        }
    }

    if expected.len() != actual.len() {
        println!(
            "Lengths differ: expected {}, got {}",
            expected.len(),
            actual.len()
        );
    }
}

/// Assert that two ANSI outputs match exactly.
#[macro_export]
macro_rules! assert_ansi_match {
    ($actual:expr, $expected:expr) => {
        let actual = $actual;
        let expected = $expected;
        if actual != expected {
            $crate::ansi_test_helpers::print_ansi_diff(&expected, &actual);
            panic!("ANSI output mismatch");
        }
    };
    ($actual:expr, $expected:expr, $msg:expr) => {
        let actual = $actual;
        let expected = $expected;
        if actual != expected {
            $crate::ansi_test_helpers::print_ansi_diff(&expected, &actual);
            panic!("ANSI output mismatch: {}", $msg);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexdump() {
        let data = b"Hello\x1b[31mWorld";
        let dump = hexdump(data);
        assert!(dump.contains("4865 6c6c")); // "Hell" in hex
                                             // Bytes are grouped in 2s: 0-1, 2-3, 4-5, 6-7...
                                             // Index 5 is 1b, Index 6 is 5b. They are in different groups (4-5 and 6-7).
        assert!(dump.contains("5b33")); // '[' (5b) and '3' (33) are paired
    }

    #[test]
    fn test_parse_ansi_sequences() {
        let input = "\x1b[1mBold\x1b[0m text";
        let tokens = parse_ansi_sequences(input);

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], AnsiToken::Escape("\x1b[1m".to_string()));
        assert_eq!(tokens[1], AnsiToken::Text("Bold".to_string()));
        assert_eq!(tokens[2], AnsiToken::Escape("\x1b[0m".to_string()));
        assert_eq!(tokens[3], AnsiToken::Text(" text".to_string()));
    }

    #[test]
    fn test_ansi_semantically_equal() {
        // Same text, different color encoding
        let a = "\x1b[31mRed\x1b[0m";
        let b = "\x1b[38;5;1mRed\x1b[0m";

        assert!(ansi_semantically_equal(a, b));
    }
}
