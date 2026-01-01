use crate::console::{Console, RenderContext};
use crate::renderable::Renderable;

/// Live display context manager.
///
/// Manages a live-updating display in the terminal.
pub struct Live<'a> {
    console: &'a Console,
    renderable: Box<dyn Renderable>,
    refresh_rate: u64, // Refresh rate in Hz (not currently used for auto-refresh thread in this version)
    transient: bool,
    height: usize,
    started: bool,
}

impl<'a> Live<'a> {
    /// Create a new Live display.
    pub fn new(renderable: impl Renderable + 'static, console: &'a Console) -> Self {
        Live {
            console,
            renderable: Box::new(renderable),
            refresh_rate: 4,
            transient: false,
            height: 0,
            started: false,
        }
    }

    /// Set whether the display should be cleared on exit.
    pub fn transient(mut self, transient: bool) -> Self {
        self.transient = transient;
        self
    }

    /// Set the refresh rate (currently unused in manual refresh mode).
    pub fn refresh_rate(mut self, rate: u64) -> Self {
        self.refresh_rate = rate;
        self
    }

    /// Start the live display.
    pub fn start(&mut self) {
        if self.started {
            return;
        }
        self.console.show_cursor(false);
        self.started = true;
        self.refresh();
    }

    /// Stop the live display.
    pub fn stop(&mut self) {
        if !self.started {
            return;
        }

        // Clean up
        if self.height > 0 {
            self.console.move_cursor_up(self.height as u16);
            // We clear lines downwards to ensure clean exit
            for _ in 0..self.height {
                self.console.clear_line();
                self.console.move_cursor_down(1);
            }
            self.console.move_cursor_up(self.height as u16);
        }

        if !self.transient {
            // If not transient, we want to leave the last frame visible
            // So we reprint it one last time, but this time we don't track height
            // to "lock" it in place (as standard output)
            self.console.print_renderable(self.renderable.as_ref());
            self.console.newline(); // Ensure final newline
        }

        self.console.show_cursor(true);
        self.started = false;
        self.height = 0;
    }

    /// Update the renderable and refresh the display.
    pub fn update(&mut self, renderable: impl Renderable + 'static) {
        self.renderable = Box::new(renderable);
        self.refresh();
    }

    /// Refresh the display with the current renderable.
    pub fn refresh(&mut self) {
        if !self.started {
            return;
        }

        // 1. Move cursor up to overwrite previous output
        if self.height > 0 {
            self.console.move_cursor_up(self.height as u16);
        }

        // 2. Render content
        let width = self.console.get_width();
        let context = RenderContext {
            width,
            height: None, direction: Default::default(),
        };

        let segments = self.renderable.render(&context);

        // 3. Calculate new height
        // We need to know how many lines this render took.
        // Similar to console.print, but we track lines.
        let mut lines = 0;
        for segment in &segments {
            // Write to console
            // We can't use console.print because that might add a newline we don't control
            // cleanly or we want to count lines.
            // Actually console.print adds a newline at the end if we use println.
            // Let's use internal write helpers or just count newlines in segments.
            if segment.newline {
                lines += 1;
            }
        }

        // If the last segment didn't have a newline, it's still occupying a line?
        // Usually renderables ensure newlines or we treat them as flow.
        // For now let's assume one newline per line.

        // Wait, Console::print_renderable just writes segments. It behaves linearly.
        // We want to verify if the output ended with a newline or not.
        // Simplest strategy:
        // We write the segments.
        // Then we ensure we end with a newline so the cursor is on the next line?
        // Or we keep the cursor at the end of the last line?

        // rich.live typically clears the area and rewrites.
        // To avoid flicker:
        // Move up N lines.
        // Clear line? Or just overwrite? Overwriting is better (less flicker).
        // If new content is shorter than old content, we must clear the remainder.

        // Strategy:
        // 1. Buffer the output (optional, but good for flicker).
        // 2. Write output.
        // 3. Count lines written.
        // 4. If new_lines < old_lines, clear remaining old lines.

        // Let's rely on Console to write but we need to buffer to count lines?
        // Or we can just count segments newlines?

        // Let's buffer it for now to be safe and to support "clearing".
        // Actually, Console has a capture mode but we are using the live console.
        // Let's just write and track.

        // NOTE: This simple implementation assumes the renderable *is* the lines.
        self.console.write_segments(&segments);

        // If the segments didn't end with a newline, we add one to separate from potential next output?
        // Rich usually ensures block display.
        if !segments.is_empty() && !segments.last().unwrap().newline {
            self.console.newline();
            lines += 1;
        }

        let new_height = lines;

        // If we shrank, clear the lines below (that were part of old height)
        if self.height > new_height {
            let diff = self.height - new_height;
            // Cursor is currently at end of new content.
            for _ in 0..diff {
                self.console.clear_line(); // Clear this line
                self.console.newline(); // Move down
            }
            // Move back up
            self.console.move_cursor_up(diff as u16);
        }

        self.height = new_height;
    }
}

impl<'a> Drop for Live<'a> {
    fn drop(&mut self) {
        self.stop();
    }
}
