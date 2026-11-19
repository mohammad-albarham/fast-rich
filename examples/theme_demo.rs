//! Before/after tour of theme style names (G-TH1): Rich's default styles
//! (`rich/default_styles.py`) by name, so built-in components can be
//! restyled through a `Theme`, like Rich's
//! `Console(theme=Theme({"table.header": "bold magenta"}))`.
//!
//! Every "after" below uses the same component code as "before"; only the
//! console theme changes.
//!
//! Run: cargo run --example theme_demo

use fast_rich::log::{LogLevel, LogMessage};
use fast_rich::prelude::*;
use fast_rich::progress::{BarColumn, PercentageColumn, TextColumn};

/// The theme: Rich style names mapped to Rich style strings.
fn theme() -> Theme {
    Theme::from_styles([
        ("table.header", "bold magenta"),
        ("table.border", "bright_black"),
        ("bar.complete", "cyan"),
        ("bar.finished", "bold green"),
        ("bar.back", "grey30"),
        ("progress.percentage", "bold cyan"),
        ("rule.line", "dark_orange3"),
        ("rule.text", "bold"),
        ("logging.level.info", "bold green"),
        ("logging.level.warning", "black on yellow"),
        ("repr.number", "bold magenta"),
    ])
}

fn table() -> Table {
    let mut table = Table::new().title("Planets");
    table.add_column("Name");
    table.add_column("Moons");
    table.add_row_strs(&["Earth", "1"]);
    table.add_row_strs(&["Mars", "2"]);
    table
}

fn progress() -> Progress {
    let progress = Progress::new().with_columns(vec![
        Box::new(TextColumn::new("[progress.description]")),
        Box::new(BarColumn::new(24)),
        Box::new(PercentageColumn::new()),
    ]);
    let download = progress.add_task("download", Some(100));
    progress.update(download, 60);
    let unpack = progress.add_task("unpack  ", Some(10));
    progress.update(unpack, 10);
    progress
}

/// Render every section on `console`.
fn show(console: &Console) {
    // Table header and border: `table.header`, `table.border`.
    console.print_renderable(&table());

    // Progress bar: `bar.complete`, `bar.finished`, `bar.back`,
    // `progress.percentage`.
    console.print_renderable(&progress());

    // Rule: `rule.line`, `rule.text`.
    console.print_renderable(&Rule::new("section"));

    // Log levels: `log.time`, `logging.level.*`, `log.path`.
    for (level, message) in [
        (LogLevel::Info, "server started"),
        (LogLevel::Warning, "disk 91% full"),
    ] {
        let log = LogMessage::new(message)
            .level(level)
            .location("src/main.rs", 42);
        console.print_renderable(&log);
    }

    // Repr numbers (the automatic highlighter): `repr.number`.
    console.println("Returned 404 after 3.5 seconds, retries=[1, 2, 3]");
}

fn main() {
    let plain = Console::new();
    let themed = Console::new().theme(theme());

    plain.println("[bold]Before: Rich's default styles (no theme)[/]\n");
    show(&plain);

    plain.println("\n[bold]After: the same code, Console::new().theme(theme())[/]\n");
    show(&themed);

    // A style set on the component still wins over the theme.
    plain.println("\n[bold]Explicit styles beat the theme[/]\n");
    themed.print_renderable(&Rule::new("explicit").style(Style::parse("blue")));

    // The semantic palettes (`primary`, `success`, ...) are still there,
    // usable as markup tags such as `[success]`.
    plain.println("\n[bold]Semantic palettes[/]\n");
    for (name, palette) in [
        ("default_theme", Theme::default_theme()),
        ("monokai", Theme::monokai()),
        ("night_owl", Theme::night_owl()),
    ] {
        let console = Console::new().theme(palette);
        console.println(&format!(
            "{name:<14}[primary]primary[/] [success]success[/] [warning]warning[/] [error]error[/]"
        ));
    }
}
