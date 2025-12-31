//! ANSI Comparison Test Suite
//! Compares fast-rich output with Python Rich reference output

use fast_rich::align::Align;
use fast_rich::prelude::*;
use std::fs;
use std::path::Path;

fn capture_ansi(renderable: &dyn fast_rich::renderable::Renderable, width: usize) -> String {
    let console = Console::capture().width(width).force_color(true);
    console.print_renderable(renderable);
    console.get_captured_output()
}

fn test_basic_styles() -> String {
    let text = fast_rich::markup::parse("[bold]Bold[/] [italic]Italic[/] [underline]Underline[/]");
    capture_ansi(&text, 60)
}

fn test_colors() -> String {
    let text = fast_rich::markup::parse(
        "[red]Red[/] [green]Green[/] [blue]Blue[/] [rgb(255,128,0)]Orange[/]",
    );
    capture_ansi(&text, 60)
}

fn test_table() -> String {
    let mut table = Table::new().set_title("Test Table");
    let col1 = Column::new("Language").style(Style::new().foreground(Color::Cyan));
    let col2 = Column::new("Features").style(Style::new().foreground(Color::Magenta));
    table.add_column(col1);
    table.add_column(col2);
    table.add_row_strs(&["Rust", "Fast & Safe"]);
    table.add_row_strs(&["Python", "Easy & Rich"]);
    capture_ansi(&table, 60)
}

fn test_panel() -> String {
    let content = fast_rich::markup::parse("Hello, [bold green]World[/]!");
    let panel = Panel::new(content).title("Greeting");
    capture_ansi(&panel, 60)
}

fn test_align() -> String {
    let text = Text::plain("Centered Text").style(Style::new().bold().foreground(Color::Green));
    let aligned = Align::center(text);
    capture_ansi(&aligned, 60)
}

fn test_padding() -> String {
    let text = Text::plain("Padded").style(Style::new().foreground(Color::Cyan));
    let padded = fast_rich::padding::Padding::symmetric(text, 1, 2);
    capture_ansi(&padded, 60)
}

fn test_theme() -> String {
    // Rich-rust doesn't have markup tags like [success], so we'll test raw theme colors
    let theme = fast_rich::theme::Theme::default_theme();
    let mut segments = Vec::new();
    segments.push(Text::plain("Success").style(theme.get_style("success")));
    segments.push(Text::plain(" "));
    segments.push(Text::plain("Warning").style(theme.get_style("warning")));
    segments.push(Text::plain(" "));
    segments.push(Text::plain("Error").style(theme.get_style("error")));

    // Combine into single output
    let console = Console::capture().width(60).force_color(true);
    for segment in segments {
        console.print_renderable(&segment);
    }
    console.get_captured_output()
}

fn export_raw_bytes(test_name: &str, output: &str) {
    let path = format!("tests/ansi_output/rust_{}.txt", test_name);
    fs::write(&path, output.as_bytes()).expect("Failed to write output");
    println!("Exported: {}", path);
}

fn compare_outputs(test_name: &str) -> bool {
    let python_path = format!("tests/ansi_output/python_{}.txt", test_name);
    let rust_path = format!("tests/ansi_output/rust_{}.txt", test_name);

    if !Path::new(&python_path).exists() {
        println!("⚠  Python reference not found: {}", python_path);
        return false;
    }

    let python_output = fs::read_to_string(&python_path).expect("Failed to read Python output");
    let rust_output = fs::read_to_string(&rust_path).expect("Failed to read Rust output");

    if python_output == rust_output {
        println!("✓ {} - EXACT MATCH", test_name);
        true
    } else {
        println!("✗ {} - MISMATCH", test_name);
        println!("  Python length: {} bytes", python_output.len());
        println!("  Rust length:   {} bytes", rust_output.len());

        // Show first difference
        for (i, (p, r)) in python_output.bytes().zip(rust_output.bytes()).enumerate() {
            if p != r {
                println!(
                    "  First diff at byte {}: Python={:02x} Rust={:02x}",
                    i, p, r
                );
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_basic_styles() {
        let output = test_basic_styles();
        export_raw_bytes("basic_styles", &output);
        // Uncomment after running Python reference:
        // assert!(compare_outputs("basic_styles"), "Basic styles ANSI mismatch");
    }

    #[test]
    fn verify_colors() {
        let output = test_colors();
        export_raw_bytes("colors", &output);
    }

    #[test]
    fn verify_table() {
        let output = test_table();
        export_raw_bytes("table", &output);
    }

    #[test]
    fn verify_panel() {
        let output = test_panel();
        export_raw_bytes("panel", &output);
    }

    #[test]
    fn verify_align() {
        let output = test_align();
        export_raw_bytes("align", &output);
    }
}

fn main() {
    fs::create_dir_all("tests/ansi_output").expect("Failed to create output dir");

    let tests = [
        ("basic_styles", test_basic_styles as fn() -> String),
        ("colors", test_colors),
        ("table", test_table),
        ("panel", test_panel),
        ("align", test_align),
        ("padding", test_padding),
        ("theme", test_theme),
    ];

    println!("=== Generating Rust ANSI Outputs ===\n");
    for (name, test_func) in &tests {
        let output = test_func();
        println!("=== {} ===", name.to_uppercase());
        println!("{:?}", output);
        export_raw_bytes(name, &output);
    }

    println!("\n=== Comparing with Python Reference ===\n");
    let mut all_match = true;
    for (name, _) in &tests {
        if !compare_outputs(name) {
            all_match = false;
        }
    }

    if all_match {
        println!("\n✓ All tests match Python Rich output exactly!");
    } else {
        println!("\n⚠ Some tests have mismatches - see details above");
    }
}
