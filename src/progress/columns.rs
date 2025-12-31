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

/// Renders the progress bar.
#[derive(Debug)]
pub struct BarColumn {
    pub bar_width: usize,
    pub complete_style: Style,
    pub finished_style: Option<Style>,
    pub pulse_style: Option<Style>,
}

impl BarColumn {
    pub fn new(bar_width: usize) -> Self {
        Self {
            bar_width,
            complete_style: Style::new().foreground(Color::Magenta), // Default rich color
            finished_style: Some(Style::new().foreground(Color::Green)),
            pulse_style: None,
        }
    }
}

impl ProgressColumn for BarColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let total = task.total.unwrap_or(100) as f64;
        let completed = task.completed as f64;
        let percentage = (completed / total).min(1.0).max(0.0);
        
        let width = self.bar_width;
        let filled_width = (width as f64 * percentage).round() as usize;
        let empty_width = width.saturating_sub(filled_width);
        
        let style = if task.finished {
            self.finished_style.unwrap_or(self.complete_style)
        } else {
            self.complete_style
        };

        let mut spans = Vec::new();
        if filled_width > 0 {
             spans.push(Span::styled("━".repeat(filled_width), style));
        }
        if empty_width > 0 {
             spans.push(Span::styled("━".repeat(empty_width), Style::new().foreground(Color::Ansi256(237)))); // Grey
        }
        spans
    }
}

/// Renders the percentage complete (e.g. "50%").
#[derive(Debug)]
pub struct PercentageColumn(pub Style);

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
        
        vec![Span::styled(frames[idx].to_string(), Style::new().foreground(Color::Green))]
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
        format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
    } else {
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }
}

#[derive(Debug)]
pub struct MofNColumn {
    separator: String
}

impl MofNColumn {
    pub fn new() -> Self {
        Self { separator: "/".to_string() }
    }
}

impl ProgressColumn for MofNColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let completed = task.completed;
        let total = task.total.unwrap_or(0);
        vec![Span::styled(format!("{}{}{}", completed, self.separator, total), Style::new().foreground(Color::Green))]
    }
}

#[derive(Debug)]
pub struct ElapsedColumn;

impl ProgressColumn for ElapsedColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let elapsed = task.elapsed();
        vec![Span::styled(format_duration(elapsed), Style::new().foreground(Color::Cyan))]
    }
}
