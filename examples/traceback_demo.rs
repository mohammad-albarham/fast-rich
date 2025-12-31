use rich_rust::prelude::*;
use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn trigger_error() -> Result<(), MyError> {
    Err(MyError::new("Failed to connect to database at 127.0.0.1:5432"))
}

fn main() {
    let console = Console::new().force_color(true);
    
    console.rule("[bold red]Traceback & Error Demo[/]");
    console.newline();

    console.print("[bold]1. install_panic_hook()[/]");
    console.print("Calling `rich_rust::traceback::install_panic_hook()` will catch panics and print specific tracebacks.");
    console.print("(Not triggering a real panic here to allow example to finish)");
    console.newline();

    // Simulate an Error Result
    console.print("[bold]2. Formatting Result::Err[/]");
    
    if let Err(e) = trigger_error() {
        // Create traceback from error
        let traceback = rich_rust::traceback::Traceback::from_error(&e.to_string())
            .with_config(rich_rust::traceback::TracebackConfig {
                show_source: false,
                border_style: rich_rust::panel::BorderStyle::Rounded,
                ..Default::default()
            });
            
        console.print_renderable(&traceback);
    }
    console.newline();

    // Source Code Traceback (Simulated)
    // In a real panic, this would read the file. We simulate it here by pointing to this file.
    console.print("[bold]3. Source Context[/]");
    let _tb = rich_rust::traceback::Traceback::from_error("Error at line 67")
        .with_config(rich_rust::traceback::TracebackConfig {
            show_source: true,
            context_lines: 2,
            ..Default::default()
        });
        
    // Hack: Manually injecting location to point to this file for demo
    // The struct fields are private, so we can't easily fake the location 
    // without using the internal API or triggering a real panic.
    // For this example, we'll just show the renderable without location since we can't set it.
    
    console.print("[dim](Source context requires panic info with file location - see 'cargo run' output on failure)[/]");
    
    console.rule("[bold red]End Traceback Demo[/]");
}
