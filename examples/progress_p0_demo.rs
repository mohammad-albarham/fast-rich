//! Demo of P0 progress bar improvements.
//!
//! Shows:
//! 1. Distinct filled (━) vs unfilled (─) characters
//! 2. Edge pointer (╸) showing progress position  
//! 3. Pulse animation for indeterminate tasks

use fast_rich::console::Console;
use fast_rich::progress::{BarColumn, PercentageColumn, Progress, SpinnerColumn, TextColumn};
use std::thread;
use std::time::Duration;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]Progress Bar P0 Improvements Demo[/]");
    console.println("[dim]─────────────────────────────────────[/]");
    console.println("");

    // Demo 1: Show the distinct characters
    console.println("[bold]1. Distinct filled vs unfilled characters:[/]");
    console.println("   Filled uses ━ (heavy), unfilled uses ─ (light)");
    console.println("");

    let progress = Progress::new().with_columns(vec![
        Box::new(TextColumn::new("[progress.description]")),
        Box::new(BarColumn::new(40)),
        Box::new(PercentageColumn::new()),
    ]);

    let task = progress.add_task("Downloading", Some(100));

    // Show progress at various stages
    println!();
    for pct in [0, 25, 50, 75, 100] {
        progress.update(task, pct);
        progress.print();
        thread::sleep(Duration::from_millis(800));
    }

    println!();
    console.println("");
    console.println("[bold]2. Indeterminate task with pulse animation:[/]");
    console.println("   Watch the pulse move back and forth!");
    console.println("");

    // Demo 2: Indeterminate progress (pulse animation)
    let progress2 = Progress::new().with_columns(vec![
        Box::new(SpinnerColumn::new()),
        Box::new(TextColumn::new("[progress.description]")),
        Box::new(BarColumn::new(40)),
    ]);

    let indeterminate_task = progress2.add_task("Connecting to server", None);

    println!();
    for _ in 0..40 {
        progress2.print();
        thread::sleep(Duration::from_millis(75));
    }
    progress2.finish(indeterminate_task);

    println!();
    console.println("");
    console.println("[bold green]✓ Demo complete![/]");
    console.println("");
    console.println("[dim]Key visual differences from before:[/]");
    console.println("  • Filled portion: ━━━━━ (heavy line)");
    console.println("  • Edge pointer:   ╸     (shows progress position)");
    console.println("  • Unfilled:       ───── (light line, distinct!)");
    console.println("  • Indeterminate:  Pulsing animation instead of 0%");
}
