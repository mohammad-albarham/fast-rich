# Console

The `Console` is your primary interface for all terminal output in Fast-Rich. It handles color detection, terminal sizing, and rendering of styled content.

## Creating a Console

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    console.print("[bold green]Ready![/]");
}
```

---

## Basic Output Methods

### `print()` - Styled Output

Prints text with markup processing, no trailing newline:

```rust
console.print("[bold]Hello[/] ");
console.print("[blue]World[/]");
// Output: Hello World (on same line)
```

### `println()` - Styled Output with Newline

Same as `print()` but adds a newline:

```rust
console.println("[green]Line 1[/]");
console.println("[blue]Line 2[/]");
```

### `print_renderable()` - Complex Objects

For tables, panels, trees, and other renderables:

```rust
let mut table = Table::new();
table.add_column("Name");
table.add_row_strs(&["Fast-Rich"]);

console.print_renderable(&table);
```

---

## Decorative Methods

### `rule()` - Horizontal Divider

Creates a decorative horizontal line:

```rust
console.rule("[bold]Section Title[/]");
```

**Output:**
```
───────────────────── Section Title ──────────────────────
```

Customize with plain text or leave empty:

```rust
console.rule("");  // Plain line
console.rule("[red]Chapter 1[/]");  // Styled title
```

### `newline()` - Blank Line

Adds a blank line:

```rust
console.print("[bold]Before[/]");
console.newline();
console.print("[bold]After[/]");
```

---

## Console Configuration

### Force Color Output

By default, Fast-Rich detects if the terminal supports colors. Override this behavior:

```rust
// Force colors on (useful for CI/piped output)
let console = Console::new().force_color(true);

// Force colors off
let console = Console::new().force_color(false);
```

### Terminal Width

Get the current terminal width:

```rust
let console = Console::new();
let width = console.get_width();
println!("Terminal is {} columns wide", width);
```

---

## Capture Mode (Testing)

For testing, capture output instead of printing to stdout:

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::capture();
    
    console.print("[bold]Hello[/]");
    console.println(" World");
    
    let output = console.get_captured_output();
    assert!(output.contains("Hello"));
    assert!(output.contains("World"));
}
```

This is useful for:

- Unit testing styled output
- Generating output for export
- Comparing expected vs. actual rendering

---

## Real Terminal Output

**Command:**
```bash
cargo run --example console_print
```

**Output:**
```
Console API Demo
This is a regular print.

This is a println which adds a newline.

─────── Padding ───────
Explicit Renderable

Emoji: 🐍🦀
```

---

## Best Practices

!!! tip "Use Prelude Import"
    The prelude re-exports everything you need:
    ```rust
    use fast_rich::prelude::*;
    ```

!!! tip "Reuse Console Instance"
    Create one `Console` and reuse it throughout your application:
    ```rust
    struct App {
        console: Console,
    }
    
    impl App {
        fn new() -> Self {
            Self { console: Console::new() }
        }
    }
    ```

!!! warning "Thread Safety"
    `Console` is not `Send` or `Sync`. For multi-threaded applications, 
    create a new `Console` in each thread or protect with appropriate synchronization.
