# Syntax Highlighting

Highlight code snippets in the terminal with beautiful syntax coloring.

!!! note "Feature Flag Required"
    Syntax highlighting requires the `syntax` feature:
    ```toml
    fast-rich = { version = "0.2.0", features = ["syntax"] }
    ```

## Quick Example

```rust
use fast_rich::prelude::*;
use fast_rich::syntax::Syntax;

fn main() {
    let console = Console::new();
    
    let code = r#"fn main() {
    println!("Hello, World!");
}"#;
    
    let syntax = Syntax::new(code, "rust");
    console.print_renderable(&syntax);
}
```

---

## Creating Syntax Objects

### Basic Usage

```rust
use fast_rich::syntax::Syntax;

let json = r#"{"name": "fast-rich", "version": "0.2.0"}"#;
let syntax = Syntax::new(json, "json");
```

### With Line Numbers

```rust
let code = r#"def hello():
    print("Hello!")

hello()"#;

let syntax = Syntax::new(code, "python")
    .line_numbers(true);
```

---

## Supported Languages

Fast-Rich uses `syntect` for highlighting. Common languages include:

| Language | Identifier |
|:---------|:-----------|
| Rust | `rust`, `rs` |
| Python | `python`, `py` |
| JavaScript | `javascript`, `js` |
| TypeScript | `typescript`, `ts` |
| JSON | `json` |
| YAML | `yaml` |
| TOML | `toml` |
| HTML | `html` |
| CSS | `css` |
| SQL | `sql` |
| Bash | `bash`, `sh` |
| Go | `go` |
| C | `c` |
| C++ | `cpp`, `c++` |

---

## Themes

Syntax supports multiple color themes:

```rust
use fast_rich::syntax::{Syntax, SyntaxTheme};

let syntax = Syntax::new(code, "rust")
    .theme(SyntaxTheme::Monokai);
```

### Available Themes

| Theme | Description |
|:------|:------------|
| `Monokai` | Dark theme with vibrant colors |
| `SolarizedDark` | Solarized dark variant |
| `SolarizedLight` | Solarized light variant |
| `Base16OceanDark` | Ocean dark theme |
| `InspiredGithub` | GitHub-inspired colors |

---

## Display in Panel

Wrap syntax in a panel for a polished look:

```rust
let code = r#"{
    "name": "fast-rich",
    "version": "0.2.0",
    "features": ["syntax", "markdown"]
}"#;

let syntax = Syntax::new(code, "json")
    .line_numbers(true);

let panel = Panel::new(syntax)
    .title("package.json")
    .border_style(BorderStyle::Rounded);

console.print_renderable(&panel);
```

---

## Real Terminal Output

**Command:**
```bash
cargo run --example syntax_highlighting --features syntax
```

**Output:**
```
───────────────────── Syntax Highlighting ─────────────────────

1. Rust Code
╭── main.rs ─────────────────────────────────────────────────╮
│   1 │ fn main() {                                          │
│   2 │     let message = "Hello, Fast-Rich!";               │
│   3 │     println!("{}", message);                         │
│   4 │ }                                                    │
╰────────────────────────────────────────────────────────────╯

2. JSON Data
╭── json ────────────────────────────────────────────────────╮
│   1 │ {"name": "fast_rich", "speed": "blazing"}            │
╰────────────────────────────────────────────────────────────╯

3. Python Code
╭── script.py ───────────────────────────────────────────────╮
│   1 │ def greet(name):                                     │
│   2 │     return f"Hello, {name}!"                         │
│   3 │                                                      │
│   4 │ print(greet("World"))                                │
╰────────────────────────────────────────────────────────────╯
```

!!! note
    Colors in the actual terminal output vary based on the theme selected.

---

## Tips

!!! tip "Language Detection"
    If you're unsure of the language, `syntect` often auto-detects from content:
    ```rust
    // At minimum, provide a reasonable guess
    let syntax = Syntax::new(code, "rust");
    ```

!!! tip "Raw Strings"
    Use Rust raw strings for code with special characters:
    ```rust
    let code = r#"let x = "quoted";"#;
    ```

!!! warning "Performance"
    Syntax highlighting can be slow for very large files. Consider 
    limiting the displayed code to relevant snippets.
