use crate::console::RenderContext;
use crate::progress::columns::{BarColumn, PercentageColumn, ProgressColumn, TextColumn, TimeRemainingColumn};
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::Span;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

/// Multi-task progress display.
#[derive(Debug)]
pub struct Progress {
    /// Tasks being tracked
    tasks: Arc<Mutex<Vec<Task>>>,
    /// Next task ID
    next_id: Arc<Mutex<usize>>,
    /// Columns to display
    columns: Vec<Box<dyn ProgressColumn>>,
    /// Whether to show the progress
    #[allow(dead_code)]
    visible: bool,
    /// Refresh rate in milliseconds
    #[allow(dead_code)]
    refresh_rate_ms: u64,
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
            tasks: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(0)),
            columns: vec![
                Box::new(TextColumn::new("[progress.description]")),
                Box::new(BarColumn::new(40)),
                Box::new(PercentageColumn::new()),
                Box::new(TimeRemainingColumn),
            ],
            visible: true,
            refresh_rate_ms: 100,
        }
    }

    /// Set custom columns.
    pub fn with_columns(mut self, columns: Vec<Box<dyn ProgressColumn>>) -> Self {
        self.columns = columns;
        self
    }

    /// Add a new task.
    pub fn add_task(&self, description: &str, total: Option<u64>) -> usize {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;

        let task = Task::new(id, description, total);
        self.tasks.lock().unwrap().push(task);

        id
    }

    /// Advance a task by the given amount.
    pub fn advance(&self, task_id: usize, amount: u64) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                task.completed += amount;
                if let Some(total) = task.total {
                    if task.completed >= total {
                        task.finished = true;
                    }
                }
            }
        }
    }

    /// Update a task's completed count.
    pub fn update(&self, task_id: usize, completed: u64) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                task.completed = completed;
                if let Some(total) = task.total {
                    if task.completed >= total {
                        task.finished = true;
                    }
                }
            }
        }
    }

    /// Mark a task as finished.
    pub fn finish(&self, task_id: usize) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                task.finished = true;
            }
        }
    }

    /// Remove a task.
    pub fn remove(&self, task_id: usize) {
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.retain(|t| t.id != task_id);
        }
    }

    /// Check if all tasks are finished.
    pub fn is_finished(&self) -> bool {
        self.tasks
            .lock()
            .map(|tasks| tasks.iter().all(|t| t.finished))
            .unwrap_or(true)
    }

    /// Render the progress display.
    pub fn render_to_string(&self) -> String {
        let context = RenderContext { width: 80, height: None };
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
    pub fn print(&self) {
        let output = self.render_to_string();

        // Move cursor up and clear lines for update
        let tasks = self.tasks.lock().unwrap();
        let num_lines = tasks.len();
        drop(tasks);

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
}

impl Renderable for Progress {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        let tasks = self.tasks.lock().unwrap();
        let mut segments = Vec::new();

        for task in tasks.iter() {
            let mut spans = Vec::new();

            for (i, column) in self.columns.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::raw(" "));
                }
                spans.extend(column.render(task));
            }

            segments.push(Segment::line(spans));
        }

        segments
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

        let tasks = progress.tasks.lock().unwrap();
        assert_eq!(tasks[0].completed, 50);
    }

    #[test]
    fn test_progress_bar_render() {
        use crate::progress::columns::BarColumn;
        let bar_col = BarColumn::new(10);
        let mut task = Task::new(0, "Test", Some(100));
        task.completed = 50;

        let spans = bar_col.render(&task);
        assert_eq!(spans.len(), 2);
    }
}
