use rich_rust::prelude::*;

fn run(console: &Console) {
    console.rule("[bold cyan]FASTRICH RUST SHOWCASE[/]");
    console.newline();

    // 1. Text & Styles
    console.print("[bold yellow]1. Text & Styles[/]");
    console.print("Rich supports [bold]bold[/], [italic]italic[/], [underline]underline[/], and [strike]strike[/].");
    console.print("Colors: [red]red[/], [green]green[/], [blue]blue[/], [magenta]magenta[/], [cyan]cyan[/], [yellow]yellow[/].");
    console.print("Backgrounds: [white on blue] white on blue [/], [black on yellow] warning [/].");
    console.print("Truecolor: [color(255,100,0)]orange[/], [color(0,255,100)]spring green[/].");
    console.newline();

    // 2. Panels
    console.print("[bold yellow]2. Panels[/]");
    let inner_text = rich_rust::markup::parse("This is a panel with a [bold]title [/] and [blue]blue border[/].\nContent automatically wraps to fit.");
    let panel = Panel::new(inner_text)
        .title("Panel Example")
        .border_style(BorderStyle::Rounded)
        .style(Style::new().foreground(Color::Blue));
    console.print_renderable(&panel);
    console.newline();

    // 3. Tables
    console.print("[bold yellow]3. Tables[/]");
    let mut table = Table::new().title("User Statistics");
    table.add_column("ID");
    table.add_column("Username");
    table.add_column("Role");
    table.add_column("Status");

    table.add_row(vec![
        Text::plain("1"),
        Text::plain("alice"),
        Text::plain("Admin"),
        rich_rust::markup::parse("[green]Active[/]"),
    ]);
    table.add_row(vec![
        Text::plain("2"),
        Text::plain("bob"),
        Text::plain("Developer"),
        rich_rust::markup::parse("[yellow]Away[/]"),
    ]);
    table.add_row(vec![
        Text::plain("3"),
        Text::plain("charlie"),
        Text::plain("User"),
        rich_rust::markup::parse("[red]Offline[/]"),
    ]);

    console.print_renderable(&table);
    console.newline();

    // 4. Tree
    console.print("[bold yellow]4. Tree[/]");
    let mut tree = Tree::new(rich_rust::markup::parse("[bold]Root[/]"));

    let mut src = TreeNode::new(rich_rust::markup::parse("[blue]src/[/]"));
    src.add("main.rs");
    src.add("lib.rs");

    let mut tests = TreeNode::new(rich_rust::markup::parse("[blue]tests/[/]"));
    tests.add("test_core.rs");

    tree.add(src);
    tree.add(tests);
    tree.add("Cargo.toml");

    console.print_renderable(&tree);
    console.newline();

    // 5. Progress (Snapshot)
    console.print("[bold yellow]5. Progress (Snapshot)[/]");
    let progress = Progress::new();
    let id1 = progress.add_task("Downloading...", Some(100)); // 100 total
    let id2 = progress.add_task("Processing...", Some(200)); // 200 total

    // Simulate some progress
    progress.advance(id1, 45); // 45%
    progress.advance(id2, 120); // 60%

    // Render single frame without animation loop
    let rendered_bars = progress.render_to_string();
    console.print(rendered_bars.trim_end()); // print rendered bars
    console.newline();

    // 6. Traceback
    console.print("[bold yellow]6. Traceback (Simulated)[/]");
    let tb = rich_rust::traceback::Traceback::from_error("Simulated connection error").with_config(
        rich_rust::traceback::TracebackConfig {
            show_source: false, // No real source to show here
            ..Default::default()
        },
    );
    console.print_renderable(&tb);
    console.newline();

    // 7. Markdown (If feature enabled)
    #[cfg(feature = "markdown")]
    {
        console.print("[bold yellow]7. Markdown Rendering[/]");
        let md = rich_rust::markdown::Markdown::new(
            "# Heading 1\n## Heading 2\n- List item 1\n- List item 2\n\n`code block`",
        );
        console.print_renderable(&md);
    }

    // 8. Syntax (If feature enabled)
    #[cfg(feature = "syntax")]
    {
        console.newline();
        console.print("[bold yellow]8. Syntax Highlighting (JSON)[/]");
        let json = r#"{"name": "fast_rich", "speed": "blazing", "version": 0.3}"#;
        let syntax = rich_rust::syntax::Syntax::new(json, "json");
        console.print_renderable(&syntax);
    }

    console.newline();
    console.rule("[bold cyan]End of Showcase[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_showcase_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();

        assert!(output.contains("FASTRICH RUST SHOWCASE"));
        assert!(output.contains("Text"));
        assert!(output.contains("Styles"));
        assert!(output.contains("Panels"));
        assert!(output.contains("Panel Example"));
        assert!(output.contains("Active")); // Table content
        assert!(output.contains("Tree"));
        assert!(output.contains("tests/")); // Tree content
        assert!(output.contains("Progress"));
        assert!(output.contains("Traceback"));
        assert!(output.contains("End of Showcase"));
    }
}
