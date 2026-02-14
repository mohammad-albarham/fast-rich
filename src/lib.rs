//! # fast-rich
//!
//! A Rust port of Python's [Rich](https://github.com/Textualize/rich) library
//! for beautiful terminal formatting.
//!
//! ## Features
//!
//! - **Rich text** with colors, styles, and markup
//! - **Tables** with Unicode borders and auto-sizing
//! - **Progress bars** with multiple tasks, spinners, and customizable columns
//! - **Live Display** for flicker-free auto-updating content
//! - **Logging** handler for colorful structured logs
//! - **Tree views** for hierarchical data
//! - **Panels** and **Rules** for visual organization
//! - **Markdown** rendering (optional)
//! - **Syntax highlighting** (optional)
//! - **Pretty tracebacks** for better error display
//!
//! ## Quick Start
//!
//! ### Drop-in Print Replacement
//!
//! ```no_run
//! // Shadow standard print macros with rich versions
//! use fast_rich::{print, println};
//!
//! println!("[bold magenta]Hello, World![/]");
//! println!("Player {} scored [yellow]{}[/] points", "Alice", 100);
//! ```
//!
//! ### Using Console Directly
//!
//! ```no_run
//! use fast_rich::prelude::*;
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
//!
//! ## Markup Syntax
//!
//! The print macros support Rich markup syntax:
//!
//! - `[bold]text[/]` - Bold text
//! - `[red]text[/]` - Colored text
//! - `[bold red on blue]text[/]` - Combined styles
//! - `[[` and `]]` - Escaped brackets (literal `[` and `]`)
//!
//! ## Important: Bracket Handling
//!
//! The markup parser uses a smart heuristic to distinguish between style tags
//! and data brackets:
//!
//! - **Valid Tags**: `[bold]`, `[red]`, `[link=url]` -> Parsed as style
//! - **Data**: `[1, 2, 3]`, `[Unknown]` -> Printed literally as text
//!
//! This means standard debug output usually works out of the box:
//!
//! ```no_run
//! use fast_rich::println;
//!
//! let data = vec![1, 2, 3];
//! println!("Data: {:?}", data); // Prints: Data: [1, 2, 3]
//! ```
//!
//! However, there is a **Known Limitation**: if your data looks exactly like a style tag,
//! the parser will consume it.
//!
//! ```no_run
//! // ⚠️ Unintended Tag Collision
//! let colors = vec!["red", "blue"];
//! // Parser sees "[red", parses it as color tag!
//! // println!("Colors: {:?}", colors);
//! ```
//!
//! For untrusted input or strict correctness, always use the `_raw` macros:
//!
//! ```no_run
//! use fast_rich::println_raw;
//!
//! let colors = vec!["red", "blue"];
//! println_raw!("Colors: {:?}", colors); // ✅ Safe: Prints ["red", "blue"]
//! ```
//!
//! ### Macro Summary
//!
//! | Macro | Markup | Use Case |
//! |-------|--------|----------|
//! | `print!` / `println!` | ✅ Smart | Styled output & most debug data |
//! | `print_raw!` / `println_raw!` | ❌ Skipped | Strict raw data output |
//! | `rprint!` / `rprintln!` | ✅ Smart | Alias (when you need both std and rich) |
//!
//! ### Macro Summary
//!
//! | Macro | Markup | Use Case |
//! |-------|--------|----------|
//! | `print!` / `println!` | ✅ Parsed | Styled output you control |
//! | `print_raw!` / `println_raw!` | ❌ Skipped | Data output, debug values |
//! | `rprint!` / `rprintln!` | ✅ Parsed | Alias (when you need both std and rich) |

use std::cell::RefCell;

// Core modules
pub mod align;
pub mod bar;
pub mod box_drawing;
pub mod console;
pub mod emoji;
pub mod group;
pub mod highlighter;
pub mod markup;
pub mod measure;
pub mod nested_progress;
pub mod padding;
pub mod pager;
pub mod renderable;
pub mod screen;
pub mod style;
pub mod text;
pub mod theme;

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
pub mod json;
pub mod prompt;
pub mod traceback;

// Optional feature-gated modules
#[cfg(feature = "markdown")]
pub mod markdown;

#[cfg(feature = "syntax")]
pub mod syntax;

