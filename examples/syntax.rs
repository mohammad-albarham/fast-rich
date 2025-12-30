//! Syntax highlighting example.
//!
//! Demonstrates the high-performance syntax highlighting capabilities using syntect.

use rich_rust::{syntax::Syntax, Console};

fn main() {
    let console = Console::new();

    let rust_code = r#"
fn main() {
    let x = 42;
    println!("Hello, {}!", x);
}
"#;

    let python_code = r#"
def main():
    x = 42
    print(f"Hello, {x}!")
"#;

    let config = r#"
[server]
port = 8080
host = "0.0.0.0"
"#;

    console.println("[bold cyan]═══ Syntax Highlighting ═══[/]");
    console.println("");

    console.println("[yellow]Rust Code:[/]");
    let syntax = Syntax::new(rust_code, "rust");
    console.print_renderable(&syntax);
    console.println("");

    console.println("[yellow]Python Code (Solarized):[/]");
    let syntax = Syntax::new(python_code, "python").theme(rich_rust::syntax::Theme::SolarizedDark);
    console.print_renderable(&syntax);
    console.println("");

    console.println("[yellow]TOML Config (No Panel):[/]");
    let syntax = Syntax::new(config, "toml").panel(false).line_numbers(false);
    console.print_renderable(&syntax);
}
