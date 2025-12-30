//! Progress bar example.

use rich_rust::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Progress Bar Examples ═══[/]");
    console.println("");

    // Simple track example
    console.println("[dim]Using track() for simple iteration:[/]");
    console.println("");

    for _i in track(0..20, "Processing items") {
        // Simulate work
        thread::sleep(Duration::from_millis(50));
    }

    console.println("");
    console.println("[dim]Multi-task progress:[/]");
    console.println("");

    // Multi-task progress
    let progress = Progress::new();

    let task1 = progress.add_task("Downloading files", Some(100));
    let task2 = progress.add_task("Processing data", Some(50));

    // Print initial state
    println!();

    for i in 0..100 {
        // Update tasks
        progress.update(task1, i + 1);
        if i % 2 == 0 && i < 50 {
            progress.advance(task2, 1);
        }

        progress.print();
        thread::sleep(Duration::from_millis(30));
    }

    // Finish second task
    for _i in 25..50 {
        progress.advance(task2, 1);
        progress.print();
        thread::sleep(Duration::from_millis(30));
    }

    println!();
    console.println("[bold green]:check_mark: All tasks completed![/]");

    console.println("");
    console.println("[dim]Spinner styles:[/]");
    console.println("");

    // Show different spinner styles
    let styles = [
        (SpinnerStyle::Dots, "Dots"),
        (SpinnerStyle::Line, "Line"),
        (SpinnerStyle::Dots2, "Dots2"),
        (SpinnerStyle::Arc, "Arc"),
        (SpinnerStyle::Circle, "Circle"),
    ];

    for (style, name) in styles {
        print!("  {} ", name);
        for _ in 0..10 {
            let spinner = Spinner::new("").style(style);
            print!("\r  {} {}", name, spinner.current_frame());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(style.interval_ms()));
        }
        println!();
    }

    console.println("");
    console.println("[bold green]:sparkles: Demo complete![/]");
}
