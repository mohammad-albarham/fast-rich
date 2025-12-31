# Layouts

Create complex terminal UIs by splitting the screen into rows and columns with the `Layout` component.

## Quick Example

```rust
use fast_rich::prelude::*;
use fast_rich::layout::Layout;

fn main() {
    let console = Console::new();
    
    let mut layout = Layout::new();
    layout.split_row(vec![
        Layout::new().with_name("left"),
        Layout::new().with_name("right"),
    ]);
    
    layout.children_mut()[0].update(Panel::new(Text::plain("Left")));
    layout.children_mut()[1].update(Panel::new(Text::plain("Right")));
    
    console.print_renderable(&layout);
}
```

---

## Creating Layouts

### Root Layout

```rust
use fast_rich::layout::Layout;

let layout = Layout::new();
```

### Naming Layouts

Names help identify layout regions:

```rust
let layout = Layout::new().with_name("main");
```

---

## Splitting Layouts

### Horizontal Split (Rows)

Split into side-by-side columns:

```rust
let mut layout = Layout::new();
layout.split_row(vec![
    Layout::new().with_name("left"),
    Layout::new().with_name("right"),
]);
```

### Vertical Split (Columns)

Stack layouts top-to-bottom:

```rust
let mut layout = Layout::new();
layout.split_column(vec![
    Layout::new().with_name("header"),
    Layout::new().with_name("body"),
    Layout::new().with_name("footer"),
]);
```

---

## Size Control

### Ratio-Based Sizing

Divide space proportionally:

```rust
layout.split_row(vec![
    Layout::new().with_name("sidebar").with_ratio(1),  // 1/4 of width
    Layout::new().with_name("content").with_ratio(3),  // 3/4 of width
]);
```

### Fixed Sizing

Set exact sizes (in lines/columns):

```rust
layout.split_column(vec![
    Layout::new().with_name("header").with_size(3),   // Exactly 3 lines
    Layout::new().with_name("body").with_ratio(1),    // Remaining space
    Layout::new().with_name("footer").with_size(1),   // Exactly 1 line
]);
```

---

## Nesting Layouts

Create complex UIs by nesting splits:

```rust
let mut root = Layout::new().with_name("root");

// Root vertical split: header, main, footer
root.split_column(vec![
    Layout::new().with_name("header").with_size(3),
    Layout::new().with_name("main").with_ratio(1),
    Layout::new().with_name("footer").with_size(1),
]);

// Split "main" horizontally: sidebar, content
root.children_mut()[1].split_row(vec![
    Layout::new().with_name("sidebar").with_ratio(1),
    Layout::new().with_name("content").with_ratio(3),
]);
```

This creates:
```
┌─────────────────────────────────────────────────────┐
│                      Header                         │
├───────────┬─────────────────────────────────────────┤
│           │                                         │
│  Sidebar  │              Content                    │
│           │                                         │
├───────────┴─────────────────────────────────────────┤
│                      Footer                         │
└─────────────────────────────────────────────────────┘
```

---

## Adding Content

Update layout regions with renderables:

```rust
// Access children by index
layout.children_mut()[0].update(header_panel);
layout.children_mut()[1].update(body_table);
layout.children_mut()[2].update(footer_text);
```

For nested layouts:

```rust
// main (index 1) -> sidebar (index 0)
layout.children_mut()[1].children_mut()[0].update(sidebar);

// main (index 1) -> content (index 1)
layout.children_mut()[1].children_mut()[1].update(content);
```

---

## Dashboard Example

```rust
use fast_rich::prelude::*;
use fast_rich::layout::Layout;

fn main() {
    let console = Console::new();
    
    // Create layout structure
    let mut layout = Layout::new();
    layout.split_column(vec![
        Layout::new().with_name("header").with_size(3),
        Layout::new().with_name("body").with_ratio(1),
        Layout::new().with_name("footer").with_size(1),
    ]);
    
    // Header
    let header = Panel::new(
        Text::styled("Dashboard", Style::new().bold())
            .alignment(Alignment::Center)
    ).border_style(BorderStyle::Heavy);
    
    // Body with sidebar
    layout.children_mut()[1].split_row(vec![
        Layout::new().with_name("menu").with_ratio(1),
        Layout::new().with_name("content").with_ratio(3),
    ]);
    
    let menu = Panel::new(Text::plain("Menu\n━━━━\nHome\nStats\nConfig"));
    
    let mut stats = Table::new();
    stats.add_column("Metric");
    stats.add_column("Value");
    stats.add_row_strs(&["CPU", "45%"]);
    stats.add_row_strs(&["Memory", "2.1 GB"]);
    
    let content = Panel::new(stats).title("Statistics");
    
    // Footer
    let footer = Text::from("Press Ctrl+C to exit")
        .alignment(Alignment::Center)
        .style(Style::new().dim());
    
    // Assemble
    layout.children_mut()[0].update(header);
    layout.children_mut()[1].children_mut()[0].update(menu);
    layout.children_mut()[1].children_mut()[1].update(content);
    layout.children_mut()[2].update(footer);
    
    console.print_renderable(&layout);
}
```

---

## With Live Display

Combine layouts with `Live` for animated dashboards:

```rust
use fast_rich::live::Live;

let mut live = Live::new();
live.start().unwrap();

loop {
    let layout = build_dashboard(current_data);
    live.update(layout);
    live.refresh().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

See the [Live Display guide](live.md) and the [Dashboard Tutorial](../tutorial_dashboard.md) for complete examples.

---

## Real Terminal Output

**Command:**
```bash
cargo run --example layout_demo
```

**Output:**
```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                        Dashboard                              ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
╭─────────────╮╭── Statistics ──────────────────────────────────╮
│ Menu        ││ ╭────────┬───────╮                             │
│ ━━━━        ││ │ Metric │ Value │                             │
│ Home        ││ ├────────┼───────┤                             │
│ Stats       ││ │ CPU    │ 45%   │                             │
│ Config      ││ │ Memory │ 2.1GB │                             │
│             ││ ╰────────┴───────╯                             │
╰─────────────╯╰────────────────────────────────────────────────╯
              Press Ctrl+C to exit
```

---

## Tips

!!! tip "Use Names for Clarity"
    Always name your layouts for maintainability:
    ```rust
    Layout::new().with_name("sidebar")
    ```

!!! tip "Responsive Design"
    Use ratios instead of fixed sizes when possible. The layout 
    will adapt to different terminal widths.

!!! warning "Clone for Live"
    When using `Live::update()` with layouts, clone the layout:
    ```rust
    live.update(layout.clone());
    ```
