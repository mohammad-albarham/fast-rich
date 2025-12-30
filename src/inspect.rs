//! Introspection tools for debugging.

use crate::console::Console;
use crate::panel::Panel;

use std::fmt::Debug;

/// Configuration for the inspect function.
#[derive(Debug, Clone, Default)]
pub struct InspectConfig {
    /// Show detailed documentation (mock).
    pub help: bool,
    /// Show methods (mock).
    pub methods: bool,
}

/// Inspect an object and print a report to the console.
///
/// In Rust, this relies on the `Debug` trait.
pub fn inspect<T: Debug>(obj: &T, config: InspectConfig) {
    let console = Console::new();
    let debug_str = format!("{:#?}", obj);

    // Header
    let type_name = std::any::type_name::<T>();
    let title = format!("Inspect: [bold cyan]{}[/]", type_name);

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