// Re-exports for convenience
pub use console::Console;
pub use layout::Layout;
pub use live::Live;
pub use panel::{BorderStyle, Panel};
pub use renderable::Renderable;
pub use rule::Rule;
pub use style::{Color, Style};
pub use table::{Column, ColumnAlign, Table};
pub use text::{Alignment, Text};
pub use tree::{Tree, TreeNode};

// ============================================================================
// Thread-local Console for Print Macros
// ============================================================================

thread_local! {
    static STDOUT_CONSOLE: RefCell<Console> = RefCell::new(Console::new());
    static STDERR_CONSOLE: RefCell<Console> = RefCell::new(Console::stderr());
    // Raw consoles have markup parsing disabled - for data output
    static STDOUT_RAW_CONSOLE: RefCell<Console> = RefCell::new(Console::new().markup(false));
    static STDERR_RAW_CONSOLE: RefCell<Console> = RefCell::new(Console::stderr().markup(false));
}

/// Internal helper for print macros - DO NOT USE DIRECTLY.
#[doc(hidden)]
pub fn __internal_print(content: String, newline: bool) {
    STDOUT_CONSOLE.with(|c| {
        let console = c.borrow();
        if newline {
            console.println(&content);
        } else {
            console.print(&content);
        }
    });
}

/// Internal helper for eprint macros - DO NOT USE DIRECTLY.
#[doc(hidden)]
pub fn __internal_eprint(content: String, newline: bool) {
    STDERR_CONSOLE.with(|c| {
        let console = c.borrow();
        if newline {
            console.println(&content);
        } else {
            console.print(&content);
        }
    });
}

/// Internal helper for raw print macros (no markup parsing) - DO NOT USE DIRECTLY.
#[doc(hidden)]
pub fn __internal_print_raw(content: String, newline: bool) {
    STDOUT_RAW_CONSOLE.with(|c| {
        let console = c.borrow();
        if newline {
            console.println(&content);
        } else {
            console.print(&content);
        }
    });
}

/// Internal helper for raw eprint macros (no markup parsing) - DO NOT USE DIRECTLY.
#[doc(hidden)]
pub fn __internal_eprint_raw(content: String, newline: bool) {
    STDERR_RAW_CONSOLE.with(|c| {
        let console = c.borrow();
        if newline {
            console.println(&content);
        } else {
            console.print(&content);
        }
    });
}

// ============================================================================
// Print Macros - Drop-in replacements for std::print! and std::println!
// ============================================================================

/// Print formatted text with Rich markup to stdout (no newline).
///
/// This macro is a drop-in replacement for `std::print!` that adds
/// Rich markup support for colors, styles, and formatting.
///
/// # Example
///
/// ```no_run
/// use fast_rich::print;
///
/// print!("[bold blue]Status:[/] checking... ");
/// print!("Value: [yellow]{}[/]", 42);
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::__internal_print(format!($($arg)*), false);
    }};
}

/// Print formatted text with Rich markup to stdout (with newline).
///
/// This macro is a drop-in replacement for `std::println!` that adds
/// Rich markup support for colors, styles, and formatting.
///
/// # Example
///
/// ```no_run
/// use fast_rich::println;
///
/// println!("[bold green]Success![/] All tests passed.");
/// println!("Player {} scored [yellow]{}[/] points", "Alice", 100);
/// println!();  // Empty line
/// ```
#[macro_export]
macro_rules! println {
    () => {{
        $crate::__internal_print(String::new(), true);
    }};
    ($($arg:tt)*) => {{
        $crate::__internal_print(format!($($arg)*), true);
    }};
}

/// Print formatted text with Rich markup to stderr (no newline).
///
/// This macro is a drop-in replacement for `std::eprint!` that adds
/// Rich markup support for colors, styles, and formatting.
///
/// # Example
///
/// ```no_run
/// use fast_rich::eprint;
///
/// eprint!("[red]Error:[/] ");
/// ```
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::__internal_eprint(format!($($arg)*), false);
    }};
}

