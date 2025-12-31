# Markdown Rendering

Render Markdown content directly in the terminal with styled headers, lists, code blocks, and more.

!!! note "Feature Flag Required"
    Markdown rendering requires the `markdown` feature:
    ```toml
    fast-rich = { version = "0.2.0", features = ["markdown"] }
    ```

## Quick Example

```rust
use fast_rich::prelude::*;
use fast_rich::markdown::Markdown;

fn main() {
    let console = Console::new();
    
    let md = Markdown::new("# Hello World\n\nThis is **bold** and *italic* text.");
    console.print_renderable(&md);
}
```

---

## Creating Markdown

```rust
use fast_rich::markdown::Markdown;

let content = r#"
# Main Title

## Introduction

This is a paragraph with **bold** and *italic* text.

- Item 1
- Item 2
- Item 3

`inline code` is also supported.
"#;

let md = Markdown::new(content);
console.print_renderable(&md);
```

---

## Supported Elements

### Headers

```markdown
# Heading 1
## Heading 2
### Heading 3
```

Headers are rendered with decorative underlines:

```
# Heading 1
════════════════════════════════════════════════════════

## Heading 2
────────────────────────────────────────
```

### Text Formatting

| Markdown | Rendered As |
|:---------|:------------|
| `**bold**` | **Bold text** |
| `*italic*` | *Italic text* |
| `` `code` `` | Inline code |

### Lists

Unordered lists:

```markdown
- First item
- Second item
  - Nested item
```

Rendered:

```
• First item
• Second item
  • Nested item
```

### Blockquotes

```markdown
> This is a blockquote.
> It can span multiple lines.
```

### Code Blocks

````markdown
```python
def hello():
    print("Hello!")
```
````

Code blocks render with syntax highlighting if the `syntax` feature is also enabled.

---

## Rendering Options

```rust
let md = Markdown::new(content)
    .code_theme("monokai");  // Set code block theme
```

---

## Display README

Read and display a README file:

```rust
use std::fs;
use fast_rich::prelude::*;
use fast_rich::markdown::Markdown;

fn main() {
    let console = Console::new();
    
    let readme = fs::read_to_string("README.md").expect("Failed to read");
    let md = Markdown::new(&readme);
    
    console.print_renderable(&md);
}
```

---

## Real Terminal Output

**Command:**
```bash
cargo run --example markdown_parity --features markdown
```

**Output:**
```
# Heading 1
════════════════════════════════════════════════════════════

## Heading 2
────────────────────────────────────────

This is a paragraph with bold and italic text.

• List item 1
• List item 2
• List item 3

> This is a blockquote that demonstrates
> how quoted text is rendered.

Here is some `inline code` in a paragraph.
```

---

## With Syntax Highlighting

When both `markdown` and `syntax` features are enabled, fenced code blocks get full syntax highlighting:

```toml
[dependencies]
fast-rich = { version = "0.2.0", features = ["markdown", "syntax"] }
```

````markdown
```rust
fn main() {
    println!("Syntax highlighted!");
}
```
````

The code block above will render with Rust syntax colors.

---

## Tips

!!! tip "Combine Features"
    Enable `full` to get both markdown and syntax highlighting:
    ```toml
    fast-rich = { version = "0.2.0", features = ["full"] }
    ```

!!! tip "Documentation Display"
    Use Markdown rendering to display help text, changelogs, or 
    documentation directly in your CLI.

!!! warning "Terminal Width"
    Very long lines may wrap awkwardly. Consider preprocessing 
    markdown to add appropriate line breaks for terminal display.
