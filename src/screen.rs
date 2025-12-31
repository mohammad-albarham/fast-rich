//! Alternate screen support for full-screen terminal applications.
//!
//! Provides utilities for entering/exiting alternate screen buffer.

use crossterm::{
    cursor, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

/// Guard that automatically exits alternate screen when dropped.
pub struct AlternateScreen {
    active: bool,
}

impl AlternateScreen {
    /// Enter the alternate screen.
    pub fn enter() -> io::Result<Self> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), cursor::Hide)?;

        Ok(AlternateScreen { active: true })
    }

    /// Manually exit the alternate screen.
    pub fn exit(&mut self) -> io::Result<()> {
        if self.active {
            execute!(io::stdout(), cursor::Show)?;
            terminal::disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
            self.active = false;
        }
        Ok(())
    }

    /// Check if alternate screen is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Clear the alternate screen.
    pub fn clear(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        io::stdout().flush()
    }

    /// Get terminal size.
    pub fn size(&self) -> io::Result<(u16, u16)> {
        terminal::size()
    }
}

impl Drop for AlternateScreen {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}

/// Run a function in alternate screen mode.
pub fn with_alternate_screen<F, R>(f: F) -> io::Result<R>
where
    F: FnOnce(&AlternateScreen) -> io::Result<R>,
{
    let screen = AlternateScreen::enter()?;
    let result = f(&screen);
    drop(screen); // Explicit drop to exit alternate screen
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_alternate_screen_creation() {
        // Can't easily test in CI, but we can verify the struct compiles
        assert!(true);
    }
}
