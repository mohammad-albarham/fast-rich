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

### Automatic Highlighting

Like Python Rich, `print()` / `println()` color common patterns in plain
strings automatically: numbers, `'strings'`, `True`/`False`/`None` (and Rust's
`true`/`false`), URLs, file paths, UUIDs, IPv4/IPv6/MAC addresses,
`key=value` pairs, `<tags>`, calls like `Some(` and brackets.

```rust
console.println("GET https://api.example.com/v1 -> 404 in 3.5s, retry=true");
```

The token rules are the same as Rich's `ReprHighlighter`, checked
character by character against Rich 15 output (`tests/highlight_parity.rs`).

- **Markup wins**: `"[red]42[/]"` is red (and bold, from the number style).
- **Turn it off**: `Console::new().highlight(false)`. The `print_raw` /
  `println_raw` methods and macros, and renderables such as `Text` or
  `Table`, are never highlighted.
- **Restyle it**: override the `repr.*` theme styles, e.g.
  `theme.add_style("repr.number", Style::parse("bold yellow"))`.
- **Replace it**: `Console::new().highlighter(my_highlighter)` with any type
  implementing `fast_rich::highlighter::Highlighter`.
- **Exact Rich behavior**:
  `.highlighter(ReprHighlighter::new().rich_compatible())` skips Rust's
  lowercase `true`/`false`.
- **Inspect the result**: `console.render_str("...")` returns the styled
  `Text` that `print` would write.

### `print_renderable()` - Complex Objects

For tables, panels, trees, and other renderables:

```rust
let mut table = Table::new();
table.add_column("Name");
table.add_row_strs(&["Fast-Rich"]);

console.print_renderable(&table);
```

---

## Pretty Printing

`print_debug` lays out a value's `Debug` output the way Rich's
`rich.pretty` does: a container stays on one line if it fits the console
width, otherwise it is expanded one item per line and the same check runs
again for each item. The result is highlighted like other printed values.

```rust
use fast_rich::prelude::*;
use std::collections::BTreeMap;

let console = Console::new();
let mut stock = BTreeMap::new();
stock.insert("fruit", vec![("apple", 3), ("banana", 12), ("cherry", 40)]);
stock.insert("veg", vec![("leek", 1)]);
console.print_debug(&stock);
```

At 30 columns this prints:

```text
{
    "fruit": [
        ("apple", 3),
        ("banana", 12),
        ("cherry", 40)
    ],
    "veg": [("leek", 1)]
}
```

For more control, use the `Pretty` renderable, `pretty_repr` (a plain
`String`) or `pprint` (print to stdout):

```rust
use fast_rich::prelude::*;

let console = Console::new();
let numbers: Vec<u32> = (1..=100).collect();

// [1, 2, 3, 4, 5, ... +95]
console.print_renderable(&Pretty::new(&numbers).max_length(5));

// ["Lorem ipsum"+44]
let quote = vec!["Lorem ipsum dolor sit amet, consectetur adipiscing elit"];
console.print_renderable(&Pretty::new(&quote).max_string(11));

// Indent guides (dim green │), every container expanded
console.print_renderable(&Pretty::new(&numbers[..3]).indent_guides(true).expand_all(true));

// A String, laid out for 20 cells
let s = pretty_repr(&vec![(1, "one"), (2, "two")], &PrettyOptions::new().max_width(20));
assert_eq!(s, "[\n    (1, \"one\"),\n    (2, \"two\")\n]");
```

| Option | Effect |
|:-------|:-------|
| `max_width` | Width to fit (`pretty_repr` only; `Pretty` uses the render width) |
| `indent_size` | Spaces per level (default 4) |
| `max_length` | Show the first N items of a list, tuple, map or set, then `... +K` |
| `max_string` | Crop longer strings: `"abc"+5` |
| `max_depth` | Containers at this depth or deeper become `[...]` / `Name { ... }` |
| `expand_all` | Expand every container, even ones that fit |
| `indent_guides` | Draw `│` guides in the `repr.indent` style |

