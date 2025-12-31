# Rust Examples

This page demonstrates the capabilities of the `fast-rich` library with runnable examples.

## How to Run

You can run any example using `cargo`:

```bash
cargo run --example <example_name>
# For examples requiring features (like markdown/syntax):
cargo run --example <example_name> --features full
```

---

## 1. Hello World (`hello`)

The simplest example confirming the library is working.

**Command:**
```bash
cargo run --example hello
```

**Output:**
```
Hello, World!
```

---

## 2. Console Printing (`console_print`)

Demonstrates various `Console` methods for printing and features like padding and emoji.

**Command:**
```bash
cargo run --example console_print
```

**Output:**
```text
Console API Demo
This is a regular print.

This is a println which adds a newline.

─────── Padding ───────
Explicit Renderable

Emoji: 🐍🦀
```

---

## 3. Styles & Colors (`styles_demo`)

Showcases the rich styling system, including 8 standard colors, bright variants, attributes (bold, italic, etc.), text alignment, and Truecolor support.

**Command:**
```bash
cargo run --example styles_demo
```

**Output:**
```text
Text & Styles Demo

Standard Colors:
black red green yellow blue magenta cyan white
bright_black bright_red bright_green bright_yellow bright_blue bright_magenta bright_cyan bright_white

Attributes:
bold italic underline dim reverse strike blink hidden

Nested & Parsing:
Current style: bold red underline no-underline no-red no-bold
Escaped tags: [bold] is not bold

Text Alignment (20 chars):
|       Center       |
|Left                |
|               Right|

Truecolor (RGB):
RGB Gradient Step 0..9

Backgrounds:
CRITICAL WARNING SUCCESS INFO
```

---

## 4. Tables (`tables_demo`)

Demonstrates the `Table` component with automatic column sizing, borders, titles, and cell styling.

**Command:**
```bash
cargo run --example tables_demo
```

**Output:**
```text
Table Features Demo

1. Basic Table with Styling
  Star Wars Movies
╭───┬────────────┬─────────────╮
│ # │ Title      │ Director    │
├───┼────────────┼─────────────┤
│ I │ A New Hope │ George Lucas│
│ V │ The Empire │ Irvin Kersh │
│ VI│ Return of  │ Richard Mar │
╰───┴────────────┴─────────────╯

(Tables support various border styles like ASCII, Double, Heavy, etc.)
```

---

## 5. Panels & Layouts (`panel_layout`)

Showcases `Panel` for framing content and simple `Layout` structures.

**Command:**
```bash
cargo run --example panel_layout
```

**Output:**
```text
Panels & Layout Demo

1. Panel Variations
╭── Title ────────────╮
│ Simple Panel        │
╰─────────────────────╯

2. Layout (Splits)
(Demonstrates stacking panels vertically and horizontally)
```

---

## 6. Tree View (`tree_view`)

Visualizes hierarchical data like file systems using `Tree`.

**Command:**
```bash
cargo run --example tree_view
```

**Output:**
```text
Tree View Demo

project_root/
├── src/
│   ├── core/
│   │   ├── lib.rs
│   │   └── main.rs
│   └── utils.rs
├── assets/
│   └── logo.png
└── Cargo.toml
```

---

## 7. Progress Bars (`progress_bar`)

A multi-step progress bar simulation with styling and ETA.

**Command:**
```bash
cargo run --example progress_bar
```

**Output:**
```text
Progress Bar Demo

Downloading ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% ETA 00:00
Extracting  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% ETA 00:00
Installing  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% ETA 00:00

Done!
```

---

## 8. Markdown & Syntax (`markdown_syntax`)

Renders Markdown and performs syntax highlighting (requires features).

**Command:**
```bash
cargo run --example markdown_syntax --features full
```

**Output:**
```text
Markdown & Syntax Demo

1. Markdown Rendering
Fast Rich Markdown
...

2. Syntax Highlighting
(JSON/Rust code highlighted)
```

---

## 9. Tracebacks (`traceback_demo`)

Pretty-prints error tracebacks for panic handling or `Result` errors.

**Command:**
```bash
cargo run --example traceback_demo
```

**Output:**
```text
Traceback & Error Demo

1. install_panic_hook()
...

2. Formatting Result::Err
╭── Traceback ──────────────────────────────────────────────────────────╮
│ Error: Failed to connect to database at 127.0.0.1:5432                │
╰───────────────────────────────────────────────────────────────────────╯
```

---

## 10. Logging (`logging_demo`)

Demonstrates how to emulate logging levels and inspect structs.

**Command:**
```bash
cargo run --example logging_demo
```

**Output:**
```text
Logging & Inspect Demo

1. Inspect/Debug
Inspecting struct: Config { host: "localhost", port: 8080, active: true }

2. Logging Levels
[12:00:00] [dim blue]DEBUG Connecting to server...
[12:00:00] [green]INFO Connection established.
[12:00:00] [bold yellow]WARN Latency high (150ms).
[12:00:00] [bold red]ERROR Connection dropped!
```

---

## 11. Full Showcase (`showcase`)

A comprehensive tour of all major features in one run.

**Command:**
```bash
cargo run --example showcase --features full
```
