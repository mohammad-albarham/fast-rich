use crate::console::{Console, RenderContext};
use crate::live::count_terminal_lines;
use crate::progress::columns::{
    BarColumn, PercentageColumn, ProgressColumn, TextColumn, TimeRemainingColumn,
};
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::Span;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// Lock a mutex, tolerating poisoning.
///
/// A panic in another thread while holding the lock must not turn every
/// subsequent render into a cascade of panics.
fn lock_or_recover<T>(lock: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    lock.lock().unwrap_or_else(|e| e.into_inner())
}

/// A task being tracked by the progress bar.
#[derive(Debug, Clone)]
pub struct Task {
    /// Task ID
    pub id: usize,
    /// Task description
    pub description: String,
    /// Total units of work
    pub total: Option<u64>,
    /// Completed units
    pub completed: u64,
    /// Start time
    pub start_time: Instant,
    /// Whether the task is finished
    pub finished: bool,
    /// Style for the progress bar (can be used by columns)
    pub style: Style,
}

impl Task {
    /// Create a new task.
    pub fn new(id: usize, description: &str, total: Option<u64>) -> Self {
        Task {
            id,
            description: description.to_string(),
            total,
            completed: 0,
            start_time: Instant::now(),
            finished: false,
            style: Style::new().foreground(Color::Cyan),
        }
    }

    /// Get the progress percentage (0.0 - 1.0).
    pub fn percentage(&self) -> f64 {
        match self.total {
            Some(total) if total > 0 => (self.completed as f64 / total as f64).min(1.0),
            _ => 0.0,
        }
    }

    /// Get elapsed time.
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Estimate time remaining.
    pub fn eta(&self) -> Option<Duration> {
        if self.completed == 0 {
            return None;
        }

        let elapsed = self.elapsed().as_secs_f64();
        let rate = self.completed as f64 / elapsed;

        self.total.and_then(|total| {
            let remaining = total.saturating_sub(self.completed);
            if rate > 0.0 {
                Some(Duration::from_secs_f64(remaining as f64 / rate))
            } else {
                None
            }
        })
    }

    /// Get the speed (units per second).
    pub fn speed(&self) -> f64 {
        let elapsed = self.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.completed as f64 / elapsed
        } else {
            0.0
        }
    }
}

/// A single progress bar configuration (Deprecated/Legacy support wrapper or helper).
/// Kept for backward compat if anyone used it directly, but mainly used by BarColumn now.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    /// Width of the bar portion
    pub bar_width: usize,
    /// Character for completed portion
    pub complete_char: char,
    /// Character for remaining portion
    pub remaining_char: char,
    /// Style for completed portion
    pub complete_style: Style,
    /// Style for remaining portion
    pub remaining_style: Style,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressBar {
    /// Create a new progress bar.
    pub fn new() -> Self {
        ProgressBar {
            bar_width: 40,
            complete_char: '━',
            remaining_char: '━',
            complete_style: Style::new().foreground(Color::Cyan),
            remaining_style: Style::new().foreground(Color::BrightBlack),
        }
    }
    // ... setters can stay if needed, but we are moving to columns ...

    pub fn width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }
}

/// Multi-task progress display with a live lifecycle.
///
/// `Progress` is cheaply shareable: all mutable state lives behind an `Arc`,
/// so task handles can be advanced from other threads while the display
/// refreshes. `start()`/`stop()` hide the cursor for flicker-free updates;
/// when `auto_refresh` is enabled (the default), a background thread
/// re-renders the display at `refresh_per_second` until `stop()`.
pub struct Progress {
    /// Shared state (tasks, columns, console, refresh bookkeeping).
    inner: Arc<ProgressInner>,
    /// Handle to the background refresh thread (when auto-refresh runs).
    refresh_thread: Option<JoinHandle<()>>,
}

