# API Reference

This page provides an overview of the Fast-Rich API. For detailed documentation of every type and function, see the [docs.rs documentation](https://docs.rs/fast-rich).

## Generate Local Documentation

```bash
cargo doc --open --all-features
```

---

## Core Modules

### `console`

The main output interface for Fast-Rich.

| Type | Description |
|:-----|:------------|
| `Console` | Primary output handler with color detection and styling |

```rust
use fast_rich::Console;
let console = Console::new();
console.print("[bold]Hello[/]");
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/console/index.html)

---

### `style`

Colors and text styling.

| Type | Description |
|:-----|:------------|
| `Style` | Text style with colors and attributes |
| `Color` | Color enum (Named, RGB, Ansi256) |

```rust
use fast_rich::{Style, Color};
let style = Style::new().foreground(Color::Red).bold();
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/style/index.html)

---

### `text`

Rich text with spans and alignment.

| Type | Description |
|:-----|:------------|
| `Text` | Styled text with multiple spans |
| `Alignment` | Text alignment (Left, Center, Right) |

```rust
use fast_rich::text::{Text, Alignment};
let text = Text::plain("Hello").alignment(Alignment::Center);
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/text/index.html)

---

### `table`

Data tables with customizable borders.

| Type | Description |
|:-----|:------------|
| `Table` | Table with columns and rows |
| `Column` | Column configuration |
| `ColumnAlign` | Column alignment (Left, Center, Right) |

```rust
use fast_rich::{Table, Column, ColumnAlign};
let mut table = Table::new();
table.add_column("Name");
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/table/index.html)

---

### `panel`

Bordered panels for content organization.

| Type | Description |
|:-----|:------------|
| `Panel` | Content box with border and title |
| `BorderStyle` | Border style (Rounded, Square, Heavy, etc.) |

```rust
use fast_rich::{Panel, BorderStyle};
let panel = Panel::new(text).border_style(BorderStyle::Rounded);
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/panel/index.html)

---

### `progress`

Progress bars and spinners.

| Type | Description |
|:-----|:------------|
| `Progress` | Multi-task progress manager |
| `ProgressBar` | Single progress bar |
| `Spinner` | Animated spinner |
| `Status` | Simple status indicator |

```rust
use fast_rich::progress::{Progress, Spinner};
let progress = Progress::new();
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/progress/index.html)

---

### `live`

Flicker-free live updating display.

| Type | Description |
|:-----|:------------|
| `Live` | Auto-refreshing display manager |

```rust
use fast_rich::live::Live;
let mut live = Live::new();
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/live/index.html)

---

### `layout`

Screen splitting and layouts.

| Type | Description |
|:-----|:------------|
| `Layout` | Nestable screen layout |

```rust
use fast_rich::Layout;
let layout = Layout::new().with_name("main");
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/layout/index.html)

---

### `tree`

Hierarchical data visualization.

| Type | Description |
|:-----|:------------|
| `Tree` | Tree with root node |
| `TreeNode` | Child node for nesting |
| `GuideStyle` | Guide line style |

```rust
use fast_rich::{Tree, TreeNode};
let tree = Tree::new(Text::plain("Root"));
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/tree/index.html)

---

### `rule`

Horizontal divider lines.

| Type | Description |
|:-----|:------------|
| `Rule` | Decorative horizontal line |

```rust
use fast_rich::Rule;
console.rule("[bold]Title[/]");
```

[View on docs.rs](https://docs.rs/fast-rich/latest/fast_rich/rule/index.html)

---

## Optional Modules

These require feature flags.

### `syntax` (requires `syntax` feature)

| Type | Description |
|:-----|:------------|
| `Syntax` | Syntax-highlighted code |
| `SyntaxTheme` | Color theme for highlighting |

```rust
use fast_rich::syntax::Syntax;
let syntax = Syntax::new(code, "rust");
```

---

### `markdown` (requires `markdown` feature)

| Type | Description |
|:-----|:------------|
| `Markdown` | Markdown renderer |

```rust
use fast_rich::markdown::Markdown;
let md = Markdown::new("# Hello");
```

---

### `log` (requires `logging` feature)

| Type | Description |
|:-----|:------------|
| `RichLogger` | Log crate handler |
| `ConsoleLog` | Console-based logging |

```rust
use fast_rich::log::RichLogger;
RichLogger::init().unwrap();
```

---

### `traceback`

| Type | Description |
|:-----|:------------|
| `Traceback` | Pretty error display |
| `TracebackConfig` | Traceback options |
| `install_panic_hook` | Function to install panic handler |

```rust
use fast_rich::traceback::{Traceback, install_panic_hook};
install_panic_hook();
```

---

## The Prelude

Import everything you need with the prelude:

```rust
use fast_rich::prelude::*;
```

The prelude includes:
- `Console`
- `Style`, `Color`
- `Text`, `Alignment`
- `Table`, `Column`, `ColumnAlign`
- `Panel`, `BorderStyle`
- `Rule`
- `Tree`, `TreeNode`, `GuideStyle`
- `Progress`, `ProgressBar`, `Spinner`, `Status`
- `Columns`
- `inspect`, `InspectConfig`
- `ConsoleLog`
- `install_panic_hook`

---

## The Renderable Trait

All displayable types implement `Renderable`:

```rust
pub trait Renderable {
    fn render(&self, context: &RenderContext) -> Vec<String>;
}
```

Use `console.print_renderable(&item)` to display any `Renderable`.

Custom types can implement `Renderable` to integrate with Fast-Rich output.
