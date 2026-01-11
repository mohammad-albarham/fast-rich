//! Integration tests for print!/println! macro shadowing
//!
//! These tests verify that the print macros work correctly as drop-in
//! replacements for std::print!/std::println!

// Import our macros to shadow std
use fast_rich::{eprint, eprintln, print, println, rprint, rprintln};

#[test]
fn test_println_basic() {
    // Should compile and run without panic
    println!("[bold]Hello[/] World!");
}

#[test]
fn test_println_format_args() {
    let name = "Test";
    let value = 42;
    println!("[cyan]{}[/] = [yellow]{}[/]", name, value);
}

#[test]
fn test_println_named_args() {
    println!(
        "[green]{name}[/] scored [yellow]{score}[/]",
        name = "Alice",
        score = 100
    );
}

#[test]
fn test_println_no_args() {
    // Empty println should just print newline
    println!();
}

#[test]
fn test_print_inline() {
    // print! should not add newline
    print!("[blue]Loading[/]");
    print!("...");
    println!(" [green]Done[/]");
}

#[test]
fn test_eprint_macros() {
    // stderr variants
    eprintln!("[red]Error message[/]");
    eprint!("[yellow]Warning: [/]");
    eprintln!("details here");
}

#[test]
fn test_eprintln_no_args() {
    eprintln!();
}

#[test]
fn test_rprint_aliases() {
    // Aliases work the same way
    rprint!("[bold]Bold[/] ");
    rprintln!("[italic]Italic[/]");
}

#[test]
fn test_rprintln_no_args() {
    rprintln!();
}

#[test]
fn test_no_markup_passthrough() {
    // Plain strings should work unchanged
    println!("Plain text without any markup");
}

#[test]
fn test_literal_braces() {
    // Double braces should produce literal braces
    println!("Literal braces: {{not markup}}");
}

#[test]
fn test_unicode_and_emoji() {
    println!("[bold]Unicode:[/] 你好世界 🎉 مرحبا");
}

#[test]
fn test_multiline() {
    println!(
        "[bold magenta]Header[/]\n\
         Line 1\n\
         Line 2"
    );
}

#[test]
fn test_format_with_expressions() {
    let x = 10;
    let y = 20;
    println!("[green]Sum:[/] {} + {} = [bold]{}[/]", x, y, x + y);
}

#[test]
fn test_debug_format() {
    // Smart parser should handle debug output with brackets properly (treating them as text)
    let p = vec![1, 2, 3];
    println!("Vector: {:?}", p); // Should print "Vector: [1, 2, 3]"

    // Also verify normal struct output
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let pt = Point { x: 10, y: 20 };
    println!("[cyan]Point:[/] {:?}", pt);
}

#[test]
fn test_smart_bracket_handling() {
    // These should be treated as text because they contain invalid styles
    println!("Invalid style: [1, 2, 3]");
    println!("Unknown word: [nonsense]");
    println!("Empty tag: []");

    // These should be treated as tags
    println!("Valid style: [bold]bold text[/]");
}

#[test]
fn test_edge_case_collision() {
    #[derive(Debug)]
    enum Status {
        Red,
    }
    let s = vec![Status::Red];
    println!("Breaking case: {:?}", s);
}

#[test]
fn test_empty_string() {
    println!("");
}

#[test]
fn test_numeric_formats() {
    println!("Hex: [yellow]{:x}[/], Binary: [cyan]{:b}[/]", 255, 255);
}

// =============================================================================
// Raw Print Macro Tests (no markup parsing)
// =============================================================================

use fast_rich::{eprint_raw, eprintln_raw, print_raw, println_raw};

#[test]
fn test_println_raw_basic() {
    // Brackets should be printed literally, not interpreted as markup
    println_raw!("Data: [1, 2, 3]");
}

#[test]
fn test_println_raw_debug() {
    // Debug format with brackets should work correctly
    let data = vec![1, 2, 3, 4, 5];
    println_raw!("Vector: {:?}", data);
}

#[test]
fn test_println_raw_format_args() {
    let x = 10;
    let y = 20;
    println_raw!("{} + {} = {}", x, y, x + y);
}

#[test]
fn test_println_raw_no_args() {
    println_raw!();
}

#[test]
fn test_print_raw_inline() {
    print_raw!("No newline");
    println_raw!(" - but this has one");
}

#[test]
fn test_eprintln_raw_stderr() {
    eprintln_raw!("Error data: {:?}", vec!["a", "b"]);
}

#[test]
fn test_eprint_raw_inline() {
    eprint_raw!("Stderr: ");
    eprintln_raw!("message");
}

#[test]
fn test_raw_markup_not_parsed() {
    // This should print [bold] literally, not make text bold
    println_raw!("[bold]This is not bold[/]");
}
