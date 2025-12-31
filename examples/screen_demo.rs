use rich_rust::screen::{with_alternate_screen, AlternateScreen};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Alternate Screen Demo ===\n");
    println!("This will switch to alternate screen for 5 seconds.");
    println!("Press Enter to start...\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    // Method 1: Manual control
    {
        let mut screen = AlternateScreen::enter().unwrap();
        screen.clear().unwrap();

        let (width, height) = screen.size().unwrap();
        println!("Alternate screen active!");
        println!("Terminal size: {}x{}", width, height);
        println!("\nThis is running in alternate screen mode.");
        println!("The main screen is preserved behind this.");
        println!("\nWaiting 3 seconds...");

        io::stdout().flush().ok();
        thread::sleep(Duration::from_secs(3));

        screen.exit().unwrap();
    }

    println!("\nBack to main screen!\n");
    thread::sleep(Duration::from_secs(1));

    // Method 2: Using closure (auto-cleanup)
    println!("Now using with_alternate_screen closure...\n");
    thread::sleep(Duration::from_secs(1));

    with_alternate_screen(|screen| {
        screen.clear()?;
        println!("Inside alternate screen (via closure)");
        println!("This automatically cleans up when done.");
        println!("\nWaiting 2 seconds...");
        io::stdout().flush()?;
        thread::sleep(Duration::from_secs(2));
        Ok(())
    })
    .ok();

    println!("\nDemo complete!");
}
