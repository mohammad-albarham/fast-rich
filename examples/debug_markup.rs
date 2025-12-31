// Test file to debug markup spacing issue
use rich_rust::markup;

fn main() {
    println!("=== Markup Spacing Debug ===\n");

    // Test 1: Simple case
    let test1 = "word1[bold]word2[/]word3";
    println!("Test 1: {}", test1);
    let result1 = markup::parse(test1);
    println!("  Result: '{}'", result1.plain_text());
    println!("  Expected: 'word1word2word3'");
    println!("  Spans:");
    for (i, span) in result1.spans.iter().enumerate() {
        println!("    {}: '{}'", i, span.text);
    }

    // Test 2: With space after tag
    let test2 = "word1[bold]word2[/] word3";
    println!("\nTest 2: {}", test2);
    let result2 = markup::parse(test2);
    println!("  Result: '{}'", result2.plain_text());
    println!("  Expected: 'word1word2 word3'");
    println!("  Spans:");
    for (i, span) in result2.spans.iter().enumerate() {
        println!("    {}: '{}'", i, span.text);
    }

    // Test 3: Real example
    let test3 = "Check [bold cyan]export.html[/] and [bold magenta]export.svg[/] files!";
    println!("\nTest 3: {}", test3);
    let result3 = markup::parse(test3);
    println!("  Result: '{}'", result3.plain_text());
    println!("  Expected: 'Check export.html and export.svg files!'");
    println!("  Spans:");
    for (i, span) in result3.spans.iter().enumerate() {
        println!("    {}: '{}' (style: {:?})", i, span.text, span.style);
    }
}
