use rich_rust::prelude::*;

fn run(console: &Console) {
    console.rule("[bold blue]Markdown & Syntax Demo[/]");
    console.newline();

    // 1. Markdown
    #[cfg(feature = "markdown")]
    {
        console.print("[bold]1. Markdown Rendering[/]");
        let md_content = r#"
# Fast Rich Markdown

Rich supports **markdown** with typical features:

## Features
- Headers (H1-H6)
- *Italic*, **Bold**, `Code`
- Bulleted lists
- [Links](https://example.com)

> Blockquotes are correctly formatted
> with styling.

```rust
fn main() {
    println!("Code blocks too!");
}
```
        "#;

        let md = rich_rust::markdown::Markdown::new(md_content);
        console.print_renderable(&md);
        console.newline();
    }

    #[cfg(not(feature = "markdown"))]
    console.print("[yellow]Markdown feature not enabled. Run with --features markdown[/]");

    // 2. Syntax Highlighting
    #[cfg(feature = "syntax")]
    {
        console.print("[bold]2. Syntax Highlighting[/]");

        let code = r#"
[
    {
        "name": "fast_rich",
        "language": "Rust",
        "speed": 100,
        "tags": ["cli", "tui", "rich"]
    }
]
        "#;

        let syntax = rich_rust::syntax::Syntax::new(code, "json");
        // Syntax implements Renderable, so we can print it directly
        console.print_renderable(&syntax);
    }

    #[cfg(not(feature = "syntax"))]
    console.print("[yellow]Syntax feature not enabled. Run with --features syntax[/]");

    console.rule("[bold blue]End Markdown Demo[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_syntax_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();
        eprintln!(
            "CAPTURED (Markdown={}):\n{}",
            cfg!(feature = "markdown"),
            output
        );

        assert!(output.contains("Markdown & Syntax Demo"));

        if cfg!(feature = "markdown") {
            assert!(output.contains("Fast Rich Markdown"));
            assert!(output.contains("Features"));
        } else {
            assert!(output.contains("Run"));
            assert!(output.contains("features"));
        }

        if cfg!(feature = "syntax") {
            assert!(output.contains("Syntax Highlighting"));
        }

        assert!(output.contains("End Markdown Demo"));
    }
}
