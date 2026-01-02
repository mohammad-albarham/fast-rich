<div align="center">
  <img src="assets/logo.png" width="800" alt="Fast-Rich Logo" />
</div>

# Fast-Rich

<div align="center" markdown>

[![Crates.io](https://img.shields.io/crates/v/fast-rich.svg)](https://crates.io/crates/fast-rich)
[![Documentation](https://docs.rs/fast-rich/badge.svg)](https://docs.rs/fast-rich)
[![License](https://img.shields.io/crates/l/fast-rich.svg)](https://github.com/mohammad-albarham/fast-rich/blob/main/LICENSE)
[![Build Status](https://github.com/mohammad-albarham/fast-rich/workflows/CI/badge.svg)](https://github.com/mohammad-albarham/fast-rich/actions)
![Status](https://img.shields.io/badge/status-under--development-orange)

**High-performance Rust port of Python's [Rich](https://github.com/Textualize/rich) library**

*Beautiful terminal formatting for Rust applications*

</div>

!!! warning "Alpha Status"
    This project is currently in **Alpha**. APIs may change and some features may be incomplete.

---

## What is Fast-Rich?

Fast-Rich brings the power of Python's popular Rich library to Rust, enabling you to create beautiful, colorful terminal output with minimal effort.

![Fast-Rich Hero](assets/hero.gif)

## See it in Action

Below is a demonstration of all library features running in sequence, including tables, panels, progress bars, and live displays:

![Fast-Rich Examples](assets/run_all.gif)

## ✨ Features

<div class="grid cards" markdown>

-   :material-palette:{ .lg .middle } **Rich Text & Styles**

    ---

    Bold, italic, underline, colors (8/256/RGB), and markup syntax
    
    ```rust
    console.print("[bold red]Error:[/] failed");
    ```

-   :material-table:{ .lg .middle } **Tables**

    ---

    Unicode borders, auto-sizing columns, custom styles
    
    ```rust
    table.add_column("Name");
    table.add_row_strs(&["Alice"]);
    ```

-   :material-progress-check:{ .lg .middle } **Progress Bars**

    ---

    Multi-task progress, spinners, ETA, customizable columns
    
    ```rust
    let task = progress.add_task("Loading", Some(100));
    progress.advance(task, 10);
    ```

-   :material-refresh:{ .lg .middle } **Live Display**

    ---

    Flicker-free auto-updating content for dashboards
    
    ```rust
    live.update(panel);
    live.refresh()?;
    ```

-   :material-code-tags:{ .lg .middle } **Syntax Highlighting**

    ---

    Code highlighting with multiple themes via `syntect`
    
    ```rust
    let syntax = Syntax::new(code, "rust");
    console.print_renderable(&syntax);
    ```

-   :material-language-markdown:{ .lg .middle } **Markdown**

    ---

    Render Markdown directly in the terminal
    
    ```rust
    let md = Markdown::new("# Hello");
    console.print_renderable(&md);
    ```

-   :material-file-tree:{ .lg .middle } **Tree Views**

    ---

    Hierarchical data visualization with customizable guides
    
    ```rust
    let mut tree = Tree::new(Text::plain("Root"));
    tree.add("Child");
    ```

-   :material-view-dashboard:{ .lg .middle } **Layouts**

    ---

    Split screens and complex terminal layouts
    
    ```rust
    layout.split_row(vec![left, right]);
    ```

</div>

## Quick Start

Add `fast-rich` to your `Cargo.toml`:

```toml
[dependencies]
fast-rich = "0.2.0"
```

Then create beautiful terminal output:

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    // Styled text with markup
    console.print("[bold red]Hello[/] [blue]World[/]!");

    // Create a table
    let mut table = Table::new();
    table.add_column("Feature");
    table.add_column("Status");
    table.add_row_strs(&["Rich Text", "✅ Ready"]);
    table.add_row_strs(&["Tables", "✅ Ready"]);
    
    console.print_renderable(&table);
}
```

**Output:**

```
Hello World!
╭─────────────┬──────────╮
│ Feature     │ Status   │
├─────────────┼──────────┤
│ Rich Text   │ ✅ Ready │
│ Tables      │ ✅ Ready │
╰─────────────┴──────────╯
```

## Feature Comparison (Still under development!!)

| Feature | Python Rich | Fast-Rich |
|:--------|:-----------:|:---------:|
| Rich Text & Styles | ✅ | ✅ |
| Tables | ✅ | ✅ |
| Progress Bars | ✅ | ✅ |
| Live Display | ✅ | ✅ |
| Syntax Highlighting | ✅ | ✅ |
| Markdown | ✅ | ✅ |
| Tree Views | ✅ | ✅ |
| Layouts | ✅ | ✅ |
| Tracebacks | ✅ | ✅ |
| Logging Handler | ✅ | ✅ |

## Next Steps

<div class="grid cards" markdown>

-   :material-rocket-launch:{ .lg .middle } **[Getting Started](getting-started.md)**

    ---

    Installation and first steps

-   :material-book-open-variant:{ .lg .middle } **[Guides](guides/index.md)**

    ---

    Detailed feature documentation

-   :material-code-braces:{ .lg .middle } **[Examples](rust_examples.md)**

    ---

    Runnable example programs

-   :material-api:{ .lg .middle } **[API Reference](reference/api.md)**

    ---

    Full API documentation

</div>
