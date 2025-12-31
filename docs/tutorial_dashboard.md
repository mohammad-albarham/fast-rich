# Tutorial: Building a Live Dashboard

This tutorial demonstrates how to build a dynamic, full-screen terminal dashboard using `fast-rich`. We will combine **Layouts**, **Panels**, **Tables**, and **Live Display** to create a monitoring tool interface.

## 1. The Goal

We want to create a UI with:
*   A **Header** (Top)
*   A **Main Area** split into:
    *   **Sidebar** (Left, Menu)
    *   **Body** (Right, Data Table)
*   A **Footer** (Bottom, Status)
*   **Live Updates**: The body content will update automatically.

## 2. Setting up the Layout

First, we define the static structure using `Layout`.

```rust
use rich_rust::prelude::*;

fn create_layout() -> rich_rust::layout::Layout {
    // Root layout (vertical stack)
    let mut layout = rich_rust::layout::Layout::new().with_name("root");
    
    // Split into Header (size 3), Main (auto), Footer (size 1)
    layout.split_column(vec![
        rich_rust::layout::Layout::new().with_name("header").with_size(3),
        rich_rust::layout::Layout::new().with_name("main").with_ratio(1),
        rich_rust::layout::Layout::new().with_name("footer").with_size(1),
    ]);

    // Split "main" into Sidebar (ratio 1) and Body (ratio 3)
    // Note: We access children by index. 0=header, 1=main, 2=footer.
    layout.children_mut()[1].split_row(vec![
        rich_rust::layout::Layout::new().with_name("sidebar").with_ratio(1),
        rich_rust::layout::Layout::new().with_name("body").with_ratio(3),
    ]);
    
    layout
}
```

## 3. Creating Content Components

We can use helper functions to generate the content (Renderables) for each section.

```rust
fn get_header() -> Panel {
    Panel::new(
        Text::from("Server Monitor v1.0")
            .alignment(Alignment::Center)
            .style(Style::new().bold().foreground(Color::White).background(Color::Blue))
    )
    .border_style(BorderStyle::Heavy)
}

fn get_sidebar() -> Panel {
    Panel::new(
        Text::from("Dashboard\nProcesses\nNetwork\nDisk\nSettings")
    )
    .title("Menu")
}

fn get_footer() -> Text {
    Text::from("Status: Online | Press Ctrl+C to exit")
        .style(Style::new().dim())
        .alignment(Alignment::Center)
}

fn get_body_table(tick: u64) -> Panel {
    let mut table = Table::new();
    table.add_column("PID");
    table.add_column("Process");
    table.add_column("CPU %");
    table.add_column("Mem %");
    
    // Simulate changing data
    let cpu_load = (tick * 7) % 100;
    table.add_row_strs(&["1023", "fast-rich-demo", &format!("{}", cpu_load), "1.2"]);
    table.add_row_strs(&["4096", "cargo", "0.0", "0.5"]);
    table.add_row_strs(&["8888", "postgres", "3.4", "12.0"]);
    
    Panel::new(table)
        .title("Active Processes")
        .border_style(BorderStyle::Rounded)
}
```

## 4. Bringing it to Live

Now we use `Live` to manage the render loop.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let mut layout = create_layout();
    
    // Fill static parts immediately
    layout.children_mut()[0].update(get_header());
    layout.children_mut()[2].update(get_footer());
    
    // Access main -> sidebar
    layout.children_mut()[1].children_mut()[0].update(get_sidebar());
    
    // Start Live display
    let mut live = rich_rust::live::Live::new();
    live.start().unwrap();
    
    for tick in 0..100 {
        // Update the body (dynamic part)
        // main (idx 1) -> body (idx 1)
        let body_content = get_body_table(tick);
        layout.children_mut()[1].children_mut()[1].update(body_content);
        
        // Update the live display with the full layout tree
        live.update(layout.clone()); // Cloning layout is cheap (Arc internally for renderables)
        live.refresh().unwrap();
        
        thread::sleep(Duration::from_millis(200));
    }
    
    live.stop().unwrap();
}
```

## Summary

In this tutorial, you learned how to:
1.  Structure a complex terminal UI using recursive `Layout`s.
2.  Use `Panel` and `Table` to organize information.
3.  Use `Live` to animate the dashboard without screen flickering.

Check out `examples/layout_demo.rs` and `examples/live_table.rs` in the repository for runnable code!
