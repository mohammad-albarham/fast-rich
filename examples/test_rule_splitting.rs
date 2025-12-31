//! Test example to investigate divider splitting bug

use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    println!("=== Testing Rule with various inputs ===\n");

    // Test 1: Rule::line() - simple horizontal rule
    println!("Test 1: Rule::line()");
    console.print_renderable(&Rule::line());
    println!();

    // Test 2: Rule::new("") - empty string
    println!("\nTest 2: Rule::new(\"\")");
    console.print_renderable(&Rule::new(""));
    println!();

    // Test 3: Rule::new("Title")
    println!("\nTest 3: Rule::new(\"Title\")");
    console.print_renderable(&Rule::new("Title"));
    println!();

    // Test 4: Rule with very short width (via captured)
    println!("\nTest 4: Captured with width check");
    let capture_console = Console::capture();
    capture_console.print_renderable(&Rule::line());
    let output = capture_console.get_captured_output();
    println!(
        "Captured output bytes: {:?}",
        output.as_bytes().iter().take(50).collect::<Vec<_>>()
    );
    println!(
        "Has space in middle: {}",
        output.chars().filter(|c| *c == ' ').count() > 0
    );

    // Test 5: Rule with title, check for centered splitting
    println!("\nTest 5: Rule with title captured");
    let capture_console = Console::capture();
    capture_console.print_renderable(&Rule::new("Section"));
    let output = capture_console.get_captured_output();
    let stripped: String = output.chars().filter(|c| !c.is_control()).collect();
    println!("Stripped output: '{}'", stripped);
    println!("Contains '  ' (double space): {}", stripped.contains("  "));

    // Test 6: Check character around title
    println!("\nTest 6: Character analysis around title");
    if let Some(title_pos) = stripped.find("Section") {
        let chars: Vec<char> = stripped.chars().collect();
        let title_char_start = stripped[..title_pos].chars().count();
        let before: String = chars[title_char_start.saturating_sub(3)..title_char_start]
            .iter()
            .collect();
        let after: String = chars[title_char_start + 7..].iter().take(3).collect();
        println!("Before title: '{}'", before);
        println!("After title: '{}'", after);
    }
}