### Exact structure with serde or valuable

`Debug` output is text, so a hand-written `Debug` impl can hide a value's
structure (it is then printed as one piece). With the `serde` or
`valuable` feature, `Pretty` can read the real structure instead:

```toml
fast-rich = { version = "0.4", features = ["serde"] }   # or "valuable"
```

```rust
use fast_rich::prelude::*;

#[derive(serde::Serialize)]          // or #[derive(valuable::Valuable)]
struct Matrix { rows: Vec<Vec<i32>> }

let m = Matrix { rows: vec![vec![1, 2, 3], vec![4, 5, 6]] };
let console = Console::new();
console.print_renderable(&Pretty::from_serialize(&m));   // or from_valuable
let s = pretty_repr_serialize(&m, &PrettyOptions::new().max_width(20));
```

| | `Pretty::new` (Debug) | `from_serialize` (serde) | `from_valuable` (valuable) |
|:--|:--|:--|:--|
| Works with | any `Debug` type | `Serialize` types | `Valuable` types |
| Custom `Debug` impls | printed as text | real fields | real fields |
| `Option` | `Some(x)` / `None` | `Some(x)` / `None` | `x` / `None` |
| Sets | `{1, 2}` | `[1, 2]` | `[1, 2]` |
| Renames | — | `#[serde(rename)]`, `skip` apply | — |

For derived types (apart from those rows) all three print the same.

Differences from Rich, since the input is Rust `Debug` output: strings use
`"` quotes, booleans are `true`/`false`, structs keep Rust syntax
(`Point { x: 1, y: 2 }`), and enum variants such as `Some(1)` are laid out
like Rich lays out a namedtuple. Output the parser does not recognize
(custom `Debug` impls such as `1.5s`) is printed verbatim.

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

## Working with Rendered Lines (Segments)

Rich's `Segment` helpers are associated functions on
`fast_rich::renderable::Segment`. They work on spans (`Span` is what Rich
calls a segment) and on lines (`Vec<Span>`), for example the lines from
`renderable::render_lines`:

```rust
use fast_rich::prelude::*;
use fast_rich::renderable::Segment;
use fast_rich::text::Span;

let spans = vec![Span::styled("Hello\nWorld, this is long", Style::new().bold())];
// Split at newlines, crop or pad every line to 8 cells.
let lines = Segment::split_and_crop_lines(&spans, 8, Style::new(), true, false);
// Make it a 8x4 block with blank lines centered around the text.
let block = Segment::align_middle(&lines, 8, 4, Style::new(), false);
assert_eq!(Segment::get_shape(&block), (8, 4));
// Cut a line into columns at cell positions.
let columns = Segment::divide(&lines[1], [3, 8]);
assert_eq!(Segment::plain(&columns[0]), "Wor");
```

Also available: `split_lines`, `adjust_line_length`, `get_line_length`,
`set_shape`, `align_top` / `align_bottom`, `simplify`, `strip_links`,
`strip_styles`, `remove_color`, `apply_style`, `filter_control`, and on
`Span`: `cell_length`, `split_cells` and `is_control`. See
`examples/segment_demo.rs`.

## Control Codes

`fast_rich::control::Control` writes terminal control codes, with the
same escape strings as Rich's `Control`:

```rust
use fast_rich::control::Control;

console.print_renderable(&Control::title("Building..."));
console.print_renderable(&Control::show_cursor(false));
console.print_renderable(&Control::move_to(0, 0));
console.print_renderable(&Control::show_cursor(true));
```

Constructors: `bell`, `home`, `clear`, `move_cursor(x, y)` (Rich's
`move`), `move_to_column`, `move_to`, `show_cursor`, `alt_screen`,
`title`, or `Control::new` with any `ControlCode`s. A `Control` renders as
one span that the segment helpers treat as a control span: it takes no
cells and is never cropped.

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
