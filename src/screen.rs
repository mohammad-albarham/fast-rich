//! Alternate screen support for full-screen terminal applications.
//!
//! Provides utilities for entering/exiting alternate screen buffer.

use crossterm::{
    cursor, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

/// Guard that automatically exits alternate screen when dropped.
#[derive(Debug)]
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

/// Guard that enables terminal raw mode and always disables it on drop —
/// including on panics and early returns.
///
/// Use this instead of calling `enable_raw_mode`/`disable_raw_mode`
/// sequentially so an error or panic can never leave the user's terminal
/// in raw mode.
#[derive(Debug)]
pub struct RawModeGuard {
    active: bool,
}

impl RawModeGuard {
    /// Enable raw mode.
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(RawModeGuard { active: true })
    }

    /// Manually disable raw mode (also happens on drop).
    pub fn release(&mut self) -> io::Result<()> {
        if self.active {
            terminal::disable_raw_mode()?;
            self.active = false;
        }
        Ok(())
    }

    /// Whether raw mode is still active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        self.active = false;
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
        // Can't easily test alternate screen in CI (requires TTY),
        // but compilation verifies the struct is valid
    }
}
