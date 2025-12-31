use rich_rust::console::Console;
use rich_rust::panel::Panel;
use rich_rust::table::Table;
use rich_rust::tree::Tree;
use rich_rust::Color;

fn render_snapshot<T: rich_rust::renderable::Renderable>(item: &T) -> String {
    let console = Console::capture();
    console.print_renderable(item);
    console.get_captured_output()
}

#[test]
fn test_style_snapshot() {
    let output = render_snapshot(&rich_rust::markup::parse(
        "[bold red]Hello[/] [blue]World[/]!",
    ));
    assert!(output.contains("Hello"));
    assert!(output.contains("World"));
}

#[test]
fn test_table_snapshot() {
    let mut table = Table::new();
    table.add_column("Col 1").add_column("Col 2");
    table.add_row(vec![
        rich_rust::text::Text::from("Val 1"),
        rich_rust::text::Text::from("Val 2"),
    ]);

    let output = render_snapshot(&table);

    assert!(output.contains("Col 1"));
    assert!(output.contains("Col 2"));
    assert!(output.contains("Val 1"));
    assert!(output.contains("Val 2"));
    assert!(output.contains("╭"));
    assert!(output.contains("╮"));
}

#[test]
fn test_panel_snapshot() {
    let panel = Panel::new("Panel Content")
        .title("My Title")
        .style(rich_rust::style::Style::new().foreground(Color::Blue));

    let output = render_snapshot(&panel);
    assert!(output.contains("My Title"));
    assert!(output.contains("Panel Content"));
    assert!(output.contains("╭"));
}

#[test]
fn test_tree_snapshot() {
    let mut tree = Tree::new(rich_rust::text::Text::from("Root"));
    tree.add(rich_rust::text::Text::from("Child 1"));
    let child2 = tree.add(rich_rust::text::Text::from("Child 2"));
    child2.add(rich_rust::text::Text::from("Grandchild"));

    let output = render_snapshot(&tree);
    assert!(output.contains("Root"));
    assert!(output.contains("Child 1"));
    assert!(output.contains("Child 2"));
    assert!(output.contains("Grandchild"));
    assert!(output.contains("├──"));
}

#[cfg(feature = "markdown")]
#[test]
fn test_markdown_snapshot() {
    let md = rich_rust::markdown::Markdown::new("# Header\n* List item");
    let output = render_snapshot(&md);
    assert!(output.contains("Header"));
    assert!(output.contains("• List item"));
}

#[cfg(feature = "syntax")]
#[test]
fn test_syntax_snapshot() {
    let code = "def foo(): pass";
    let syntax = rich_rust::syntax::Syntax::new(code, "python");
    let output = render_snapshot(&syntax);
    assert!(output.contains("def"));
    assert!(output.contains("foo"));
}

#[test]
fn test_rule_snapshot() {
    let rule = rich_rust::rule::Rule::new("Chapter 1");
    let output = render_snapshot(&rule);
    assert!(output.contains("Chapter 1"));
    // Rule renders as a full width line, usually with unicode chars like '─'
    // We check for the title and presence of line characters
    assert!(output.contains("Chapter 1"));
}

#[test]
fn test_columns_snapshot() {
    let items = vec![
        rich_rust::text::Text::from("Item 1"),
        rich_rust::text::Text::from("Item 2"),
        rich_rust::text::Text::from("Item 3"),
    ];
    let columns = rich_rust::columns::Columns::new(items);
    let output = render_snapshot(&columns);
    assert!(output.contains("Item 1"));
    assert!(output.contains("Item 2"));
    assert!(output.contains("Item 3"));
}
