//! Live demonstration of the fixed progress auto-refresh.
//!
//! The background refresh thread (review item 5.1) re-renders the display on
//! its own — this demo only advances task counters from worker threads and
//! never calls `refresh()` manually. Run it in a terminal and watch the bars
//! move by themselves:
//!
//! ```sh
//! cargo run --all-features --example auto_refresh_demo
//! ```

use fast_rich::console::Console;
use fast_rich::progress::{BarColumn, PercentageColumn, Progress, TextColumn, TimeRemainingColumn};
use std::time::Duration;

fn main() {
    let console = Console::new(); // real terminal: TTY colors, live redraws

    console.rule("[bold green]Auto-refresh demo[/]");
    console.print("[dim]No manual refresh() calls — a background thread redraws at 20 Hz.[/]");
    console.newline();

    let mut progress = Progress::new()
        .with_console(console)
        .refresh_per_second(20.0)
        .with_columns(vec![
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(20).expand(true)), // fills spare width (5.3)
            Box::new(PercentageColumn::new()),
            Box::new(TimeRemainingColumn),
        ]);

    let download = progress.add_task("Downloading", Some(200));
    let compile = progress.add_task("Compiling", Some(80));
    let upload = progress.add_task("Uploading", None); // indeterminate pulse

    progress.start();

    // Three "workers" advance tasks concurrently — exactly the pattern the
    // old no-op auto_refresh was supposed to serve. NOTE: no manual
    // refresh() anywhere — the background thread does the redrawing.
    std::thread::scope(|s| {
        for (id, total, step) in [(download, 200u64, 8u64), (compile, 80, 3), (upload, 0, 2)] {
            let progress = &progress;
            s.spawn(move || {
                let rounds = if total == 0 { 40 } else { total / step };
                for _ in 0..rounds {
                    std::thread::sleep(Duration::from_millis(45));
                    progress.advance(id, step);
                }
            });
        }
    });

    std::thread::sleep(Duration::from_millis(300));
    progress.stop();

    println!();
    println!("Done — the display above was redrawn by the auto-refresh thread.");
}
