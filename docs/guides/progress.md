# Progress Bars

Fast-Rich provides a powerful progress bar system for tracking long-running tasks with multi-bar support, spinners, ETA, and customizable columns.

## Python-Style track() Iterator

The simplest way to add progress bars—just wrap any iterator:

```rust
use fast_rich::progress::track;
use std::thread;
use std::time::Duration;

fn main() {
    // Just like Python's: for item in track(range(100), description="Processing")
    for item in track(0..100, "Processing") {
        // Do work with each item
        thread::sleep(Duration::from_millis(50));
    }
}
```

![Track demo](../assets/track_demo.gif)

---

## Manual Progress Control

For more control, use `Progress` directly:

```rust
use fast_rich::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let progress = Progress::new();
    let task = progress.add_task("Processing...", Some(100));
    
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(50));
        progress.advance(task, 1);
    }
}
```

---

## Multi-Task Progress

Track multiple tasks simultaneously:

```rust
use fast_rich::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let progress = Progress::new();
    
    let download = progress.add_task("Downloading", Some(100));
    let extract = progress.add_task("Extracting", Some(50));
    let install = progress.add_task("Installing", Some(200));
    
    // Simulate work
    for i in 0..100 {
        thread::sleep(Duration::from_millis(30));
        progress.advance(download, 1);
        
        if i % 2 == 0 {
            progress.advance(extract, 1);
        }
        if i % 1 == 0 {
            progress.advance(install, 2);
        }
    }
}
```

**Output:**
```
Downloading ━━━━━━━━━━━━━━━━━━━━ 100% 00:00
Extracting  ━━━━━━━━━━━━━━━━━━━━ 100% 00:00
Installing  ━━━━━━━━━━━━━━━━━━━━ 100% 00:00
```

---

## Task Management

### Adding Tasks

```rust
// Known total (shows percentage)
let task1 = progress.add_task("Download", Some(total_bytes));

// Unknown total (shows spinner)
let task2 = progress.add_task("Scanning", None);
```

### Updating Progress

```rust
// Advance by amount
progress.advance(task_id, 10);

// Set absolute progress
progress.set_completed(task_id, 50);

// Mark as complete
progress.set_completed(task_id, total);
```

### Updating Description

```rust
progress.update_description(task_id, "Almost done...");
```

---

## Spinners

For tasks with unknown duration, use spinners:

```rust
use fast_rich::progress::{Progress, SpinnerColumn, SpinnerStyle, TextColumn, BarColumn};
use fast_rich::console::Console;

fn main() {
    let columns: Vec<Box<dyn fast_rich::progress::ProgressColumn>> = vec![
        Box::new(SpinnerColumn::new().with_style(SpinnerStyle::Moon)),
        Box::new(TextColumn::new("[progress.description]")),
        Box::new(BarColumn::new(30)),
    ];
    
    let mut progress = Progress::new()
        .with_console(Console::new())
        .with_columns(columns);
    
    progress.start();
    let task = progress.add_task("Processing...", Some(100));
    // ... update progress ...
    progress.stop();
}
```

### Spinner Styles

Fast-Rich includes **80+ spinner styles**. Here are some popular ones:

| Style | Pattern | Description |
|:------|:--------|:------------|
| `SpinnerStyle::Dots` | `⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏` | Classic braille dots (default) |
| `SpinnerStyle::Line` | `- \\ | /` | Simple ASCII line |
| `SpinnerStyle::Moon` | 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘 | Moon phases |
| `SpinnerStyle::Earth` | 🌍 🌎 🌏 | Rotating globe |
| `SpinnerStyle::Clock` | 🕐 🕑 🕒 ... 🕛 | Clock faces |
| `SpinnerStyle::Hearts` | 💛 💙 💜 💚 ❤️ | Color hearts |
| `SpinnerStyle::Star` | ✶ ✸ ✹ ✺ ✹ ✷ | Twinkling star |
| `SpinnerStyle::Arrow` | ← ↖ ↑ ↗ → ↘ ↓ ↙ | Rotating arrow |
| `SpinnerStyle::BouncingBar` | `[=   ]` `[ =  ]` ... | Bouncing bar |
| `SpinnerStyle::GrowHorizontal` | ▏ ▎ ▍ ▌ ▋ ▊ ▉ █ | Growing block |
| `SpinnerStyle::Aesthetic` | ▰▱▱▱ ▰▰▱▱ ... | Aesthetic blocks |

