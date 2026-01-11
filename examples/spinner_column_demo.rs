//! Demo: Testing SpinnerColumn with different spinner styles
//!
//! Run with: cargo run --example spinner_column_demo

use fast_rich::console::Console;
use fast_rich::progress::{
    BarColumn, PercentageColumn, Progress, SpinnerColumn, SpinnerStyle, TextColumn,
};
use std::{thread, time::Duration};

fn main() {
    println!("\n🎯 SpinnerColumn Demo\n");
    println!("Testing different spinner styles with progress:\n");

    // Demo 1: Default Dots spinner
    demo_spinner("Dots (default)", SpinnerStyle::Dots);

    // Demo 2: Moon emoji spinner
    demo_spinner("Moon 🌙", SpinnerStyle::Moon);

    // Demo 3: Arrow spinner
    demo_spinner("Arrow", SpinnerStyle::Arrow);

    // Demo 4: Aesthetic bar
    demo_spinner("Aesthetic", SpinnerStyle::Aesthetic);

    // Demo 5: Hearts emoji
    demo_spinner("Hearts 💕", SpinnerStyle::Hearts);

    // Demo 6: Clock emoji
    demo_spinner("Clock 🕐", SpinnerStyle::Clock);

    // Demo 7: Bouncing bar animation
    demo_spinner("BouncingBar", SpinnerStyle::BouncingBar);

    // Demo 8: Earth globe
    demo_spinner("Earth 🌍", SpinnerStyle::Earth);

    // Demo 9: Growing horizontal
    demo_spinner("GrowHorizontal", SpinnerStyle::GrowHorizontal);

    // Demo 10: Star
    demo_spinner("Star ✨", SpinnerStyle::Star);

    println!("\n🎉 Demo complete!");
    println!(
        "\n📋 Available spinners: {}",
        SpinnerStyle::all_names().len()
    );
}

fn demo_spinner(name: &str, style: SpinnerStyle) {
    println!("━━━ {} ━━━\n", name);

    // This is the proper user-facing API:
    // SpinnerColumn::new().with_style(SpinnerStyle::Moon)
    let columns: Vec<Box<dyn fast_rich::progress::ProgressColumn>> = vec![
        Box::new(SpinnerColumn::new().with_style(style)),
        Box::new(TextColumn::new("[progress.description]")),
        Box::new(BarColumn::new(30)),
        Box::new(PercentageColumn::new()),
    ];

    let console = Console::new();
    let mut progress = Progress::new().with_console(console).with_columns(columns);

    progress.start();
    let task_id = progress.add_task("Processing...", Some(20));

    for i in 0..=20 {
        progress.update(task_id, i);
        progress.refresh();
        thread::sleep(Duration::from_millis(80));
    }
    progress.stop();

    println!("\n");
}
