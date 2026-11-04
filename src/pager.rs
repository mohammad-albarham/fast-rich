//! Pager for interactive output pagination.
//!
//! Provides a less-like paging interface for large outputs.

use crate::screen::RawModeGuard;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

/// A simple pager for displaying content with pagination.
pub struct Pager {
    lines: Vec<String>,
    current_line: usize,
    terminal_height: usize,
}

impl Pager {
    /// Create a new pager with content.
    pub fn new(content: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let terminal_height = terminal::size()
            .map(|(_, h)| h as usize)
            .unwrap_or(24)
            .saturating_sub(1); // Reserve one line for status

        Pager {
            lines,
            current_line: 0,
            terminal_height,
        }
    }

    /// Show the pager and handle user interaction.
    ///
    /// Raw mode is held by a [`RawModeGuard`], so the terminal is restored
    /// even if rendering or event handling panics. The cursor is made
    /// visible again regardless of whether raw mode disabling succeeded.
    pub fn show(&mut self) -> io::Result<()> {
        let _guard = RawModeGuard::new()?;

        let result = self.run();

        // Always restore the cursor, even if `run` failed.
        let cursor_result = execute!(io::stdout(), cursor::Show);
        result.and(cursor_result)
        // `_guard` drops here and always disables raw mode.
    }

    fn run(&mut self) -> io::Result<()> {
        loop {
            self.render()?;

            match event::read()? {
                // Only react to key presses (crossterm also delivers
                // Release/Repeat events on some platforms, which would
                // double-scroll).
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => self.scroll_down(1),
                    KeyCode::Up | KeyCode::Char('k') => self.scroll_up(1),
                    KeyCode::PageDown | KeyCode::Char(' ') => {
                        self.scroll_down(self.terminal_height)
                    }
                    KeyCode::PageUp => self.scroll_up(self.terminal_height),
                    KeyCode::Home | KeyCode::Char('g') => self.current_line = 0,
                    KeyCode::End | KeyCode::Char('G') => {
                        self.current_line = self.lines.len().saturating_sub(self.terminal_height);
                    }
                    _ => {}
                },
                // Window resizes: re-read the terminal height so the page
                // layout follows the new size.
                Event::Resize(_, height) => {
                    self.terminal_height = (height as usize).saturating_sub(1).max(1);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear screen
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide
        )?;

        // Display visible lines
        let end_line = (self.current_line + self.terminal_height).min(self.lines.len());
        for line in &self.lines[self.current_line..end_line] {
            writeln!(stdout, "{}", line)?;
        }

        // Show status line
        let percent = if self.lines.is_empty() {
            100
        } else {
            (self.current_line * 100) / self.lines.len().max(1)
        };

        execute!(stdout, cursor::MoveTo(0, self.terminal_height as u16))?;
        write!(
            stdout,
            "\r\x1b[7m Lines {}-{}/{} ({}%) | q: quit, arrows/jk: scroll, space: page down \x1b[0m",
            self.current_line + 1,
            end_line,
            self.lines.len(),
            percent
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn scroll_down(&mut self, amount: usize) {
        let max_scroll = self.lines.len().saturating_sub(self.terminal_height);
        self.current_line = (self.current_line + amount).min(max_scroll);
    }

    fn scroll_up(&mut self, amount: usize) {
        self.current_line = self.current_line.saturating_sub(amount);
    }

    /// Get the total number of lines.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

/// Page content through an interactive viewer.
pub fn page(content: String) -> io::Result<()> {
    let mut pager = Pager::new(content);
    pager.show()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pager_creation() {
        let content = "Line 1\nLine 2\nLine 3".to_string();
        let pager = Pager::new(content);

        assert_eq!(pager.line_count(), 3);
        assert_eq!(pager.current_line, 0);
    }

    #[test]
    fn test_scroll_down() {
        let content = (0..100)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let mut pager = Pager::new(content);

        pager.scroll_down(10);
        assert_eq!(pager.current_line, 10);
    }

    #[test]
    fn test_scroll_up() {
        let content = (0..100)
            .map(|i| format!("Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let mut pager = Pager::new(content);

        pager.scroll_down(20);
        pager.scroll_up(5);
        assert_eq!(pager.current_line, 15);
    }

    #[test]
    fn test_scroll_bounds() {
        let content = "Line 1\nLine 2\nLine 3".to_string();
        let mut pager = Pager::new(content);

        // Can't scroll up from beginning
        pager.scroll_up(10);
        assert_eq!(pager.current_line, 0);

        // Can't scroll past end
        pager.scroll_down(1000);
        assert!(pager.current_line <= pager.line_count());
    }
}
