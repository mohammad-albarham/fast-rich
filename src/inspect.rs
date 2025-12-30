//! Inspect functionality for pretty-printing Rust types.
//!
//! Similar to Rich's `inspect()` function, but adapted for Rust's type system.

use crate::console::RenderContext;
use crate::panel::{BorderStyle, Panel};
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::table::{ColumnAlign, Table};
use crate::text::{Span, Text};
use std::any::type_name;
use std::fmt::Debug;

/// Configuration for inspect output.
#[derive(Debug, Clone)]
pub struct InspectConfig {
    /// Show type name
    pub show_type: bool,
    /// Show value
    pub show_value: bool,
    /// Show size in memory
    pub show_size: bool,
    /// Title for the panel
    pub title: Option<String>,
    /// Maximum depth for nested structures
    pub max_depth: usize,
    /// Whether to use a panel
    pub use_panel: bool,
}

impl Default for InspectConfig {
    fn default() -> Self {
        InspectConfig {
            show_type: true,
            show_value: true,
            show_size: true,
            title: None,
            max_depth: 3,
            use_panel: true,
        }
    }
}

impl InspectConfig {
    /// Create a new config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to show type name.
    pub fn show_type(mut self, show: bool) -> Self {
        self.show_type = show;
        self
    }

    /// Set whether to show value.
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set whether to show size.
    pub fn show_size(mut self, show: bool) -> Self {
        self.show_size = show;
        self
    }

    /// Set the title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set max depth.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Set whether to use a panel.
    pub fn use_panel(mut self, use_panel: bool) -> Self {
        self.use_panel = use_panel;
        self
    }
}

/// Inspection result for a value.
pub struct Inspection {
    /// Type name
    type_name: String,
    /// Debug representation
    debug_value: String,
    /// Size in bytes
    size: usize,
    /// Configuration
    config: InspectConfig,
}

impl Inspection {
    /// Create an inspection of a value.
    pub fn new<T: Debug>(value: &T) -> Self {
        Inspection {
            type_name: type_name::<T>().to_string(),
            debug_value: format!("{:#?}", value),
            size: std::mem::size_of_val(value),
            config: InspectConfig::default(),
        }
    }

    /// Create with config.
    pub fn with_config<T: Debug>(value: &T, config: InspectConfig) -> Self {
        Inspection {
            type_name: type_name::<T>().to_string(),
            debug_value: format!("{:#?}", value),
            size: std::mem::size_of_val(value),
            config,
        }
    }

    fn build_content(&self) -> Text {
        let mut text = Text::new();

        if self.config.show_type {
            text.push_styled("Type: ", Style::new().foreground(Color::Cyan));
            text.push_styled(
                format!("{}\n", self.type_name),
                Style::new().foreground(Color::Yellow),
            );
        }

        if self.config.show_size {
            text.push_styled("Size: ", Style::new().foreground(Color::Cyan));
            text.push_styled(
                format!("{} bytes\n", self.size),
                Style::new().foreground(Color::Green),
            );
        }

        if self.config.show_value {
            text.push_styled("Value:\n", Style::new().foreground(Color::Cyan));

            // Truncate if too long
            let max_lines = 20;
            let lines: Vec<&str> = self.debug_value.lines().collect();

            if lines.len() > max_lines {
                for line in &lines[..max_lines] {
                    text.push(format!("{}\n", line));
                }
                text.push_styled(
                    format!("... and {} more lines", lines.len() - max_lines),
                    Style::new().foreground(Color::BrightBlack).italic(),
                );
            } else {
                text.push(self.debug_value.clone());
            }
        }

        text
    }
}

impl Renderable for Inspection {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let content = self.build_content();

        if self.config.use_panel {
            let title = self.config.title.clone().unwrap_or_else(|| {
                // Just use the type name without module path for title
                self.type_name
                    .rsplit("::")
                    .next()
                    .unwrap_or(&self.type_name)
                    .to_string()
            });

            let panel = Panel::new(content)
                .title(&title)
                .border_style(BorderStyle::Rounded)
                .style(Style::new().foreground(Color::BrightBlack));

            panel.render(context)
        } else {
            content.render(context)
        }
    }
}

/// Inspect a value and return a renderable.
pub fn inspect<T: Debug>(value: &T) -> Inspection {
    Inspection::new(value)
}

/// Inspect a value with custom configuration.
pub fn inspect_with_config<T: Debug>(value: &T, config: InspectConfig) -> Inspection {
    Inspection::with_config(value, config)
}

/// Print inspection of a value to the console.
pub fn print_inspect<T: Debug>(value: &T) {
    let console = crate::Console::new();
    let inspection = inspect(value);
    console.print_renderable(&inspection);
}

/// Macro for inspecting with automatic variable name as title.
#[macro_export]
macro_rules! inspect {
    ($value:expr) => {{
        let config = $crate::inspect::InspectConfig::new().title(stringify!($value));
        $crate::inspect::inspect_with_config(&$value, config)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_primitive() {
        let value = 42i32;
        let inspection = inspect(&value);

        assert!(inspection.type_name.contains("i32"));
        assert_eq!(inspection.size, 4);
        assert!(inspection.debug_value.contains("42"));
    }

    #[test]
    fn test_inspect_struct() {
        #[derive(Debug)]
        struct TestStruct {
            name: String,
            value: i32,
        }

        let value = TestStruct {
            name: "test".to_string(),
            value: 123,
        };

        let inspection = inspect(&value);
        assert!(inspection.type_name.contains("TestStruct"));
        assert!(inspection.debug_value.contains("test"));
        assert!(inspection.debug_value.contains("123"));
    }

    #[test]
    fn test_inspect_vec() {
        let value = vec![1, 2, 3, 4, 5];
        let inspection = inspect(&value);

        assert!(inspection.type_name.contains("Vec"));
        assert!(inspection.debug_value.contains("1"));
        assert!(inspection.debug_value.contains("5"));
    }

    #[test]
    fn test_inspect_config() {
        let value = 42;
        let config = InspectConfig::new()
            .show_type(true)
            .show_size(false)
            .title("My Value");

        let inspection = inspect_with_config(&value, config);
        // Config should be applied
        assert!(!inspection.config.show_size);
    }

    #[test]
    fn test_inspect_render() {
        let value = vec!["hello", "world"];
        let inspection = inspect(&value);

        let context = RenderContext { width: 60 };
        let segments = inspection.render(&context);

        // Should produce some output
        assert!(!segments.is_empty());
    }
}
