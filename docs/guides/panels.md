# Panels & Rules

Panels and rules help organize and visually separate content in your terminal output.

## Panels

Panels wrap content in a decorative border with an optional title.

### Basic Panel

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    let panel = Panel::new(Text::plain("Hello, World!"));
    console.print_renderable(&panel);
}
```

**Output:**
```
╭────────────────────────────────────────────────────────────────╮
│ Hello, World!                                                  │
╰────────────────────────────────────────────────────────────────╯
```

### Panel with Title

```rust
let panel = Panel::new(Text::plain("Content goes here"))
    .title("My Title");

console.print_renderable(&panel);
```

**Output:**
```
╭── My Title ────────────────────────────────────────────────────╮
│ Content goes here                                              │
╰────────────────────────────────────────────────────────────────╯
```

### Styled Panels

Apply colors to panel borders:

```rust
let panel = Panel::new(fast_rich::markup::parse("[bold]Important info[/]"))
    .title("Notice")
    .border_style(BorderStyle::Rounded)
    .style(Style::new().foreground(Color::Blue));

console.print_renderable(&panel);
```

---

## Border Styles

Panels support the same border styles as tables:

=== "Rounded"

    ```rust
    Panel::new(text).border_style(BorderStyle::Rounded)
    ```
    ```
    ╭── Title ───╮
    │ Content    │
    ╰────────────╯
    ```

=== "Square"

    ```rust
    Panel::new(text).border_style(BorderStyle::Square)
    ```
    ```
    ┌── Title ───┐
    │ Content    │
    └────────────┘
    ```

=== "Heavy"

    ```rust
    Panel::new(text).border_style(BorderStyle::Heavy)
    ```
    ```
    ┏━━ Title ━━━┓
    ┃ Content    ┃
    ┗━━━━━━━━━━━━┛
    ```

=== "Double"

    ```rust
    Panel::new(text).border_style(BorderStyle::Double)
    ```
    ```
    ╔══ Title ═══╗
    ║ Content    ║
    ╚════════════╝
    ```

---

## Rules

Rules are horizontal lines that can be used as dividers:

### Basic Rule

```rust
console.rule("");
```

**Output:**
```
────────────────────────────────────────────────────────────────
```

### Rule with Title

```rust
console.rule("[bold]Section Title[/]");
```

**Output:**
```
───────────────────── Section Title ──────────────────────────
```

### Styled Rules

The title can include any markup:

```rust
console.rule("[bold red]⚠️ Warning[/]");
console.rule("[blue]═══ Chapter 1 ═══[/]");
```

---

## Combining Panels and Content

Panels can contain any Renderable:

### Panel with Table

```rust
let mut table = Table::new();
table.add_column("Key");
table.add_column("Value");
table.add_row_strs(&["Name", "Fast-Rich"]);
table.add_row_strs(&["Version", "0.2.0"]);

let panel = Panel::new(table)
    .title("Package Info")
    .border_style(BorderStyle::Rounded);

console.print_renderable(&panel);
```

### Multi-line Content

```rust
let content = "Line 1\nLine 2\nLine 3";
let panel = Panel::new(Text::plain(content));
console.print_renderable(&panel);
```

---

## Real Terminal Output

**Command:**
```bash
cargo run --example panel
```

**Output:**
```
───────────────────────── Panel Demo ─────────────────────────

1. Basic Panel
╭────────────────────────────────────────────────────────────╮
│ This is a simple panel with no title.                      │
╰────────────────────────────────────────────────────────────╯

2. Panel with Title
╭── My Panel ────────────────────────────────────────────────╮
│ Content inside a titled panel.                             │
╰────────────────────────────────────────────────────────────╯

3. Styled Panel
╭── Important ───────────────────────────────────────────────╮
│ This panel has a blue border!                              │
╰────────────────────────────────────────────────────────────╯
```

---

## Tips

!!! tip "Use Panels for Emphasis"
    Wrap important information in panels to make it stand out:
    ```rust
    let error = Panel::new(Text::styled("Error: File not found", 
        Style::new().foreground(Color::Red)))
        .title("Error")
        .border_style(BorderStyle::Heavy);
    ```

!!! tip "Combine with Rules"
    Use rules to separate logical sections of output:
    ```rust
    console.rule("[bold]Configuration[/]");
    console.print_renderable(&config_table);
    console.rule("[bold]Status[/]");
    console.print_renderable(&status_panel);
    ```
