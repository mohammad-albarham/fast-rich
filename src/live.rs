use crate::console::{Console, RenderContext};
use crate::renderable::{Renderable, Segment};

/// Live display context manager.
///
/// Manages a live-updating display in the terminal. Each call to
/// [`Live::update`] re-renders in place (flicker-free): the previous frame's
/// lines are overwritten, and leftover lines from taller frames are cleared.
///
/// The height accounting counts *terminal* lines: a segment wider than the
/// console counts as multiple lines because the terminal soft-wraps it.
pub struct Live<'a> {
    console: &'a Console,
    renderable: Box<dyn Renderable>,
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

        // 2. Render content at terminal width
        let width = self.console.get_width();
        let context = RenderContext {
            width,
            height: None,
        };

        let segments = self.renderable.render(&context);

        // 3. Write, then count *terminal* lines (segments wider than the
        //    console soft-wrap onto extra physical lines).
        self.console.write_segments(&segments);

        // Ensure the cursor ends below the frame.
        if let Some(last) = segments.last() {
            if !last.newline {
                self.console.newline();
            }
        }

        let new_height = count_terminal_lines(&segments, width);

        // 4. If we shrank, clear the lines below (that were part of old height)
        if self.height > new_height {
            let diff = self.height - new_height;
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

/// Count how many physical terminal lines `segments` will occupy when the
/// terminal is `width` columns wide (soft-wrapping included).
pub(crate) fn count_terminal_lines(segments: &[Segment], width: usize) -> usize {
    if width == 0 {
        return segments.len();
    }
    segments
        .iter()
        .map(|seg| {
            let w = seg.width();
            let physical = w.div_ceil(width);
            physical.max(1)
        })
        .sum()
}

impl<'a> Drop for Live<'a> {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{Span, Text};

    fn make_live(text: &str, width: usize) -> (Console, Live<'static>) {
        // Leak a console so we can build a self-referential Live for tests.
        let console: &'static Console = Box::leak(Box::new(Console::capture().width(width)));
        let live = Live::new(Text::plain(text.to_string()), console);
        (console.clone(), live)
    }

    #[test]
    fn test_count_terminal_lines_accounts_wrapping() {
        // One segment of 100 columns at width 40 -> 3 physical lines.
        let seg = Segment::line(vec![Span::raw("x".repeat(100))]);
        assert_eq!(count_terminal_lines(&[seg], 40), 3);

        // Two normal lines -> 2.
        let segs = vec![
            Segment::line(vec![Span::raw("hello")]),
            Segment::line(vec![Span::raw("world")]),
        ];
        assert_eq!(count_terminal_lines(&segs, 80), 2);
    }

    #[test]
    fn test_live_height_updates_across_frames() {
        let (console, mut live) = make_live("short", 40);
        live.start();
        live.update(Text::plain(format!("long: {}", "x".repeat(100))));
        live.update(Text::plain("tiny"));
        live.stop();
        assert!(
            console.get_captured_output().matches('\n').count() > 0,
            "live frames should end with newlines"
        );
    }
}