/// Print formatted text with Rich markup to stderr (with newline).
///
/// This macro is a drop-in replacement for `std::eprintln!` that adds
/// Rich markup support for colors, styles, and formatting.
///
/// # Example
///
/// ```no_run
/// use fast_rich::eprintln;
///
/// eprintln!("[bold red]Error:[/] Something went wrong!");
/// eprintln!("[yellow]Warning:[/] {} items skipped", 5);
/// ```
#[macro_export]
macro_rules! eprintln {
    () => {{
        $crate::__internal_eprint(String::new(), true);
    }};
    ($($arg:tt)*) => {{
        $crate::__internal_eprint(format!($($arg)*), true);
    }};
}

// ============================================================================
// Raw Print Macros - No markup parsing, safe for data output
// ============================================================================

/// Print to stdout without markup parsing (no newline).
///
/// Use this for data output that may contain brackets like `[1, 2, 3]`
/// which would otherwise be interpreted as style tags.
///
/// # Example
///
/// ```no_run
/// use fast_rich::print_raw;
///
/// let data = vec![1, 2, 3];
/// print_raw!("Data: {:?}", data);  // Brackets printed literally
/// ```
#[macro_export]
macro_rules! print_raw {
    ($($arg:tt)*) => {{
        $crate::__internal_print_raw(format!($($arg)*), false);
    }};
}

/// Print to stdout without markup parsing (with newline).
///
/// Use this for data output that may contain brackets like `[1, 2, 3]`
/// which would otherwise be interpreted as style tags.
///
/// # Example
///
/// ```no_run
/// use fast_rich::println_raw;
///
/// let items = vec!["a", "b", "c"];
/// println_raw!("Items: {:?}", items);  // Brackets printed literally
/// ```
#[macro_export]
macro_rules! println_raw {
    () => {{
        $crate::__internal_print_raw(String::new(), true);
    }};
    ($($arg:tt)*) => {{
        $crate::__internal_print_raw(format!($($arg)*), true);
    }};
}

/// Print to stderr without markup parsing (no newline).
#[macro_export]
macro_rules! eprint_raw {
    ($($arg:tt)*) => {{
        $crate::__internal_eprint_raw(format!($($arg)*), false);
    }};
}

/// Print to stderr without markup parsing (with newline).
#[macro_export]
macro_rules! eprintln_raw {
    () => {{
        $crate::__internal_eprint_raw(String::new(), true);
    }};
    ($($arg:tt)*) => {{
        $crate::__internal_eprint_raw(format!($($arg)*), true);
    }};
}

// ============================================================================
// Aliases - For users who want both standard and rich macros
// ============================================================================

/// Alias for `print!` - use when you need both `std::print!` and rich print.
///
/// # Example
///
/// ```no_run
/// use fast_rich::rprint;
///
/// std::print!("Standard: no markup [bold]");
/// rprint!("Rich: [bold]this is bold[/]");
/// ```
#[macro_export]
macro_rules! rprint {
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
    }};
}

/// Alias for `println!` - use when you need both `std::println!` and rich println.
///
/// # Example
///
/// ```no_run
/// use fast_rich::rprintln;
///
/// std::println!("Standard: no markup [bold]");
/// rprintln!("Rich: [bold]this is bold[/]");
/// ```
#[macro_export]
macro_rules! rprintln {
    () => {{
        $crate::println!();
    }};
    ($($arg:tt)*) => {{
        $crate::println!($($arg)*);
    }};
}

/// Prelude module for convenient imports.
///
/// ## Print Macros
///
/// The print macros are NOT included in the prelude to avoid conflicts with `std`.
/// Import them explicitly if you want drop-in shadowing:
///
/// ```no_run
/// use fast_rich::{print, println};
/// println!("[bold green]Hello![/]");
/// ```
///
/// The following ARE included in the prelude:
/// - `rprint!` / `rprintln!` - Aliases that don't conflict with std
/// - `print_raw!` / `println_raw!` - Raw output without markup parsing
pub mod prelude {
    // Aliases that don't conflict with std
    pub use crate::{rprint, rprintln};

    // Raw print macros for data output (no markup parsing, no std conflicts)
    pub use crate::{eprint_raw, eprintln_raw, print_raw, println_raw};

    pub use crate::columns::Columns;
    pub use crate::console::Console;
    pub use crate::inspect::{inspect, InspectConfig};
    pub use crate::json::Json;
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
            .render(&console::RenderContext {
                width: 40,
                height: None
            })
            .is_empty());
    }
}
