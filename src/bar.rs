//! Bar charts for visualizing data.
//!
//! Provides horizontal bar charts with customizable colors and widths.

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::Span;

/// A single bar in a bar chart.
#[derive(Debug, Clone)]
pub struct BarData {
    /// Label for this bar
    pub label: String,
    /// Value (will be scaled to fit width)
    pub value: f64,
    /// Optional color for this bar
    pub color: Option<Color>,
}

impl BarData {
    /// Create a new bar with label and value.
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        BarData {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set the color for this bar.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// A horizontal bar chart.
#[derive(Debug, Clone)]
pub struct BarChart {
    bars: Vec<BarData>,
    width: Option<usize>,
    bar_char: char,
    default_color: Color,
    show_values: bool,
}

impl BarChart {
    /// Create a new empty bar chart.
    pub fn new() -> Self {
        BarChart {
            bars: Vec::new(),
            width: None,
            bar_char: '█',
            default_color: Color::Green,
            show_values: true,
        }
    }

    /// Add a bar to the chart.
    pub fn add_bar(&mut self, bar: BarData) -> &mut Self {
        self.bars.push(bar);
        self
    }

    /// Add a simple bar with label and value.
    pub fn bar(&mut self, label: impl Into<String>, value: f64) -> &mut Self {
        self.add_bar(BarData::new(label, value));
        self
    }

    /// Set the width for bars (excluding label).
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the character used for bars.
    pub fn bar_char(mut self, c: char) -> Self {
        self.bar_char = c;
        self
    }

    /// Set the default color for bars.
    pub fn default_color(mut self, color: Color) -> Self {
        self.default_color = color;
        self
    }

    /// Set whether to show values next to bars.
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }
}

impl Default for BarChart {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for BarChart {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if self.bars.is_empty() {
            return vec![Segment::empty_line()];
        }

        // Find max value for scaling
        let max_value = self.bars.iter().map(|b| b.value).fold(0.0, f64::max);
        if max_value == 0.0 {
            return vec![Segment::empty_line()];
        }

        // Find max label width
        let max_label_width = self.bars.iter().map(|b| b.label.len()).max().unwrap_or(0);

        // Calculate bar width
        let value_width = if self.show_values { 12 } else { 0 }; // Space for value display
        let bar_width = self
            .width
            .unwrap_or_else(|| context.width.saturating_sub(max_label_width + 3 + value_width));

        let mut segments = Vec::new();

        for bar in &self.bars {
            // Calculate bar length
            let bar_length = ((bar.value / max_value) * bar_width as f64).round() as usize;
            let bar_length = bar_length.min(bar_width);

            // Choose color
            let color = bar.color.unwrap_or(self.default_color);
            let style = Style::new().foreground(color);

            let mut spans = Vec::new();

            // Label (left-aligned, padded)
            let label_padded = format!("{:<width$}", bar.label, width = max_label_width);
            spans.push(Span::styled(label_padded, Style::new().dim()));
            spans.push(Span::raw(" "));

            // Bar
            let bar_str = self.bar_char.to_string().repeat(bar_length);
            spans.push(Span::styled(bar_str, style));

            // Value (if enabled)
            if self.show_values {
                let remaining = bar_width.saturating_sub(bar_length);
                spans.push(Span::raw(" ".repeat(remaining.max(1))));
                spans.push(Span::styled(
                    format!("{:.1}", bar.value),
                    Style::new().foreground(Color::Cyan),
                ));
            }

            segments.push(Segment::line(spans));
        }

        segments
    }
}
