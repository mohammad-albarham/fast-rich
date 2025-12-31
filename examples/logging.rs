use log::{debug, error, info, trace, warn};
use rich_rust::log::RichLogger;

fn main() {
    // Initialize the logger with default settings
    RichLogger::init().expect("Failed to initialize logger");

    info!("Starting logging example...");
    
    // Test different levels
    trace!("This is a trace message - usually seemingly invisible by default default level?");
    debug!("This is a debug message - useful for developers");
    info!("This is an info message - standard operational info");
    warn!("This is a warning - something might vary slightly wrong");
    error!("This is an error - something went wrong!");

    // Test structured-like logging (Rich style)
    info!("User logged in: user_id={}", 12345);
    
    // Demonstrate logging from a function to see path
    complex_calculation();

    info!("Logging example complete.");
}

fn complex_calculation() {
    debug!("Starting complex calculation...");
    // Simulate work
    warn!("Calculation taking longer than expected");
    // Simulate error
    error!("Calculation failed: division by zero");
}
