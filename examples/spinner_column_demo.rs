//! Demo: Testing SpinnerColumn with different spinner styles
//!
//! Run with: cargo run --example spinner_column_demo

use fast_rich::console::Console;
use fast_rich::progress::{Progress, SpinnerColumn, BarColumn, TextColumn, PercentageColumn, SpinnerStyle};
use std::{thread, time::Duration};

/// Custom SpinnerColumn that allows setting the style
#[derive(Debug)]
struct StyledSpinnerColumn {
    style: SpinnerStyle,
}

impl StyledSpinnerColumn {
    fn new(style: SpinnerStyle) -> Self {
        Self { style }
    }
}

impl fast_rich::progress::ProgressColumn for StyledSpinnerColumn {
    fn render(&self, task: &fast_rich::progress::Task) -> Vec<fast_rich::text::Span> {
        let frames = self.style.frames();
        let interval = self.style.interval_ms();
        let elapsed_ms = task.elapsed().as_millis() as u64;
        let idx = ((elapsed_ms / interval) as usize) % frames.len();
        
        vec![fast_rich::text::Span::styled(
            frames[idx].to_string(),
            fast_rich::style::Style::new().foreground(fast_rich::style::Color::Green),
        )]
    }
}

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
    
    println!("\n🎉 Demo complete!");
    println!("\n📋 Available spinners: {}", SpinnerStyle::all_names().len());
}

fn demo_spinner(name: &str, style: SpinnerStyle) {
    println!("━━━ {} ━━━\n", name);
    
    let columns: Vec<Box<dyn fast_rich::progress::ProgressColumn>> = vec![
        Box::new(StyledSpinnerColumn::new(style)),
        Box::new(TextColumn::new("[progress.description]")),
        Box::new(BarColumn::new(30)),
        Box::new(PercentageColumn::new()),
    ];
    
    let console = Console::new();
    let mut progress = Progress::new()
        .with_console(console)
        .with_columns(columns);
    
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
