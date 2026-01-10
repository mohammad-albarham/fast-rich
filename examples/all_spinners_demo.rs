//! Demo: Showcase all 80+ spinner styles
//!
//! Run with: cargo run --example all_spinners_demo

use fast_rich::console::Console;
use fast_rich::progress::SpinnerStyle;
use std::{thread, time::Duration, io::Write};

fn main() {
    let console = Console::new();
    
    console.println("[bold cyan]═══ All Spinner Styles Demo ═══[/]");
    console.println("");
    console.println("[dim]Showing a sample of 80+ available spinner styles...[/]");
    console.println("");

    // Show a selection of popular spinners
    let spinners_to_show = [
        // Braille dots
        ("dots", SpinnerStyle::Dots),
        ("dots2", SpinnerStyle::Dots2),
        ("dots12", SpinnerStyle::Dots12),
        
        // Lines
        ("line", SpinnerStyle::Line),
        ("pipe", SpinnerStyle::Pipe),
        
        // Shapes
        ("arc", SpinnerStyle::Arc),
        ("circle", SpinnerStyle::Circle),
        ("triangle", SpinnerStyle::Triangle),
        
        // Arrows
        ("arrow", SpinnerStyle::Arrow),
        ("arrow3", SpinnerStyle::Arrow3),
        
        // Growing
        ("growVertical", SpinnerStyle::GrowVertical),
        ("growHorizontal", SpinnerStyle::GrowHorizontal),
        ("aesthetic", SpinnerStyle::Aesthetic),
        
        // Toggles
        ("toggle", SpinnerStyle::Toggle),
        ("toggle3", SpinnerStyle::Toggle3),
        
        // Animations
        ("bouncingBar", SpinnerStyle::BouncingBar),
        ("bouncingBall", SpinnerStyle::BouncingBall),
        
        // Emoji
        ("clock", SpinnerStyle::Clock),
        ("moon", SpinnerStyle::Moon),
        ("earth", SpinnerStyle::Earth),
        ("hearts", SpinnerStyle::Hearts),
        
        // Misc
        ("star", SpinnerStyle::Star),
        ("noise", SpinnerStyle::Noise),
    ];

    for (name, style) in spinners_to_show {
        let frames = style.frames();
        let interval = style.interval_ms();
        
        // Show spinner animating for a bit
        let iterations = 15;
        print!("  {:20} ", name);
        std::io::stdout().flush().unwrap();
        
        for i in 0..iterations {
            let frame = frames[i % frames.len()];
            print!("\r  {:20} {}", name, frame);
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(interval));
        }
        println!();
    }

    console.println("");
    console.println("[bold green]✓ Total spinner styles available:[/] [cyan]{}[/]", );
    println!("  Total available: {}", SpinnerStyle::all_names().len());
    
    console.println("");
    console.println("[dim]Use SpinnerStyle::from_name(\"name\") for string lookup[/]");
    console.println("[dim]Use SpinnerStyle::all_names() to list all options[/]");
}
