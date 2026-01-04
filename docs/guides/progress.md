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
use fast_rich::prelude::*;

fn main() {
    let spinner = Spinner::new()
        .message("Loading...")
        .style(SpinnerStyle::Dots);
    
    // Use with Status for single-task indication
    let status = Status::new("Processing data");
    // ... do work ...
}
```

### Spinner Styles

| Style | Pattern |
|:------|:--------|
| `SpinnerStyle::Dots` | `⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏` |
| `SpinnerStyle::Line` | `- \ | /` |
| `SpinnerStyle::Braille` | `⣾ ⣽ ⣻ ⢿ ⡿ ⣟ ⣯ ⣷` |

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
