use rich_rust::prelude::*;

fn main() {
    let console = Console::new().force_color(true);

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
