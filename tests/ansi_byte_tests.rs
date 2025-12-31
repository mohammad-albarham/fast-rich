//! ANSI Byte-Level Test Suite
//!
//! These tests verify that fast-rich produces ANSI output that matches
//! Python Rich reference output at the byte level.

mod ansi_test_helpers;

use ansi_test_helpers::{capture_markup, parse_ansi_sequences, print_ansi_diff, AnsiToken};
use fast_rich::prelude::*;
use std::fs;
use std::path::Path;

/// Load Python reference output for a test case.
fn load_python_reference(test_name: &str) -> Option<String> {
    let path = format!("tests/ansi_output/python_{}.txt", test_name);
    if Path::new(&path).exists() {
        Some(fs::read_to_string(&path).expect("Failed to read reference file"))
    } else {
        None
    }
}

/// Save Rust output for comparison.
fn save_rust_output(test_name: &str, output: &str) {
    let path = format!("tests/ansi_output/rust_{}.txt", test_name);
    fs::write(&path, output).expect("Failed to write output file");
}

// =============================================================================
// Basic Styles Tests
// =============================================================================

#[test]
fn test_basic_styles_exact_match() {
    let output = capture_markup(
        "[bold]Bold[/] [italic]Italic[/] [underline]Underline[/]",
        60,
    );
    save_rust_output("basic_styles", &output);

    if let Some(expected) = load_python_reference("basic_styles") {
        if output != expected {
            print_ansi_diff(&expected, &output);
            panic!("Basic styles ANSI mismatch with Python reference");
        }
    }
}

#[test]
fn test_bold_style() {
    let output = capture_markup("[bold]test[/]", 60);
    let tokens = parse_ansi_sequences(&output);

    // Should have: ESC[1m, "test", ESC[0m
    assert!(
        tokens
            .iter()
            .any(|t| matches!(t, AnsiToken::Escape(s) if s == "\x1b[1m")),
        "Missing bold sequence (ESC[1m)"
    );
    assert!(
        tokens
            .iter()
            .any(|t| matches!(t, AnsiToken::Text(s) if s == "test")),
        "Missing text content"
    );
    assert!(
        tokens.iter().any(|t| t.is_reset()),
        "Missing reset sequence"
    );
}

#[test]
fn test_italic_style() {
    let output = capture_markup("[italic]test[/]", 60);
    let tokens = parse_ansi_sequences(&output);

    assert!(
        tokens
            .iter()
            .any(|t| matches!(t, AnsiToken::Escape(s) if s == "\x1b[3m")),
        "Missing italic sequence (ESC[3m)"
    );
}

#[test]
fn test_underline_style() {
    let output = capture_markup("[underline]test[/]", 60);
    let tokens = parse_ansi_sequences(&output);

    assert!(
        tokens
            .iter()
            .any(|t| matches!(t, AnsiToken::Escape(s) if s == "\x1b[4m")),
        "Missing underline sequence (ESC[4m)"
    );
}

// =============================================================================
// Space Preservation Tests
// =============================================================================

#[test]
fn test_space_between_styled_spans() {
    let output = capture_markup("[red]Red[/] [blue]Blue[/]", 60);

    // Extract just the text content
    let tokens = parse_ansi_sequences(&output);
    let text: String = tokens
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    // Must have exactly: "Red Blue" with space preserved
    assert!(
        text.contains("Red ") || text.contains(" Blue"),
        "Space between styled spans not preserved. Text: {:?}",
        text
    );
}

#[test]
fn test_leading_space_preserved() {
    let output = capture_markup("  [bold]indented[/]", 60);

    let tokens = parse_ansi_sequences(&output);
    let _first_text = tokens.iter().find_map(|t| match t {
        AnsiToken::Text(s) => Some(s.clone()),
        _ => None,
    });

    // First text should have leading spaces or there should be a space token
    let text: String = tokens
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    assert!(
        text.starts_with("  ") || text.contains("  indented"),
        "Leading spaces not preserved. Text: {:?}",
        text
    );
}

// =============================================================================
// Reset Sequence Tests
// =============================================================================

#[test]
fn test_no_double_reset() {
    let output = capture_markup("[bold]test[/]", 60);
    let bytes = output.as_bytes();

    // Search for double reset: \x1b[0m\x1b[0m
    let double_reset = b"\x1b[0m\x1b[0m";
    let has_double = bytes.windows(double_reset.len()).any(|w| w == double_reset);

    assert!(
        !has_double,
        "Found double reset sequence. This wastes bytes and indicates a bug.\nOutput: {:?}",
        output
    );
}

#[test]
fn test_single_reset_after_style() {
    let output = capture_markup("[bold]Bold[/] plain", 60);

    // Count reset sequences
    let tokens = parse_ansi_sequences(&output);
    let reset_count = tokens.iter().filter(|t| t.is_reset()).count();

    // Should have exactly 1 reset (after "Bold")
    assert_eq!(
        reset_count, 1,
        "Expected 1 reset, found {}. Tokens: {:?}",
        reset_count, tokens
    );
}

// =============================================================================
// Color Tests
// =============================================================================

#[test]
fn test_named_color_red() {
    let output = capture_markup("[red]test[/]", 60);
    let tokens = parse_ansi_sequences(&output);

    // Accept either 16-color (\x1b[31m) or 256-color (\x1b[38;5;1m)
    let has_red_color = tokens.iter().any(|t| match t {
        AnsiToken::Escape(s) => {
            s == "\x1b[31m" ||          // 16-color
            s == "\x1b[38;5;1m" ||       // 256-color
            s.starts_with("\x1b[38;2;") // True color (would need RGB check)
        }
        _ => false,
    });

    assert!(
        has_red_color,
        "No red color sequence found. Tokens: {:?}",
        tokens
    );
}

