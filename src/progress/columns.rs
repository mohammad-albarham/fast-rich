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

    /// Render the column with an available width hint.
    ///
    /// Columns that can expand (like BarColumn with `expand: true`) should
    /// override this to use the provided width. Default implementation
    /// ignores the width and calls `render()`.
    fn render_with_width(&self, task: &Task, _available_width: Option<usize>) -> Vec<Span> {
        self.render(task)
    }

    /// Returns true if this column should expand to fill available width.
    fn is_expandable(&self) -> bool {
        false
    }

    /// Get the minimum width this column needs.
    fn min_width(&self) -> usize {
        0
    }
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
/// 
/// Use `expand(true)` to make the bar expand to fill available width.
/// Use `use_sub_blocks(true)` for smoother progress using 8th-block Unicode characters.
#[derive(Debug)]
pub struct BarColumn {
    /// Width of the bar in characters (used as min_width when expand is true)
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
    /// If true, expand to fill available width
    pub expand: bool,
    /// If true, use sub-character (8th-block) Unicode for smoother progress
    pub use_sub_blocks: bool,
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
            expand: false,
            use_sub_blocks: false,
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

    /// Set whether the bar should expand to fill available width.
    ///
    /// When expand is true, the bar will use the remaining terminal width
    /// after other columns are rendered. The `bar_width` becomes the minimum width.
    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Enable sub-character progress using 8th-block Unicode characters.
    ///
    /// When enabled, uses characters like ▏▎▍▌▋▊▉█ for smoother
    /// progress rendering with 8 levels of precision per character.
    pub fn use_sub_blocks(mut self, enabled: bool) -> Self {
        self.use_sub_blocks = enabled;
        self
    }

    /// Get the sub-block character for a given fraction (0-7).
    /// Returns full blocks for full progress, partial blocks otherwise.
    fn sub_block_char(eighths: usize) -> char {
        match eighths {
            0 => ' ',       // Empty
            1 => '▏',       // 1/8 block (U+258F)
            2 => '▎',       // 2/8 block (U+258E)
            3 => '▍',       // 3/8 block (U+258D)
            4 => '▌',       // 4/8 block (U+258C)
            5 => '▋',       // 5/8 block (U+258B)
            6 => '▊',       // 6/8 block (U+258A)
            7 => '▉',       // 7/8 block (U+2589)
            _ => '█',       // Full block (U+2588)
        }
    }