/// State shared between the owning handle and the auto-refresh thread.
struct ProgressInner {
    /// Tasks being tracked
    tasks: Mutex<Vec<Task>>,
    /// Next task ID
    next_id: Mutex<usize>,
    /// Columns to display
    columns: Vec<Box<dyn ProgressColumn>>,
    /// Console for output (used with start/stop lifecycle)
    console: Option<Console>,
    /// Whether the progress display is currently started
    started: AtomicBool,
    /// Current height in terminal lines (for cursor movement)
    height: Mutex<usize>,
    /// If true, clear output on stop instead of leaving visible
    transient: bool,
    /// Refresh rate in Hz (refreshes per second, default: 10.0)
    refresh_per_second: f64,
    /// Whether auto-refresh is enabled (default: true when console is set)
    auto_refresh: bool,
    /// Signal to stop the refresh thread
    stop_signal: AtomicBool,
    /// Serializes rendering between the owner and the refresh thread.
    render_lock: Mutex<()>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    /// Create a new progress display with default columns.
    pub fn new() -> Self {
        Progress {
            inner: Arc::new(ProgressInner {
                tasks: Mutex::new(Vec::new()),
                next_id: Mutex::new(0),
                columns: vec![
                    Box::new(TextColumn::new("[progress.description]")),
                    Box::new(BarColumn::new(40)),
                    Box::new(PercentageColumn::new()),
                    Box::new(TimeRemainingColumn),
                ],
                console: None,
                started: AtomicBool::new(false),
                height: Mutex::new(0),
                transient: false,
                refresh_per_second: 10.0,
                auto_refresh: true,
                stop_signal: AtomicBool::new(false),
                render_lock: Mutex::new(()),
            }),
            refresh_thread: None,
        }
    }

