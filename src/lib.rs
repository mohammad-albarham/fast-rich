//! # rich-rust
//!
//! A Rust port of Python's [Rich](https://github.com/Textualize/rich) library
//! for beautiful terminal formatting.
//!
//! ## Features
//!
//! - **Rich text** with colors, styles, and markup
//! - **Tables** with Unicode borders and auto-sizing
//! - **Progress bars** with multiple tasks and spinners
//! - **Tree views** for hierarchical data
//! - **Panels** and **Rules** for visual organization
//! - **Markdown** rendering (optional)
//! - **Syntax highlighting** (optional)
//! - **Pretty tracebacks** for better error display
//!
//! ## Quick Start
//!
//! ```no_run
//! use rich_rust::prelude::*;
//!
//! let console = Console::new();
//!
//! // Simple styled output
//! console.print("Hello, [bold magenta]World[/]!");
//!
//! // Tables
//! let mut table = Table::new();
//! table.add_column("Name");
//! table.add_column("Age");
//! table.add_row_strs(&["Alice", "30"]);
//! console.print_renderable(&table);
//! ```

// Core modules
pub mod console;
pub mod emoji;
pub mod markup;
pub mod renderable;
pub mod style;
pub mod text;

// Renderables
pub mod columns;
pub mod filesize;
pub mod layout;
pub mod live;
pub mod log;
pub mod panel;
pub mod rule;
pub mod table;
pub mod tree;

// Progress
pub mod progress;

// Utilities
pub mod inspect;
pub mod prompt;
pub mod traceback;

// Optional feature-gated modules
#[cfg(feature = "markdown")]
pub mod markdown;

#[cfg(feature = "syntax")]
pub mod syntax;

// Re-exports for convenience
pub use console::Console;
pub use panel::{BorderStyle, Panel};
pub use renderable::Renderable;
pub use rule::Rule;
pub use style::{Color, Style};
pub use table::{Column, ColumnAlign, Table};
pub use text::{Alignment, Text};
pub use tree::{Tree, TreeNode};

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::columns::Columns;
    pub use crate::console::Console;
    pub use crate::inspect::{inspect, InspectConfig};
    pub use crate::log::ConsoleLog;
    pub use crate::panel::{BorderStyle, Panel};
    pub use crate::progress::{track, Progress, ProgressBar, Spinner, SpinnerStyle, Status};
    pub use crate::renderable::Renderable;
    pub use crate::rule::Rule;
    pub use crate::style::{Color, Style};
    pub use crate::table::{Column, ColumnAlign, Table};
    pub use crate::text::{Alignment, Text};
    pub use crate::traceback::install_panic_hook;
    pub use crate::tree::{GuideStyle, Tree, TreeNode};
}

/// Print text with markup to stdout.
///
/// This is a convenience function for quick output.
///
/// # Example
///
/// ```no_run
/// rich_rust::print("[bold green]Success![/]");
/// ```
pub fn print(content: &str) {
    Console::new().print(content);
}

/// Print text with markup to stdout, followed by a newline.
pub fn println(content: &str) {
    Console::new().println(content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_creation() {
        let console = Console::new();
        assert!(console.get_width() > 0);
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new().foreground(Color::Red).bold().underline();

        assert!(style.bold);
        assert!(style.underline);
    }

    #[test]
    fn test_text_creation() {
        let text = Text::plain("Hello, World!");
        assert_eq!(text.plain_text(), "Hello, World!");
    }

    #[test]
    fn test_table_creation() {
        let mut table = Table::new();
        table.add_column("Col1");
        table.add_column("Col2");
        table.add_row_strs(&["a", "b"]);

        // Table should have columns and rows
        assert!(!table
            .render(&console::RenderContext { width: 40 })
            .is_empty());
    }
}
