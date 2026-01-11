//! List all available spinner styles with live preview using Progress
//!
//! Run with: cargo run --example list_spinners

use fast_rich::console::Console;
use fast_rich::progress::{Progress, SpinnerColumn, TextColumn, SpinnerStyle};
use std::{thread, time::Duration};

fn main() {
    println!("All {} available spinner styles:\n", SpinnerStyle::all_names().len());
    
    for name in SpinnerStyle::all_names() {
        if let Some(style) = SpinnerStyle::from_name(name) {
            // This is how users choose spinner styles:
            // SpinnerColumn::new().with_style(SpinnerStyle::Moon)
            let columns: Vec<Box<dyn fast_rich::progress::ProgressColumn>> = vec![
                Box::new(SpinnerColumn::new().with_style(style)),
                Box::new(TextColumn::new(&format!("{:20}", name))),
            ];
            
            let console = Console::new();
            let mut progress = Progress::new()
                .with_console(console)
                .with_columns(columns);
            
            progress.start();
            let task_id = progress.add_task("", Some(8));
            
            // Show spinner animating for a few frames
            for i in 0..=8 {
                progress.update(task_id, i);
                progress.refresh();
                thread::sleep(Duration::from_millis(style.interval_ms()));
            }
            progress.stop();
        }
    }
    
    println!("\n✓ Done!");
    println!("\nUsage:");
    println!("  SpinnerColumn::new().with_style(SpinnerStyle::Moon)");
    println!("  SpinnerStyle::from_name(\"moon\")");
}
