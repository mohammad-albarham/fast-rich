# Getting Started

This guide will help you get up and running with Fast-Rich in minutes.

## Installation

Add `fast-rich` to your `Cargo.toml`:

=== "Basic"

    ```toml
    [dependencies]
    fast-rich = "0.2.0"
    ```

=== "Full Features"

    ```toml
    [dependencies]
    fast-rich = { version = "0.2.0", features = ["full"] }
    ```

=== "Selective Features"

    ```toml
    [dependencies]
    fast-rich = { version = "0.2.0", features = ["syntax", "markdown", "logging"] }
    ```

### Available Features

| Feature | Description |
|:--------|:------------|
| `syntax` | Syntax highlighting for code (adds `syntect` dependency) |
| `markdown` | Markdown rendering (adds `pulldown-cmark` dependency) |
| `logging` | Integration with Rust's `log` crate |
| `full` | Enables all features above |

## Your First Program

!!! example "Hello Fast-Rich"

    **Code**
    
    Create a new file `src/main.rs`:
    
    ```rust
    use fast_rich::prelude::*;
    
    fn main() {
        let console = Console::new();
        console.print("[bold green]Welcome to Fast-Rich![/]");
    }
    ```
    
    **Run it**
    
    ```bash
    cargo run
    ```
    
    **What you'll see**
    
    ![First program output](assets/hero.gif)

## Understanding the Console

The `Console` is your main entry point for all output. It handles:

- Terminal width detection
- Color capability detection
- Styled output rendering

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    // Simple print (no newline)
    console.print("Hello ");
    
    // Print with newline
    console.println("World!");
    
    // Print a decorative rule
    console.rule("[bold]Section Title[/]");
    
    // Add blank lines
    console.newline();
}
```

## Markup Syntax

Fast-Rich uses a simple markup syntax for inline styling:

```rust
// Basic styles
console.print("[bold]Bold text[/]");
console.print("[italic]Italic text[/]");
console.print("[underline]Underlined[/]");

// Colors
console.print("[red]Red text[/]");
console.print("[blue]Blue text[/]");
console.print("[color(255,128,0)]Orange (RGB)[/]");

// Combinations
console.print("[bold red on white]Bold red on white background[/]");

// Nesting
console.print("[bold]Bold [italic]and italic[/italic] back to bold[/]");
```

### Escape Markup

To display literal brackets, escape them:

```rust
console.print("Use \\[bold] to show [bold] literally");
```

## Printing Renderables

For complex objects like tables, panels, and trees, use `print_renderable`:

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    // Create a table
    let mut table = Table::new();
    table.add_column("Name");
    table.add_column("Value");
    table.add_row_strs(&["Version", "0.2.0"]);
    table.add_row_strs(&["Author", "Mohammad"]);
    
    // Print the table
    console.print_renderable(&table);
}
```

**Output:**

```
╭─────────┬──────────╮
│ Name    │ Value    │
├─────────┼──────────┤
│ Version │ 0.2.0    │
│ Author  │ Mohammad │
╰─────────┴──────────╯
```

## Terminal Compatibility

Fast-Rich automatically detects terminal capabilities:

| Environment | Color Support |
|:------------|:--------------|
| Modern terminals (iTerm2, Windows Terminal, etc.) | Full RGB/Truecolor |
| Most Linux/macOS terminals | 256 colors |
| Basic terminals | 8/16 colors |
| CI/CD, pipes, redirects | Colors disabled |

!!! tip "Force Colors"
    To force color output (e.g., in CI):
    
    ```rust
    let console = Console::new().force_color(true);
    ```

## Next Steps

Now that you have the basics, explore the feature guides:

- **[Text & Styles](guides/text-styles.md)** - Deep dive into colors and styling
- **[Tables](guides/tables.md)** - Create beautiful data tables
- **[Progress Bars](guides/progress.md)** - Track long-running tasks
- **[Live Display](guides/live.md)** - Build dynamic dashboards

Or run the examples:

```bash
cargo run --example showcase --features full
```
