use fast_rich::box_drawing;
use fast_rich::console::RenderContext;
use fast_rich::panel::{BorderStyle, Panel};
use fast_rich::renderable::Renderable;
use fast_rich::table::Table;

fn render<T: Renderable>(item: &T) -> String {
    let context = RenderContext {
        width: 40,
        height: None, direction: Default::default(),
    };
    let segments = item.render(&context);
    segments
        .iter()
        .map(|s| s.plain_text())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_box_rounded() {
    let b = box_drawing::ROUNDED;
    assert_eq!(b.top.left, '╭');
    assert_eq!(b.top.right, '╮');
    assert_eq!(b.bottom.left, '╰');
    assert_eq!(b.bottom.right, '╯');
}

#[test]
fn test_box_heavy() {
    let b = box_drawing::HEAVY;
    assert_eq!(b.top.left, '┏');
    assert_eq!(b.top.right, '┓');
    assert_eq!(b.top.mid, '━');
}

#[test]
fn test_panel_rounded() {
    let panel = Panel::new("Content").border_style(BorderStyle::Rounded);
    let output = render(&panel);
    assert!(output.contains('╭'));
    assert!(output.contains('╮'));
    assert!(output.contains('╰'));
    assert!(output.contains('╯'));
}

#[test]
fn test_panel_double() {
    let panel = Panel::new("Content").border_style(BorderStyle::Double);
    let output = render(&panel);
    assert!(output.contains('╔'));
    assert!(output.contains('╗'));
    assert!(output.contains('╚'));
    assert!(output.contains('╝'));
}

#[test]
fn test_table_heavy_head() {
    let mut table = Table::new()
        .border_style(BorderStyle::HeavyHead)
        .column("Col1");
    table.add_row_strs(&["Val1"]);

    let output = render(&table);
    // HeavyHead has heavy top and header separator
    // Top left should be heavy ┏
    assert!(output.contains('┏'));
    // Header separator line uses '┡' for left
    assert!(output.contains('┡'));
    // Bottom left should be light └
    assert!(output.contains('└'));
}

#[test]
fn test_table_ascii_double_head() {
    let mut table = Table::new()
        .border_style(BorderStyle::AsciiDoubleHead)
        .column("Col1");
    table.add_row_strs(&["Val1"]);

    let output = render(&table);
    // Header separator should be =
    assert!(output.contains('='));
}
