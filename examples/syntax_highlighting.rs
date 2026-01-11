use fast_rich::console::Console;
use fast_rich::syntax::{Syntax, Theme};

fn main() {
    let console = Console::new();

    console.rule("Syntax Highlighting Features");

    // 1. Rust code with Monokai (start line 10)
    console.rule("Rust (Monokai, Line 10)");
    let rust_code = r#"fn main() {
    println!("Hello, World!");
    let x = 42;
    // This is a comment
}"#;

    // Note: line numbering starts at 1 by default.
    // If we want it to start at 10, we need to modify SyntaxConfig or add a method.
    // Checking src/syntax.rs, there is no method exposed to set start_line yet?
    // Let's check SyntaxConfig. It has start_line pub, but Syntax has config() method.
    // We can use that.

    let config = fast_rich::syntax::SyntaxConfig {
        start_line: 10,
        theme: Theme::Monokai,
        highlight_lines: vec![12],
        ..Default::default()
    };

    let syntax = Syntax::new(rust_code, "rust").config(config);
    console.print_renderable(&syntax);
    console.print("\n");

    // 2. Python code with Base16OceanDark (matches typical dark terminal preference)
    console.rule("Python (Base16OceanDark, No Line Numbers)");
    let python_code = r#"def hello(name):
    """Greets the user."""
    print(f"Hello, {name}!")
    return True"#;

    let syntax = Syntax::new(python_code, "python")
        .theme(Theme::Base16OceanDark)
        .line_numbers(false);
    console.print_renderable(&syntax);
    console.print("\n");

    // 3. JSON without panel
    console.rule("JSON (No Panel)");
    let json_code = r#"{
  "name": "fast-rich",
  "version": "0.1.0",
  "features": ["syntax", "markdown"]
}"#;
    let syntax = Syntax::new(json_code, "json")
        .panel(false)
        .theme(Theme::SolarizedDark);
    console.print_renderable(&syntax);
    console.print("\n");
}
