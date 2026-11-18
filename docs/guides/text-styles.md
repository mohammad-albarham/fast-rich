# Text & Styles

Fast-Rich provides a powerful text styling system with support for colors, attributes, and a convenient markup syntax.

## Quick Example

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    console.print("[bold red]Error:[/] Something went wrong");
    console.print("[italic blue]Hint:[/] Check your configuration");
}
```

---

## Color System

Fast-Rich supports multiple color modes that adapt to terminal capabilities:

### Standard Colors (8 Colors)

The basic colors available on all terminals:

```rust
console.print("[black]black[/] [red]red[/] [green]green[/] [yellow]yellow[/]");
console.print("[blue]blue[/] [magenta]magenta[/] [cyan]cyan[/] [white]white[/]");
```

### Bright Colors (16 Colors)

Brighter variants available on most terminals:

```rust
console.print("[bright_red]bright_red[/] [bright_green]bright_green[/]");
console.print("[bright_blue]bright_blue[/] [bright_magenta]bright_magenta[/]");
```

### RGB/Truecolor (16 Million Colors)

For terminals with truecolor support:

```rust
console.print("[color(255,100,0)]Orange[/]");
console.print("[color(0,255,128)]Spring Green[/]");
console.print("[color(128,0,255)]Purple[/]");
```

### Background Colors

Use `on` to set background colors:

```rust
console.print("[white on blue] White text on blue background [/]");
console.print("[black on yellow] Warning style [/]");
console.print("[white on color(200,0,0)] Custom red background [/]");
```

---

## Style Attributes

Apply formatting effects to text:

| Attribute | Syntax | Description |
|:----------|:-------|:------------|
| Bold | `[bold]` | **Bold text** |
| Italic | `[italic]` | *Italic text* |
| Underline | `[underline]` | <u>Underlined text</u> |
| Strike | `[strike]` | ~~Strikethrough~~ |
| Dim | `[dim]` | Dimmed/faded text |
| Reverse | `[reverse]` | Swap foreground/background |
| Blink | `[blink]` | Blinking text (limited support) |

```rust
console.print("[bold]Bold[/] [italic]Italic[/] [underline]Underline[/]");
console.print("[strike]Strike[/] [dim]Dim[/] [reverse]Reverse[/]");
```

### Combining Styles

Combine multiple styles in one tag:

```rust
console.print("[bold italic red]Bold italic red text[/]");
console.print("[underline blue on white]Underlined blue on white[/]");
```

---

## Markup Syntax

### Basic Tags

Wrap text in `[style]...[/]` to apply styles:

```rust
console.print("[bold]This is bold[/]");
console.print("[red]This is red[/]");
```

### Nesting Styles

Styles can be nested:

```rust
console.print("[bold]Bold [italic]and italic[/italic] just bold[/]");
console.print("[red]Red [blue]with blue inside[/blue] back to red[/]");
```

### Negating Styles

Use `no-` prefix to remove a style:

```rust
console.print("[bold]Bold [no-bold]not bold[/no-bold] bold again[/]");
console.print("[red]Red [no-red]default color[/no-red] red again[/]");
```

### Escaping Brackets

To display literal brackets, use backslash:

```rust
console.print("Use \\[bold] to show [bold] literally");
// Output: Use [bold] to show [bold] literally
```

---

## The Style Struct

For programmatic styling, use the `Style` struct:

```rust
use fast_rich::prelude::*;

let style = Style::new()
    .foreground(Color::Red)
    .background(Color::White)
    .bold()
    .underline();

let text = Text::styled("Styled text", style);
console.print_renderable(&text);
```

### Style Builder Methods

| Method | Description |
|:-------|:------------|
| `.foreground(Color)` | Set text color |
| `.background(Color)` | Set background color |
| `.bold()` | Make text bold |
| `.italic()` | Make text italic |
| `.underline()` | Add underline |
| `.strike()` | Add strikethrough |
| `.dim()` | Make text dim |
| `.reverse()` | Swap fg/bg colors |

### Color Enum

```rust
use fast_rich::Color;