!!! tip "List All Spinners"
    ```rust
    use fast_rich::progress::SpinnerStyle;
    println!("Available: {} styles", SpinnerStyle::all_names().len());
    ```

Run the spinner demo:
```bash
cargo run --example spinner_column_demo
```

---

## Customizable Columns

Progress bars can display different information columns:

```rust
use fast_rich::progress::*;

let progress = Progress::new()
    .with_columns(vec![
        ProgressColumn::Description,
        ProgressColumn::Bar,
        ProgressColumn::Percentage,
        ProgressColumn::TimeRemaining,
    ]);
```

### Available Columns

| Column | Description |
|:-------|:------------|
| `Description` | Task description text |
| `Bar` | Visual progress bar |
| `Percentage` | Completion percentage |
| `Completed` | Completed/Total count |
| `TimeRemaining` | Estimated time remaining |
| `TransferSpeed` | Transfer speed (bytes/sec) |
| `Spinner` | Animated spinner |

---

## Snapshot Rendering

For non-interactive output or logging, render progress as a string:

```rust
let progress = Progress::new();
let task = progress.add_task("Working...", Some(100));
progress.advance(task, 50);

// Get rendered string
let output = progress.render_to_string();
println!("{}", output);
```

This is useful for:
- Logging progress state
- Testing
- Non-interactive environments

---

## Real Terminal Output

!!! example "Progress bar in action"

    **Code**
    
    ```rust
    use fast_rich::prelude::*;
    use std::thread;
    use std::time::Duration;
    
    fn main() {
        let progress = Progress::new();
        let download = progress.add_task("Downloading", Some(100));
        let extract = progress.add_task("Extracting", Some(50));
        let install = progress.add_task("Installing", Some(200));
        
        for i in 0..100 {
            thread::sleep(Duration::from_millis(30));
            progress.advance(download, 1);
            if i % 2 == 0 { progress.advance(extract, 1); }
            progress.advance(install, 2);
        }
    }
    ```

    **Run it**
    
    ```bash
    cargo run --example progress_bar
    ```

    **What you'll see**

    ![Progress bar animation](../assets/progress.gif)

---

## Tips

!!! tip "Use with Live Display"
    For the smoothest animations, combine Progress with Live:
    ```rust
    use fast_rich::live::Live;
    // Progress automatically integrates with Live when rendered
    ```

!!! tip "Update Rate"
    Don't update progress too frequently. For very fast iterations,
    batch updates or use a timer to limit refresh rate.

!!! warning "Terminal Compatibility"
    Progress bars use ANSI cursor control. They work best in 
    interactive terminals. In CI environments, consider using 
    snapshot rendering or simpler output.

---

## Try the Examples

Explore the full capabilities of progress bars and spinners:

```bash
# Basic progress bar demo
cargo run --example progress_rich

# Python-style track() iterator
cargo run --example docs_track

# All 85 spinner styles
cargo run --example all_spinners_demo

# Custom spinner configuration
cargo run --example spinner_column_demo
cargo run --example custom_spinner_api

# List available spinner names
cargo run --example list_spinners
```

These examples demonstrate:
- ✅ Multi-task progress tracking
- ✅ Custom columns (spinner, percentage, ETA, speed)
- ✅ 85 spinner styles (emoji, ASCII, unicode)
- ✅ `track()` iterator for simple loops
- ✅ `start()`/`stop()`/`refresh()` lifecycle
