# Live Display

The `Live` component enables flicker-free, auto-updating terminal displays. Perfect for dashboards, real-time monitoring, and dynamic content.

## Quick Example

```rust
use fast_rich::prelude::*;
use fast_rich::live::Live;
use std::thread;
use std::time::Duration;

fn main() {
    let mut live = Live::new();
    live.start().unwrap();
    
    for i in 0..10 {
        let text = Text::plain(format!("Counter: {}", i));
        live.update(text);
        live.refresh().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    
    live.stop().unwrap();
}
```

---

## How It Works

`Live` manages the terminal cursor to redraw content in place:

1. **Start**: Saves cursor position, hides cursor
2. **Update**: Stores new content to render
3. **Refresh**: Clears previous output, draws new content
4. **Stop**: Restores cursor, cleans up

This creates smooth, flicker-free updates without screen flashing.

---

## Basic Usage

### Creating and Starting

```rust
use fast_rich::live::Live;

let mut live = Live::new();
live.start().expect("Failed to start live display");
```

### Updating Content

```rust
// Update with any Renderable
live.update(Text::plain("New content"));
live.update(table);
live.update(panel);
live.update(layout);
```

### Refreshing Display

```rust
// Redraw the screen
live.refresh().expect("Failed to refresh");
```

### Stopping

```rust
// Clean up and restore terminal
live.stop().expect("Failed to stop");
```

---

## Live Clock Example

```rust
use fast_rich::prelude::*;
use fast_rich::live::Live;
use std::thread;
use std::time::Duration;
use chrono::Local;

fn main() {
    let mut live = Live::new();
    live.start().unwrap();

    for _ in 0..60 {
        let time = Local::now().format("%H:%M:%S").to_string();
        let panel = Panel::new(
            Text::styled(
                &format!("🕐 {}", time),
                Style::new().bold().foreground(Color::Cyan)
            )
        ).title("Live Clock");
        
        live.update(panel);
        live.refresh().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    live.stop().unwrap();
}
```

---

## Live Dashboard with Layout

Combine `Live` with `Layout` for complex dashboards:

```rust
use fast_rich::prelude::*;
use fast_rich::live::Live;
use fast_rich::layout::Layout;

fn main() {
    let mut live = Live::new();
    live.start().unwrap();

    for tick in 0..100 {
        // Create layout structure
        let mut layout = Layout::new();
        layout.split_row(vec![
            Layout::new().with_name("left"),
            Layout::new().with_name("right"),
        ]);
        
        // Update content
        let left_panel = Panel::new(Text::plain("Status: Active"));
        let right_panel = Panel::new(Text::plain(format!("Tick: {}", tick)));
        
        layout.children_mut()[0].update(left_panel);
        layout.children_mut()[1].update(right_panel);
        
        live.update(layout);
        live.refresh().unwrap();
        
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    live.stop().unwrap();
}
```

---

## Error Handling

Always handle potential errors:

```rust
use fast_rich::live::Live;

fn run_dashboard() -> Result<(), Box<dyn std::error::Error>> {
    let mut live = Live::new();
    live.start()?;
    
    // ... updates ...
    
    live.stop()?;
    Ok(())
}
```

---

## Real Terminal Output

!!! example "Live display in action"

    **Code**
    
    ```rust
    use fast_rich::prelude::*;
    use fast_rich::live::Live;
    use std::thread;
    use std::time::Duration;
    
    fn main() {
        let mut live = Live::new();
        live.start().unwrap();
        
        for i in 0..100 {
            let panel = Panel::new(Text::plain(format!("Update: {}", i)))
                .title("Live Display");
            live.update(panel);
            live.refresh().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        
        live.stop().unwrap();
    }
    ```

    **Run it**
    
    ```bash
    cargo run --example live_clock
    ```

    **What you'll see**

    ![Live display animation](../assets/live.gif)

---

## Tips

!!! tip "Update Frequency"
    Limit updates to ~10-30 FPS for smooth visual output without 
    overwhelming the terminal:
    ```rust
    thread::sleep(Duration::from_millis(33)); // ~30 FPS
    ```

!!! tip "Graceful Exit"
    Use Ctrl+C handling to ensure `live.stop()` is called:
    ```rust
    ctrlc::set_handler(|| {
        // Handle cleanup
    }).expect("Error setting Ctrl-C handler");
    ```

!!! warning "Nested Live Displays"
    Don't start multiple `Live` instances. Use a single root `Live` 
    with `Layout` for complex UIs.

!!! warning "Print During Live"
    Avoid using `println!` or `console.print()` while `Live` is active.
    All output should go through `live.update()`.
