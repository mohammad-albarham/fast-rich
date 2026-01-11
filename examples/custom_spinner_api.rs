//! Example: Using the new configurable SpinnerColumn API
//!
//! Run with: cargo run --example custom_spinner_api

use fast_rich::console::Console;
use fast_rich::progress::{Progress, SpinnerColumn, SpinnerStyle, TextColumn};
use std::{thread, time::Duration};

fn main() {
    let console = Console::new();

    println!("\n🚀 Testing New API: Configurable SpinnerColumn\n");

    // Choose ONE of the following options to test:

    // Option 1: Create a column with Moon style directly (Type-safe)
    let spinner_col = SpinnerColumn::new().with_style(SpinnerStyle::Moon);
    let label = "Moon Spinner";

    // Option 2: Create a column using string lookup (Flexible)
    // Uncomment the lines below to test string-based lookup:
    // let spinner_col = SpinnerColumn::new().with_style(
    //     SpinnerStyle::from_name("dots12").expect("Invalid spinner name")
    // );
    // let label = "Dots12 (lookup)";

    // 3. Setup Progress with the chosen column
    let mut progress = Progress::new().with_console(console).with_columns(vec![
        Box::new(spinner_col),
        Box::new(TextColumn::new(label)),
    ]);

    progress.start();
    let task_id = progress.add_task("Downloading parity...", Some(50));

    for i in 0..=50 {
        progress.update(task_id, i);
        progress.refresh();
        thread::sleep(Duration::from_millis(80));
    }

    progress.stop();
    println!("\n✅ Custom API test successful!");
}
