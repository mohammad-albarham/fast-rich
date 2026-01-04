use crate::progress::Task;
// use crate::progress::bar::ProgressBar;
use crate::progress::spinner::{Spinner, SpinnerStyle};
use crate::style::{Color, Style};
use crate::text::Span;
use std::fmt::Debug;
use std::time::Duration;

/// A trait for rendering a column in a progress bar.
pub trait ProgressColumn: Send + Sync + Debug {
    /// Render the column for the given task.
    fn render(&self, task: &Task) -> Vec<Span>;
}

/// Renders a static text string or text based on task properties.
#[derive(Debug)]
pub struct TextColumn {
    text: String,
    style: Style,
}

impl TextColumn {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: Style::new(),
        }
    }

    pub fn styled(text: &str, style: Style) -> Self {
        Self {
            text: text.to_string(),
            style,
        }
    }
}

impl ProgressColumn for TextColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        // Simple interpolation for now task.description
        let text = if self.text == "[progress.description]" {
            &task.description
        } else {
            &self.text
        };

        vec![Span::styled(text.clone(), self.style)]
    }
}

/// Renders the progress bar with distinct filled/unfilled characters.
/// 
/// Uses Unicode box-drawing characters for clear visual distinction:
/// - `━` (U+2501) for filled portion
/// - `╸` (U+257A) for edge pointer (shows progress position)
/// - `─` (U+2500) for unfilled portion
/// 
/// For indeterminate tasks (no total), shows a pulsing animation.
#[derive(Debug)]
pub struct BarColumn {
    /// Width of the bar in characters
    pub bar_width: usize,
    /// Character for filled portion (default: '━')
    pub complete_char: char,
    /// Character for unfilled portion (default: '─')
    pub incomplete_char: char,
    /// Optional edge pointer character (default: Some('╸'))
    pub edge_char: Option<char>,
    /// Style for completed portion
    pub complete_style: Style,
    /// Style for finished tasks
    pub finished_style: Option<Style>,
    /// Style for unfilled portion
    pub incomplete_style: Style,
    /// Style for pulse animation (indeterminate)
    pub pulse_style: Style,
}

impl Default for BarColumn {
    fn default() -> Self {
        Self::new(40)
    }
}

impl BarColumn {
    pub fn new(bar_width: usize) -> Self {
        Self {
            bar_width,
            complete_char: '━',      // Heavy horizontal
            incomplete_char: '─',    // Light horizontal (distinct!)
            edge_char: Some('╸'),    // Heavy left (pointer)
            complete_style: Style::new().foreground(Color::Magenta),
            finished_style: Some(Style::new().foreground(Color::Green)),
            incomplete_style: Style::new().foreground(Color::Ansi256(237)), // Dark grey
            pulse_style: Style::new().foreground(Color::Cyan),
        }
    }

    /// Set the complete character
    pub fn complete_char(mut self, c: char) -> Self {
        self.complete_char = c;
        self
    }

    /// Set the incomplete character
    pub fn incomplete_char(mut self, c: char) -> Self {
        self.incomplete_char = c;
        self
    }

    /// Set the edge pointer character (or None to disable)
    pub fn edge_char(mut self, c: Option<char>) -> Self {
        self.edge_char = c;
        self
    }

    /// Set the style for completed portion
    pub fn complete_style(mut self, style: Style) -> Self {
        self.complete_style = style;
        self
    }

    /// Set the style for finished tasks
    pub fn finished_style(mut self, style: Option<Style>) -> Self {
        self.finished_style = style;
        self
    }

    /// Render a pulsing bar for indeterminate progress
    fn render_pulse(&self, task: &Task) -> Vec<Span> {
        let width = self.bar_width;
        let pulse_width = 6.min(width / 3); // Pulse is ~1/3 of bar width
        
        // Calculate pulse position based on elapsed time
        let elapsed_ms = task.elapsed().as_millis() as usize;
        let cycle_duration_ms = 1500; // 1.5 seconds per cycle
        let position_in_cycle = elapsed_ms % cycle_duration_ms;
        
        // Pulse moves from left to right and back
        let half_cycle = cycle_duration_ms / 2;
        let normalized_pos = if position_in_cycle < half_cycle {
            position_in_cycle as f64 / half_cycle as f64
        } else {
            1.0 - ((position_in_cycle - half_cycle) as f64 / half_cycle as f64)
        };
        
        let pulse_start = ((width - pulse_width) as f64 * normalized_pos).round() as usize;
        let pulse_end = pulse_start + pulse_width;
        
        let mut spans = Vec::new();
        
        // Before pulse
        if pulse_start > 0 {
            spans.push(Span::styled(
                self.incomplete_char.to_string().repeat(pulse_start),
                self.incomplete_style,
            ));
        }
        
        // Pulse itself
        spans.push(Span::styled(
            self.complete_char.to_string().repeat(pulse_width),
            self.pulse_style,
        ));
        
        // After pulse
        let after_pulse = width.saturating_sub(pulse_end);
        if after_pulse > 0 {
            spans.push(Span::styled(
                self.incomplete_char.to_string().repeat(after_pulse),
                self.incomplete_style,
            ));
        }
        
        spans
    }
}