// Named colors
let red = Color::Red;
let bright_blue = Color::BrightBlue;

// RGB colors
let orange = Color::Rgb(255, 128, 0);

// 256-color palette
let color_123 = Color::Ansi256(123);
```

---

## Text Alignment

Align text within a specified width:

```rust
use fast_rich::prelude::*;

let left = Text::from("Left").alignment(Alignment::Left);
let center = Text::from("Center").alignment(Alignment::Center);
let right = Text::from("Right").alignment(Alignment::Right);

console.print_renderable(&left);
console.print_renderable(&center);
console.print_renderable(&right);
```

---

## Working with Text

`Text` has the same editing methods as Python Rich's `Text`. Offsets are
**byte offsets** (what `str::find` returns); an offset inside a multibyte
character snaps down to its start. Widths are terminal cells.

### Styling ranges

```rust
use fast_rich::prelude::*;

let mut text = Text::plain("Error: disk full on /dev/sda1");
text.stylize(Style::new().bold().foreground(Color::Red), 0, 5); // on top
text.stylize_before(Style::new().italic(), 0, usize::MAX);     // underneath
text.highlight_words(["disk"], Style::new().underline(), false);

// Ranges you found yourself (no regex needed):
let plain = text.plain_text();
let ranges = plain.match_indices("/dev").map(|(i, m)| i..i + m.len());
text.highlight_matches(ranges.collect::<Vec<_>>(), Style::new().foreground(Color::Cyan));

assert!(text.get_style_at_offset(0).bold);
```

With the `regex` feature: `text.highlight_regex(&Regex::new(r"\d+")?, style)`.

### Building and joining

```rust
use fast_rich::prelude::*;

let mut line = Text::assemble([Text::from(Span::styled("key", Style::new().bold())), Text::from("=")]);
line.append_tokens([("42", Style::new().foreground(Color::Cyan))]);
line.append_text(&Text::plain(" (ok)"));

let list = Text::plain(", ").join([Text::plain("a"), Text::plain("b")]);
assert_eq!(list.plain_text(), "a, b");
```

### Cutting, cropping and padding

| Method | Does |
|:-------|:-----|
| `divide(offsets)` | Cut at byte offsets into `offsets.len() + 1` pieces |
| `split(sep, include_separator)` | Split on a separator, keeping styles |
| `fit(width)` | Split into lines, each exactly `width` cells |
| `truncate(width, overflow, pad)` | Crop to `width` cells (`Overflow::Ellipsis` adds `…`) |
| `set_length(width)` | Crop or pad with spaces to exactly `width` cells |
| `right_crop(n)` | Drop the last `n` characters |
| `pad(n, c)`, `pad_left`, `pad_right` | Add `n` copies of `c` |
| `rstrip()`, `rstrip_end(width)` | Drop trailing whitespace (all, or only beyond `width`) |
| `remove_suffix(s)` | Drop `s` from the end if present |

```rust
use fast_rich::prelude::*;
use fast_rich::text::Overflow;

let mut name = Text::plain("a_very_long_file_name.rs");
name.truncate(10, Some(Overflow::Ellipsis), false);
assert_eq!(name.plain_text(), "a_very_lo…");
```

### Code helpers

```rust
use fast_rich::prelude::*;

let mut code = Text::plain("fn main() {\n\tprintln!(\"hi\");\n}");
code.expand_tabs(4);
assert_eq!(code.detect_indentation(), 4);
let guided = code.with_indent_guides(None, '│', Style::new().dim());
console.print_renderable(&guided);
```

Differences from Rich: `set_length` and `fit` count cells (Rich counts
characters, so CJK text does not line up there); `right_crop(0)` and
`remove_suffix("")` do nothing (Rich empties the text); `highlight_regex`
does not turn named groups into theme styles.

---

## Real Terminal Output

!!! example "Run the styles demo"

    **Command:**
    ```bash
    cargo run --example styles_demo
    ```

    **What you'll see:**

    ![Styles Demo](../assets/styles.png)
