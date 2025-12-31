use log::{info, warn};
use fast_rich::log::RichLogger;

fn main() {
    // Initialize the logger with custom settings (no time, no path)
    RichLogger::builder()
        .enable_time(false)
        .enable_path(false)
        .init()
        .expect("Failed to initialize logger");

    info!("This message has no timestamp or file location");
    warn!("Just the level and message");
}