impl ProgressColumn for BarColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        // Handle indeterminate progress (no total)
        if task.total.is_none() && !task.finished {
            return self.render_pulse(task);
        }

        let total = task.total.unwrap_or(100) as f64;
        let completed = task.completed as f64;
        let percentage = (completed / total).clamp(0.0, 1.0);

        let width = self.bar_width;
        
        let style = if task.finished {
            self.finished_style.unwrap_or(self.complete_style)
        } else {
            self.complete_style
        };

        // Calculate filled width, accounting for optional edge character
        let has_edge = self.edge_char.is_some() && !task.finished && percentage < 1.0;
        let effective_width = if has_edge { width.saturating_sub(1) } else { width };
        
        let filled_width = (effective_width as f64 * percentage).round() as usize;
        let empty_width = effective_width.saturating_sub(filled_width);

        let mut spans = Vec::new();
        
        // Filled portion
        if filled_width > 0 {
            spans.push(Span::styled(
                self.complete_char.to_string().repeat(filled_width),
                style,
            ));
        }
        
        // Edge pointer (shows current progress position)
        if has_edge && filled_width < width {
            if let Some(edge) = self.edge_char {
                spans.push(Span::styled(edge.to_string(), style));
            }
        }
        
        // Unfilled portion (using DIFFERENT character)
        if empty_width > 0 {
            spans.push(Span::styled(
                self.incomplete_char.to_string().repeat(empty_width),
                self.incomplete_style,
            ));
        }
        
        spans
    }
}

/// Renders the percentage complete (e.g. "50%").
#[derive(Debug)]
pub struct PercentageColumn(pub Style);

impl Default for PercentageColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl PercentageColumn {
    pub fn new() -> Self {
        Self(Style::new().foreground(Color::Cyan))
    }
}

impl ProgressColumn for PercentageColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let percentage = task.percentage() * 100.0;
        vec![Span::styled(format!("{:>3.0}%", percentage), self.0)]
    }
}

/// Renders the spinner
#[derive(Debug)]
pub struct SpinnerColumn {
    spinner: Spinner, // Use generic spinner for frames
}

impl Default for SpinnerColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl SpinnerColumn {
    pub fn new() -> Self {
        Self {
            spinner: Spinner::new("").style(SpinnerStyle::Dots),
        }
    }
}

impl ProgressColumn for SpinnerColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        // We use the task's elapsed time to calculate the frame
        // This keeps it stateless with respect to the column, but animated by the task's lifetime.
        // For a global spinner independent of task start, we might need a shared start time.
        // But usually spinners in task rows indicate THAT task's activity.

        // However, generic Spinner uses its own start_time.
        // We should probably rely on `SpinnerStyle` and manual calculation using task.elapsed()
        // to avoid storing state that drifts.

        // Let's copy logic from Spinner::current_frame but use task.elapsed()
        let style = self.spinner.get_style();
        let interval = style.interval_ms();
        let frames = style.frames();
        let elapsed_ms = task.elapsed().as_millis() as u64;
        let idx = ((elapsed_ms / interval) as usize) % frames.len();

        vec![Span::styled(
            frames[idx].to_string(),
            Style::new().foreground(Color::Green),
        )]
    }
}

/// Renders transfer speed
#[derive(Debug)]
pub struct TransferSpeedColumn;

impl ProgressColumn for TransferSpeedColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let speed = task.speed();
        let speed_str = if speed >= 1_000_000.0 {
            format!("{:.1} MB/s", speed / 1_000_000.0)
        } else if speed >= 1_000.0 {
            format!("{:.1} KB/s", speed / 1_000.0)
        } else {
            format!("{:.0} B/s", speed)
        };
        vec![Span::styled(speed_str, Style::new().foreground(Color::Red))]
    }
}

/// Renders time remaining
#[derive(Debug)]
pub struct TimeRemainingColumn;

impl ProgressColumn for TimeRemainingColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let eta = match task.eta() {
            Some(d) => format_duration(d),
            None => "-:--:--".to_string(),
        };
        vec![Span::styled(eta, Style::new().foreground(Color::Cyan))]
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    if secs >= 3600 {
        format!(
            "{:02}:{:02}:{:02}",
            secs / 3600,
            (secs % 3600) / 60,
            secs % 60
        )
    } else {
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }
}

#[derive(Debug)]
pub struct MofNColumn {
    separator: String,
}

impl Default for MofNColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl MofNColumn {
    pub fn new() -> Self {
        Self {
            separator: "/".to_string(),
        }
    }
}

impl ProgressColumn for MofNColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let completed = task.completed;
        let total = task.total.unwrap_or(0);
        vec![Span::styled(
            format!("{}{}{}", completed, self.separator, total),
            Style::new().foreground(Color::Green),
        )]
    }
}

#[derive(Debug)]
pub struct ElapsedColumn;

impl ProgressColumn for ElapsedColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let elapsed = task.elapsed();
        vec![Span::styled(
            format_duration(elapsed),
            Style::new().foreground(Color::Cyan),
        )]
    }
}
