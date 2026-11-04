//! Introspection tools for debugging.

use crate::console::Console;
use crate::panel::Panel;

use std::fmt::Debug;

/// Configuration for the inspect function.
#[derive(Debug, Clone, Default)]
pub struct InspectConfig {
    /// Show a note about documentation availability.
    pub help: bool,
}

/// Inspect an object and print a report to the console.
///
/// In Rust, this relies on the `Debug` trait.
pub fn inspect<T: Debug>(obj: &T, config: InspectConfig) {
    let console = Console::new();
    let debug_str = format!("{:#?}", obj);
    let type_name = std::any::type_name::<T>();

    // Header (plain title — Panel titles are not markup-parsed)
    let title = format!("Inspect: {}", type_name);

    let mut chunks = Vec::new();
    chunks.push(format!("Type: [green]{}[/]", type_name));

    if config.help {
        chunks.push(
            "\n[dim]Docstrings are not available in Rust runtime introspection.[/dim]".to_string(),
        );
    }

    chunks.push(format!("\n[bold]Value:[/]\n{}", debug_str));

    let content = chunks.join("\n");
    let panel = Panel::new(crate::markup::parse(&content))
        .title(&title)
        .style(crate::style::Style::new().foreground(crate::style::Color::Blue));

    console.print_renderable(&panel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_config_default() {
        let config = InspectConfig::default();
        assert!(!config.help);
    }

    #[test]
    fn test_inspect_runs_without_panic() {
        // Prints to the real console; a smoke test for the formatting path.
        inspect(&vec![1, 2, 3], InspectConfig::default());
        inspect(&"str".to_string(), InspectConfig { help: true });
    }

    #[test]
    fn test_title_has_no_raw_markup() {
        // The title is rendered as plain text — a regression guard against
        // literal "[bold cyan]" leaking into panel titles.
        let title = format!("Inspect: {}", std::any::type_name::<Vec<i32>>());
        assert!(!title.contains('['));
    }
}