    /// Render the bar with a specific width
    fn render_bar(&self, task: &Task, width: usize) -> Vec<Span> {
        // Handle indeterminate progress (no total)
        if task.total.is_none() && !task.finished {
            return self.render_pulse_with_width(task, width);
        }

        let total = task.total.unwrap_or(100) as f64;
        let completed = task.completed as f64;
        let percentage = (completed / total).clamp(0.0, 1.0);

        let style = if task.finished {
            self.finished_style.unwrap_or(self.complete_style)
        } else {
            self.complete_style
        };

        // Use sub-block rendering if enabled
        if self.use_sub_blocks && !task.finished && percentage < 1.0 {
            return self.render_sub_blocks(percentage, width, style);
        }

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

    /// Render progress bar with sub-character (8th-block) precision.
    fn render_sub_blocks(&self, percentage: f64, width: usize, style: Style) -> Vec<Span> {
        // Calculate the exact fractional position
        let exact_filled = width as f64 * percentage;
        let full_blocks = exact_filled as usize;
        let fraction = exact_filled - full_blocks as f64;
        let eighths = (fraction * 8.0).round() as usize;
        
        let mut spans = Vec::new();

        // Full filled blocks (using block character)
        if full_blocks > 0 {
            spans.push(Span::styled(
                "█".repeat(full_blocks),
                style,
            ));
        }

        // Partial block for fractional progress
        if eighths > 0 && full_blocks < width {
            spans.push(Span::styled(
                Self::sub_block_char(eighths).to_string(),
                style,
            ));
        }

        // Empty portion
        let used_width = full_blocks + if eighths > 0 { 1 } else { 0 };
        let empty_width = width.saturating_sub(used_width);
        if empty_width > 0 {
            spans.push(Span::styled(
                " ".repeat(empty_width),
                self.incomplete_style,
            ));
        }

        spans
    }

    /// Render pulse animation with a specific width
    fn render_pulse_with_width(&self, task: &Task, width: usize) -> Vec<Span> {
        let pulse_width = 6.min(width / 3);
        
        let elapsed_ms = task.elapsed().as_millis() as usize;
        let cycle_duration_ms = 1500;
        let position_in_cycle = elapsed_ms % cycle_duration_ms;
        
        let half_cycle = cycle_duration_ms / 2;
        let normalized_pos = if position_in_cycle < half_cycle {
            position_in_cycle as f64 / half_cycle as f64
        } else {
            1.0 - ((position_in_cycle - half_cycle) as f64 / half_cycle as f64)
        };
        
        let pulse_start = ((width.saturating_sub(pulse_width)) as f64 * normalized_pos).round() as usize;
        let pulse_end = pulse_start + pulse_width;
        
        let mut spans = Vec::new();
        
        if pulse_start > 0 {
            spans.push(Span::styled(
                self.incomplete_char.to_string().repeat(pulse_start),
                self.incomplete_style,
            ));
        }
        
        spans.push(Span::styled(
            self.complete_char.to_string().repeat(pulse_width),
            self.pulse_style,
        ));
        
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
        self.render_bar(task, self.bar_width)
    }

    fn render_with_width(&self, task: &Task, available_width: Option<usize>) -> Vec<Span> {
        let width = if self.expand {
            available_width.unwrap_or(self.bar_width).max(self.bar_width)
        } else {
            self.bar_width
        };
        self.render_bar(task, width)
    }

    fn is_expandable(&self) -> bool {
        self.expand
    }

    fn min_width(&self) -> usize {
        self.bar_width
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

/// Formats bytes into human-readable size string (e.g., "1.2 MB").
fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GB {
        format!("{:.1} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.1} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.1} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

/// Renders the completed file size (e.g., "1.2 MB").
///
/// Displays the current progress in human-readable bytes.
#[derive(Debug)]
pub struct FileSizeColumn {
    style: Style,
}

impl Default for FileSizeColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSizeColumn {
    pub fn new() -> Self {
        Self {
            style: Style::new().foreground(Color::Green),
        }
    }

    /// Set the style for the file size display.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl ProgressColumn for FileSizeColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let size_str = format_bytes(task.completed);
        vec![Span::styled(size_str, self.style)]
    }
}

/// Renders the file size as "completed / total" (e.g., "1.2 MB / 5.0 MB").
///
/// Shows both completed and total bytes in a single display.
#[derive(Debug)]
pub struct TotalFileSizeColumn {
    separator: String,
    completed_style: Style,
    total_style: Style,
}

impl Default for TotalFileSizeColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl TotalFileSizeColumn {
    pub fn new() -> Self {
        Self {
            separator: " / ".to_string(),
            completed_style: Style::new().foreground(Color::Green),
            total_style: Style::new().foreground(Color::Blue),
        }
    }

    /// Set the separator between completed and total (default: " / ").
    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    /// Set the style for completed bytes.
    pub fn completed_style(mut self, style: Style) -> Self {
        self.completed_style = style;
        self
    }

    /// Set the style for total bytes.
    pub fn total_style(mut self, style: Style) -> Self {
        self.total_style = style;
        self
    }
}

impl ProgressColumn for TotalFileSizeColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let completed_str = format_bytes(task.completed);
        let total_str = match task.total {
            Some(t) => format_bytes(t),
            None => "?".to_string(),
        };

        vec![
            Span::styled(completed_str, self.completed_style),
            Span::styled(self.separator.clone(), Style::new()),
            Span::styled(total_str, self.total_style),
        ]
    }
}

/// Renders a combined download display: "size @ speed" (e.g., "1.2 MB @ 500 KB/s").
///
/// Combines file size progress with transfer speed in one compact column.
#[derive(Debug)]
pub struct DownloadColumn {
    size_style: Style,
    speed_style: Style,
}

impl Default for DownloadColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl DownloadColumn {
    pub fn new() -> Self {
        Self {
            size_style: Style::new().foreground(Color::Green),
            speed_style: Style::new().foreground(Color::Red),
        }
    }

    /// Set the style for file size.
    pub fn size_style(mut self, style: Style) -> Self {
        self.size_style = style;
        self
    }

    /// Set the style for transfer speed.
    pub fn speed_style(mut self, style: Style) -> Self {
        self.speed_style = style;
        self
    }
}

impl ProgressColumn for DownloadColumn {
    fn render(&self, task: &Task) -> Vec<Span> {
        let size_str = format_bytes(task.completed);
        let speed = task.speed();
        let speed_str = if speed >= 1_000_000.0 {
            format!("{:.1} MB/s", speed / 1_000_000.0)
        } else if speed >= 1_000.0 {
            format!("{:.1} KB/s", speed / 1_000.0)
        } else {
            format!("{:.0} B/s", speed)
        };

        vec![
            Span::styled(size_str, self.size_style),
            Span::raw(" @ "),
            Span::styled(speed_str, self.speed_style),
        ]
    }
}
