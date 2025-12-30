use crate::console::Console;
use crate::renderable::Renderable;
use std::io;
use std::sync::{Arc, Mutex};

/// A live display context.
pub struct Live {
    console: Console,
    renderable: Arc<Mutex<Option<Box<dyn Renderable + Send + Sync>>>>,
    #[allow(dead_code)]
    refresh_rate: u64,
    #[allow(dead_code)]
    rendering: Arc<Mutex<bool>>,
}

impl Default for Live {
    fn default() -> Self {
        Self::new()
    }
}

impl Live {
    /// Create a new Live display.
    pub fn new() -> Self {
        Self {
            console: Console::new(),
            renderable: Arc::new(Mutex::new(None)),
            refresh_rate: 4, // 4Hz default
            rendering: Arc::new(Mutex::new(false)),
        }
    }

    /// Set the object to display.
    pub fn update<R: Renderable + Send + Sync + 'static>(&mut self, renderable: R) {
        let mut lock = self.renderable.lock().unwrap();
        *lock = Some(Box::new(renderable));
    }

    /// Start the live display (blocks until stopped or generic loop).
    /// For the Rust version, we likely want a background thread or manual refresh.
    /// This implementation is a "manual refresh" handle.
    pub fn refresh(&self) -> io::Result<()> {
        let lock = self.renderable.lock().unwrap();
        if let Some(renderable) = &*lock {
            self.console.clear();
            self.console.print_renderable(renderable.as_ref());
        }
        Ok(())
    }
}
