//! Status context for showing work in progress.

use super::spinner::{Spinner, SpinnerStyle};
use crate::style::Style;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A status indicator that shows a spinner while work is happening.
///
/// This is similar to Rich's `console.status()` context manager.
pub struct Status {
    /// The message text
    message: Arc<Mutex<String>>,
    /// Spinner style
    spinner_style: SpinnerStyle,
    /// Whether the status is currently running
    running: Arc<AtomicBool>,
    /// Handle to the spinner thread
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Status {
    /// Create a new status with a message.
    pub fn new(message: &str) -> Self {
        Status {
            message: Arc::new(Mutex::new(message.to_string())),
            spinner_style: SpinnerStyle::Dots,
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    /// Set the spinner style.
    pub fn spinner_style(mut self, style: SpinnerStyle) -> Self {
        self.spinner_style = style;
        self
    }

    /// Set the spinner character style (no-op for simplified version).
    pub fn style(self, _style: Style) -> Self {
        self
    }

    /// Start the status display.
    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        let running = self.running.clone();
        let message = self.message.clone();
        let spinner_style = self.spinner_style;

        self.thread_handle = Some(thread::spawn(move || {
            let spinner = Spinner::new("").style(spinner_style);
            while running.load(Ordering::SeqCst) {
                // Clear line and print spinner
                let frame = spinner.current_frame();
                let msg = message.lock().unwrap().clone();
                print!("\r\x1B[K{} {}", frame, msg);
                let _ = io::stdout().flush();

                thread::sleep(Duration::from_millis(spinner_style.interval_ms()));
            }

            // Clear the status line when done
            print!("\r\x1B[K");
            let _ = io::stdout().flush();
        }));
    }

    /// Stop the status display.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    /// Update the status message.
    pub fn update(&mut self, message: &str) {
        *self.message.lock().unwrap() = message.to_string();
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        self.stop();
    }
}

/// A guard that displays a status while in scope.
///
/// This implements an RAII pattern similar to Python's context manager.
pub struct StatusGuard {
    status: Status,
}

impl StatusGuard {
    /// Create and start a new status guard.
    pub fn new(message: &str) -> Self {
        let mut status = Status::new(message);
        status.start();
        StatusGuard { status }
    }

    /// Create with a custom spinner style.
    #[allow(dead_code)]
    pub fn with_style(message: &str, spinner_style: SpinnerStyle) -> Self {
        let mut status = Status::new(message).spinner_style(spinner_style);
        status.start();
        StatusGuard { status }
    }

    /// Update the message.
    #[allow(dead_code)]
    pub fn update(&mut self, message: &str) {
        self.status.update(message);
    }
}

impl Drop for StatusGuard {
    fn drop(&mut self) {
        self.status.stop();
    }
}

/// Convenience function to run work with a status spinner.
///
/// # Example
///
/// ```no_run
/// use rich_rust::progress::with_status;
///
/// let result = with_status("Loading data...", || {
///     // Do some work
///     std::thread::sleep(std::time::Duration::from_secs(1));
///     42
/// });
/// ```
pub fn with_status<T, F: FnOnce() -> T>(message: &str, f: F) -> T {
    let _guard = StatusGuard::new(message);
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_new() {
        let status = Status::new("Testing...");
        assert!(!status.running.load(Ordering::SeqCst));
    }

    #[test]
    fn test_status_guard_drop() {
        // This mainly tests that drop doesn't panic
        {
            let _guard = StatusGuard::new("Test");
            thread::sleep(Duration::from_millis(50));
        }
        // Guard should be dropped and cleaned up
    }
}