    /// Create a progress display with a Console (enables start/stop lifecycle).
    pub fn with_console(mut self, console: Console) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.console = Some(console);
        }
        self
    }

    /// Set whether output is cleared on stop (transient mode).
    pub fn transient(mut self, transient: bool) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.transient = transient;
        }
        self
    }

    /// Set the refresh rate in Hz (refreshes per second).
    ///
    /// Default is 10.0. Set lower if updates are infrequent to reduce CPU
    /// usage. Values above 60 are clamped.
    pub fn refresh_per_second(mut self, rate: f64) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.refresh_per_second = rate.clamp(0.1, 60.0);
        }
        self
    }

    /// Enable or disable auto-refresh.
    ///
    /// When enabled (default) and a console is set, a background thread
    /// re-renders the display at `refresh_per_second`. When disabled, you
    /// must call `refresh()` manually after updating tasks.
    pub fn auto_refresh(mut self, enabled: bool) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.auto_refresh = enabled;
        }
        self
    }

    /// Set custom columns.
    pub fn with_columns(mut self, columns: Vec<Box<dyn ProgressColumn>>) -> Self {
        if let Some(inner) = Arc::get_mut(&mut self.inner) {
            inner.columns = columns;
        }
        self
    }

    /// Add a new task.
    pub fn add_task(&self, description: &str, total: Option<u64>) -> usize {
        let mut next_id = lock_or_recover(&self.inner.next_id);
        let id = *next_id;
        *next_id += 1;

        let task = Task::new(id, description, total);
        lock_or_recover(&self.inner.tasks).push(task);

        id
    }

    /// Advance a task by the given amount.
    pub fn advance(&self, task_id: usize, amount: u64) {
        let mut tasks = lock_or_recover(&self.inner.tasks);
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.completed += amount;
            if let Some(total) = task.total {
                if task.completed >= total {
                    task.finished = true;
                }
            }
        }
    }

    /// Update a task's completed count.
    pub fn update(&self, task_id: usize, completed: u64) {
        let mut tasks = lock_or_recover(&self.inner.tasks);
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.completed = completed;
            if let Some(total) = task.total {
                if task.completed >= total {
                    task.finished = true;
                }
            }
        }
    }

    /// Mark a task as finished.
    pub fn finish(&self, task_id: usize) {
        let mut tasks = lock_or_recover(&self.inner.tasks);
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.finished = true;
        }
    }

    /// Remove a task.
    pub fn remove(&self, task_id: usize) {
        lock_or_recover(&self.inner.tasks).retain(|t| t.id != task_id);
    }

    /// Check if all tasks are finished.
    pub fn is_finished(&self) -> bool {
        lock_or_recover(&self.inner.tasks)
            .iter()
            .all(|t| t.finished)
    }

    /// Render the progress display to a string.
    ///
    /// Uses the configured console width when available, else 80 columns.
    pub fn render_to_string(&self) -> String {
        let width = self
            .inner
            .console
            .as_ref()
            .map(|c| c.get_width())
            .unwrap_or(80);
        let context = RenderContext {
            width,
            height: None,
        };
        let segments = self.render(&context);

        let mut result = String::new();
        for segment in segments {
            result.push_str(&segment.plain_text());
            if segment.newline {
                result.push('\n');
            }
        }
        result
    }

    /// Print the progress to stdout (with cursor control for updates).
    ///
    /// Only used when no console is configured; prefer `start()`/`refresh()`.
    pub fn print(&self) {
        let output = self.render_to_string();

        let num_lines = lock_or_recover(&self.inner.tasks).len();

        if num_lines > 0 {
            // Move cursor up
            print!("\x1B[{}A", num_lines);
        }

        // Clear lines and print
        for line in output.lines() {
            println!("\x1B[2K{}", line);
        }

        let _ = io::stdout().flush();
    }

    /// Start the live progress display.
    ///
    /// Hides the cursor and prepares for flicker-free updates via `refresh()`.
    /// When `auto_refresh` is enabled, spawns a background thread that
    /// re-renders at `refresh_per_second`. Must have a Console set via
    /// `with_console()` to use the live lifecycle.
    pub fn start(&mut self) {
        if self.is_started() {
            return;
        }
        if let Some(ref console) = self.inner.console {
            console.show_cursor(false);
        }
        self.inner.started.store(true, Ordering::SeqCst);
        *lock_or_recover(&self.inner.height) = 0;
        self.inner.stop_signal.store(false, Ordering::SeqCst);

        // Spawn auto-refresh thread if enabled
        if self.inner.auto_refresh && self.inner.console.is_some() {
            let inner = Arc::clone(&self.inner);
            let interval = Duration::from_secs_f64(1.0 / self.inner.refresh_per_second.max(0.1));
            self.refresh_thread = std::thread::Builder::new()
                .name("fast-rich-progress-refresh".into())
                .spawn(move || loop {
                    if inner.stop_signal.load(Ordering::SeqCst) {
                        break;
                    }
                    std::thread::sleep(interval);
                    if inner.stop_signal.load(Ordering::SeqCst) {
                        break;
                    }
                    refresh_shared(&inner);
                })
                .ok();
        }

        self.refresh();
    }

    /// Stop the live progress display.
    ///
    /// Shows the cursor and optionally clears the output (if transient mode
    /// is enabled). Stops and joins the background refresh thread, if any.
    pub fn stop(&mut self) {
        if !self.is_started() {
            return;
        }

        // Signal stop to any background thread
        self.inner.stop_signal.store(true, Ordering::SeqCst);

        // Join refresh thread if present
        if let Some(handle) = self.refresh_thread.take() {
            let _ = handle.join();
        }

        let current_height = *lock_or_recover(&self.inner.height);

        if let Some(ref console) = self.inner.console {
            // Clear previous output
            if current_height > 0 {
                console.move_cursor_up(current_height as u16);
                for _ in 0..current_height {
                    console.clear_line();
                    console.move_cursor_down(1);
                }
                console.move_cursor_up(current_height as u16);
            }

            if !self.inner.transient {
                // Leave final output visible
                console.print_renderable(self);
                console.newline();
            }

            console.show_cursor(true);
        }

        self.inner.started.store(false, Ordering::SeqCst);
        *lock_or_recover(&self.inner.height) = 0;
    }

    /// Refresh the progress display.
    ///
    /// When used with start/stop lifecycle, provides flicker-free updates.
    /// Safe to call from any thread; rendering is serialized against the
    /// auto-refresh thread.
    pub fn refresh(&self) {
        if let Some(ref console) = self.inner.console {
            if !self.is_started() {
                // Fallback to simple print if not started
                console.print_renderable(self);
                console.newline();
                return;
            }
            refresh_shared(&self.inner);
        } else {
            // No console - use legacy print
            self.print();
        }
    }

    /// Execute a closure with automatic progress lifecycle management.
    ///
    /// Provides RAII-style progress management similar to Python's
    /// `with Progress() as progress:` pattern. Automatically calls
    /// `start()` before the closure and `stop()` after, even if the
    /// closure panics.
    ///
    /// # Example
    /// ```ignore
    /// progress.run(|p| {
    ///     let task = p.add_task("Working", Some(100));
    ///     for i in 0..100 {
    ///         p.advance(task, 1);
    ///         p.refresh();
    ///         thread::sleep(Duration::from_millis(50));
    ///     }
    /// });
    /// ```
    pub fn run<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.start();
        let result = f(self);
        self.stop();
        result
    }

    /// Check if the progress display is currently started.
    pub fn is_started(&self) -> bool {
        self.inner.started.load(Ordering::SeqCst)
    }
}

/// Re-render the live display in place (shared by owner and refresh thread).
fn refresh_shared(inner: &ProgressInner) {
    let Some(ref console) = inner.console else {
        return;
    };
    // Serialize against the owner thread's refresh() calls.
    let _guard = lock_or_recover(&inner.render_lock);
    let current_height = *lock_or_recover(&inner.height);

    // Move cursor up to overwrite previous output
    if current_height > 0 {
        console.move_cursor_up(current_height as u16);
    }

    // Get render context with terminal width
    let width = console.get_width();
    let context = RenderContext {
        width,
        height: None,
    };

    let segments = inner.render(&context);

    // Count physical terminal lines (soft-wrapped rows included)
    let lines = count_terminal_lines(&segments, width);

    // Write segments
    console.write_segments(&segments);

    // Ensure we end with a newline
    if let Some(last) = segments.last() {
        if !last.newline {
            console.newline();
        }
    }

    // Clear any leftover lines from previous render
    if current_height > lines {
        let diff = current_height - lines;
        for _ in 0..diff {
            console.clear_line();
            console.newline();
        }
        console.move_cursor_up(diff as u16);
    }

    *lock_or_recover(&inner.height) = lines;
}

