use fast_rich::prelude::*;
use fast_rich::Live;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let console = Console::new();
    
    console.print_raw("\nSimple Live Display Demo\n");
    console.print_raw("Press 'q' or Ctrl+C to exit.\n\n");

    let mut live = Live::new(Text::plain("Initializing..."), &console);
    live.start();

    // Run for 30 updates or until q is pressed
    // We can't easily listen to events while main thread acts as loop without async or threads.
    // For this simple example, we'll just loop and sleep, checking poll.

    for i in 0..100 {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        
        // Create a panel with the time
        let content = Panel::new(
            Text::plain(format!("Update sequence: {}", i))
                .alignment(fast_rich::Alignment::Center)
        )
        .title("Live Display")
        .style(fast_rich::Style::new().foreground(fast_rich::Color::Cyan));

        live.update(content);
    }
    
    live.stop();
    console.println("[bold green]Done![/]");
    
    Ok(())
}
