//! Introspection tools for debugging.

use crate::console::Console;
use crate::panel::Panel;
use crate::pretty::Pretty;

use std::fmt::Debug;

/// Configuration for the inspect function.
#[derive(Debug, Clone, Default)]
pub struct InspectConfig {
    /// Show a note about documentation availability.
    pub help: bool,
}

/// Inspect an object and print a report to the console.
///
/// In Rust, this relies on the `Debug` trait. The value is pretty printed
/// with [`Pretty`] to fit inside the panel.
pub fn inspect<T: Debug + ?Sized>(obj: &T, config: InspectConfig) {
    let console = Console::new();
    let panel = inspect_panel(obj, &config, console.get_width());
    console.print_renderable(&panel);
}

/// Build the report panel for a console `width` cells wide.
fn inspect_panel<T: Debug + ?Sized>(obj: &T, config: &InspectConfig, width: usize) -> Panel {
    let type_name = std::any::type_name::<T>();

    // Header (plain title — Panel titles are not markup-parsed)
    let title = format!("Inspect: {}", type_name);

    let mut header = format!("Type: [green]{}[/]", crate::markup::escape(type_name));
    if config.help {
        header
            .push_str("\n\n[dim]Docstrings are not available in Rust runtime introspection.[/dim]");
    }
    header.push_str("\n\n[bold]Value:[/]\n");

    // Panel border (2) plus horizontal padding (2).
    let inner_width = width.saturating_sub(4).max(1);
    let mut content = crate::markup::parse(&header);
    content.append_text(&Pretty::new(obj).text(inner_width));

    Panel::new(content)
        .title(&title)
        .style(crate::style::Style::new().foreground(crate::style::Color::Blue))
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
    fn test_value_is_pretty_printed() {
        let console = Console::capture().width(24);
        let value = vec!["[red]", "beta", "gamma"];
        // 26 cells on one line; the panel leaves 20, so it expands.
        let out = console.export_text(&inspect_panel(&value, &InspectConfig::default(), 24));
        let lines: Vec<&str> = out.lines().map(str::trim_end).collect();
        assert!(lines.contains(&"│ [                    │"), "{out}");
        assert!(lines.contains(&"│     \"[red]\",         │"), "{out}");
        assert!(lines.contains(&"│ ]                    │"), "{out}");
        let wide = Console::capture().width(30).export_text(&inspect_panel(
            &value,
            &InspectConfig::default(),
            30,
        ));
        assert!(
            wide.contains("│ [\"[red]\", \"beta\", \"gamma\"] │"),
            "{wide}"
        );
    }

    #[test]
    fn test_title_has_no_raw_markup() {
        // The title is rendered as plain text — a regression guard against
        // literal "[bold cyan]" leaking into panel titles.
        let title = format!("Inspect: {}", std::any::type_name::<Vec<i32>>());
        assert!(!title.contains('['));
    }
}