impl ProgressInner {
    /// Render one task row, giving expandable columns their share of the
    /// width left over after the natural widths of all columns.
    fn render_row(&self, task: &Task, total_width: usize) -> Vec<Span> {
        let separators = self.columns.len().saturating_sub(1);
        let naturals: Vec<usize> = self
            .columns
            .iter()
            .map(|c| c.render(task).iter().map(|s| s.width()).sum())
            .collect();
        let natural_total: usize = naturals.iter().sum::<usize>() + separators;

        let expandable: Vec<bool> = self.columns.iter().map(|c| c.is_expandable()).collect();
        let expand_count = expandable.iter().filter(|&&e| e).count();

        let mut widths = naturals.clone();
        if expand_count > 0 {
            let free = total_width.saturating_sub(natural_total);
            if free > 0 {
                let base = free.checked_div(expand_count).unwrap_or(0);
                let mut remainder = free.checked_rem(expand_count).unwrap_or(0);
                for (i, &expandable_i) in expandable.iter().enumerate() {
                    if expandable_i {
                        let mut w = naturals[i] + base;
                        if remainder > 0 {
                            w += 1;
                            remainder -= 1;
                        }
                        widths[i] = w;
                    }
                }
            }
        }

        let mut spans = Vec::new();
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(" "));
            }
            spans.extend(column.render_with_width(task, Some(widths[i])));
        }

        spans
    }
}

impl Renderable for Progress {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        self.inner.render(context)
    }
}

impl Renderable for ProgressInner {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let tasks = lock_or_recover(&self.tasks);
        let mut segments = Vec::new();

        for task in tasks.iter() {
            let spans = self.render_row(task, context.width);
            segments.push(Segment::line(spans));
        }

        segments
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        // Ensure cursor is shown and the refresh thread is stopped if we
        // exit unexpectedly.
        if self.is_started() {
            self.stop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_percentage() {
        let mut task = Task::new(0, "Test", Some(100));
        assert_eq!(task.percentage(), 0.0);

        task.completed = 50;
        assert!((task.percentage() - 0.5).abs() < 0.01);

        task.completed = 100;
        assert!((task.percentage() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_progress_add_task() {
        let progress = Progress::new();
        let id1 = progress.add_task("Task 1", Some(100));
        let id2 = progress.add_task("Task 2", Some(200));

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
    }

    #[test]
    fn test_progress_advance() {
        let progress = Progress::new();
        let id = progress.add_task("Test", Some(100));

        progress.advance(id, 25);
        progress.advance(id, 25);

        let tasks = lock_or_recover(&progress.inner.tasks);
        assert_eq!(tasks[0].completed, 50);
    }

    #[test]
    fn test_expandable_column_gets_free_width() {
        // With an expandable BarColumn, the row should use the full width.
        let progress = Progress::new().with_columns(vec![
            Box::new(TextColumn::new("ab")),
            Box::new(BarColumn::new(10).expand(true)),
        ]);
        let inner = Progress {
            inner: Arc::clone(&progress.inner),
            refresh_thread: None,
        };
        let task = Task::new(0, "ab", Some(100));
        let row = inner.inner.render_row(&task, 50);
        let row_width: usize = row.iter().map(|s| s.width()).sum::<usize>();
        assert_eq!(row_width, 50, "expandable row should fill the given width");
    }

    #[test]
    fn test_auto_refresh_thread_starts_and_stops() {
        let console = Console::capture().width(60);
        let mut progress = Progress::new()
            .with_console(console)
            .refresh_per_second(60.0);
        progress.add_task("Work", Some(100));
        progress.start();
        assert!(progress.is_started());
        // Give the background thread a moment to render at least once.
        std::thread::sleep(Duration::from_millis(60));
        progress.stop();
        assert!(!progress.is_started());
    }

    #[test]
    fn test_progress_bar_render() {
        use crate::progress::columns::BarColumn;
        let bar_col = BarColumn::new(10);
        let mut task = Task::new(0, "Test", Some(100));
        task.completed = 50;

        let spans = bar_col.render(&task);
        // Now we have 3 spans: filled (5), edge pointer (1), unfilled (4)
        assert_eq!(spans.len(), 3);
    }
}
