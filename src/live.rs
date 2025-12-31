use crate::console::Console;
use crate::renderable::Renderable;
use crossterm::{cursor, execute};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// A live display context for animating content in the terminal.
pub struct Live {
    #[allow(dead_code)]
    console: Console,
    renderable: Arc<Mutex<Option<Box<dyn Renderable + Send + Sync>>>>,
    last_height: usize,
    cursor_hidden: bool,
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
            last_height: 0,
            cursor_hidden: false,
        }
    }

    /// Set the object to display.
    pub fn update<R: Renderable + Send + Sync + 'static>(&mut self, renderable: R) {
        let mut lock = self.renderable.lock().unwrap();
        *lock = Some(Box::new(renderable));
    }

    /// Start the live display (hides cursor).
    pub fn start(&mut self) -> io::Result<()> {
        if !self.cursor_hidden {
            execute!(io::stdout(), cursor::Hide)?;
            self.cursor_hidden = true;
        }
        Ok(())
    }

    /// Stop the live display (shows cursor).
    pub fn stop(&mut self) -> io::Result<()> {
        if self.cursor_hidden {
            execute!(io::stdout(), cursor::Show)?;
            self.cursor_hidden = false;
        }
        // Move to the bottom of the last render to avoid overwriting it
        // actually, usually we want to preserve the last frame.
        // so we just print a newline.
        println!();
        Ok(())
    }

    /// Refresh the display by clearing previous height and re-rendering.
    pub fn refresh(&mut self) -> io::Result<()> {
        let lock = self.renderable.lock().unwrap();

        // 1. Clear previous output if we rendered before
        if self.last_height > 0 {
            // Move up `last_height` times
            execute!(io::stdout(), cursor::MoveUp(self.last_height as u16))?;
            // Clear from cursor down (optional, or just overwrite)
            // execute!(io::stdout(), terminal::Clear(terminal::ClearType::FromCursorDown))?;
            // Overwriting is safer than clearing which might flicker more
        }

        // 2. Render new content
        if let Some(renderable) = &*lock {
            // We can capture the output first to count lines
            // BUT Console prints directly usually.
            // We need to capture from Console helper.

            // Create a temporary capture console to measure height
            let capture = Console::capture();
            capture.print_renderable(renderable.as_ref());
            let output = capture.get_captured_output();

            // 3. Print the output to real stdout
            // We use print! instead of console.print to control raw bytes if needed,
            // but console.print is fine if we are sure it doesn't add extra newlines we don't know about.
            // console.print adds a newline at the end usually? No, `print_renderable` does not necessarily.
            // Let's use `print!("{}", output)`
            print!("{}", output);
            io::stdout().flush()?;

            // 4. Update height
            // Count newlines. Note that text wrapping might add lines not explicit.
            // Since we captured via Console (which handles wrapping), the newlines in `output` are real.
            let height = output.matches('\n').count();

            // If the output doesn't end with newline, `matches` might be off by one visually if cursor wraps?
            // Usually print_renderable ensures lines.
            self.last_height = height;
        }

        Ok(())
    }
}

impl Drop for Live {
    fn drop(&mut self) {
        // Ensure cursor is visible when dropped
        let _ = self.stop();
    }
}