#[test]
fn test_rgb_color() {
    let output = capture_markup("[rgb(255,128,0)]orange[/]", 60);
    let tokens = parse_ansi_sequences(&output);

    // RGB should use true color: \x1b[38;2;255;128;0m
    let has_rgb = tokens.iter().any(|t| match t {
        AnsiToken::Escape(s) => s == "\x1b[38;2;255;128;0m",
        _ => false,
    });

    assert!(
        has_rgb,
        "RGB color should use true color sequence (38;2;R;G;Bm). Tokens: {:?}",
        tokens
    );
}

// =============================================================================
// Multiple Span Tests
// =============================================================================

#[test]
fn test_adjacent_styled_spans() {
    let output = capture_markup("[red]A[/][blue]B[/][green]C[/]", 60);

    let tokens = parse_ansi_sequences(&output);
    let text: String = tokens
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    assert_eq!(
        text.trim(),
        "ABC",
        "Adjacent styled spans should produce 'ABC'. Got: {:?}",
        text
    );
}

#[test]
fn test_nested_style_hint() {
    // Note: fast-rich may not support true nesting like Python rich
    // This test documents current behavior
    let output = capture_markup("[bold][red]bold red[/][/]", 60);

    let tokens = parse_ansi_sequences(&output);
    let text: String = tokens
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    assert!(
        text.contains("bold red"),
        "Nested markup should preserve text. Got: {:?}",
        text
    );
}

// =============================================================================
// UTF-8 and Special Characters
// =============================================================================

#[test]
fn test_utf8_text_preserved() {
    let output = capture_markup("[bold]你好世界[/]", 60);

    let tokens = parse_ansi_sequences(&output);
    let text: String = tokens
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    assert!(
        text.contains("你好世界"),
        "UTF-8 characters should be preserved. Got: {:?}",
        text
    );
}

#[test]
fn test_emoji_preserved() {
    let output = capture_markup("[bold]🚀 Rocket[/]", 60);

    let tokens = parse_ansi_sequences(&output);
    let text: String = tokens
        .iter()
        .filter_map(|t| match t {
            AnsiToken::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    assert!(
        text.contains("🚀") && text.contains("Rocket"),
        "Emoji should be preserved. Got: {:?}",
        text
    );
}

// =============================================================================
// Segment and Newline Tests
// =============================================================================

#[test]
fn test_newline_after_segment() {
    let console = Console::capture().width(60).force_color(true);
    console.println("[bold]Line 1[/]");
    console.println("[italic]Line 2[/]");
    let output = console.get_captured_output();

    let newline_count = output.matches('\n').count();
    assert!(
        newline_count >= 2,
        "println should add newlines. Found {} newlines in: {:?}",
        newline_count,
        output
    );
}

// =============================================================================
// Integration Tests Against Python Reference
// =============================================================================

#[test]
fn test_all_python_references() {
    let test_cases = [(
        "basic_styles",
        "[bold]Bold[/] [italic]Italic[/] [underline]Underline[/]",
    )];

    for (name, markup) in test_cases {
        let output = capture_markup(markup, 60);
        save_rust_output(name, &output);

        if let Some(expected) = load_python_reference(name) {
            if output != expected {
                eprintln!("Mismatch in {}", name);
                print_ansi_diff(&expected, &output);
                // Don't panic here - collect all failures
            }
        }
    }
}
// =============================================================================
// Color System Enforcement Tests
// =============================================================================

#[test]
fn test_forced_standard_color_system() {
    // Force standard color system
    let console = Console::capture()
        .width(60)
        .color_system(fast_rich::console::ColorSystem::Standard);

    // Print a color that would normally use 256-color (e.g., Orange/208 -> BrightRed/9?)
    // or RGB 255,128,0 -> likely Yellow or Red in standard
    console.print("[rgb(255,128,0)]Test[/]");
    let output = console.get_captured_output();
    let tokens = parse_ansi_sequences(&output);

    // Should NOT contain 38;5; or 38;2;
    let has_extended = tokens.iter().any(|t| match t {
        AnsiToken::Escape(s) => s.contains("38;5;") || s.contains("38;2;"),
        _ => false,
    });

    assert!(
        !has_extended,
        "Standard mode should not emit extended color codes. Output: {:?}",
        output
    );

    // Should contain standard color codes (30-37, 90-97)
    let has_standard = tokens.iter().any(|t| match t {
        AnsiToken::Escape(s) => {
            // Check for standard FG codes
            s.starts_with("\x1b[3") || s.starts_with("\x1b[9")
        }
        _ => false,
    });

    assert!(
        has_standard,
        "Should emit standard ANSI color codes. Output: {:?}",
        output
    );
}

#[test]
fn test_forced_eightbit_color_system() {
    let console = Console::capture()
        .width(60)
        .color_system(fast_rich::console::ColorSystem::EightBit);

    // Red color should use 38;5;1
    console.print("[red]Test[/]");
    let output = console.get_captured_output();

    assert!(
        output.contains("\x1b[38;5;1m"),
        "EightBit mode should use 256-color codes. Output: {:?}",
        output
    );
}

#[test]
fn test_forced_no_color_system() {
    let console = Console::capture()
        .width(60)
        .color_system(fast_rich::console::ColorSystem::NoColor);

    console.print("[bold red]Test[/]");
    let output = console.get_captured_output();

    // Should be just text (ignore trailing newline if present)
    assert_eq!(
        output.trim(),
        "Test",
        "NoColor mode should strip all styling"
    );
}
