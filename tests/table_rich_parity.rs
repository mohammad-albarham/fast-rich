//! Table layout parity with Python Rich 15 (references generated with
//! `rich.table.Table`, plain output, trailing spaces trimmed).

use fast_rich::panel::{BorderStyle, Panel};
use fast_rich::renderable::render_lines;
use fast_rich::style::Style;
use fast_rich::table::{Cell, Column, ColumnWidth, Row, Table};

fn plain(t: &Table, width: usize) -> String {
    render_lines(t, width, Style::new())
        .iter()
        .map(|l| l.iter().map(|s| s.text.as_ref()).collect::<String>())
        .map(|l| l.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn table_basic() {
    let mut t = Table::new().border_style(BorderStyle::HeavyHead);
    t.add_column("Name");
    t.add_column(Column::new("Age").right());
    t.add_row_strs(&["Alice", "30"]);
    t.add_row_strs(&["Bob", "5"]);
    assert_eq!(plain(&t, 40), "┏━━━━━━━┳━━━━━┓\n┃ Name  ┃ Age ┃\n┡━━━━━━━╇━━━━━┩\n│ Alice │  30 │\n│ Bob   │   5 │\n└───────┴─────┘");
}

#[test]
fn table_wrap() {
    let mut t = Table::new().columns(["A", "B"]);
    t.add_row_strs(&["short", "a fairly long piece of text that must wrap inside"]);
    assert_eq!(plain(&t, 30), "╭───────┬────────────────────╮\n│ A     │ B                  │\n├───────┼────────────────────┤\n│ short │ a fairly long      │\n│       │ piece of text that │\n│       │ must wrap inside   │\n╰───────┴────────────────────╯");
}

#[test]
fn table_expand() {
    let mut t = Table::new().columns(["A", "B"]).expand(true);
    t.add_row_strs(&["1", "2"]);
    assert_eq!(plain(&t, 40), "╭───────────────────┬──────────────────╮\n│ A                 │ B                │\n├───────────────────┼──────────────────┤\n│ 1                 │ 2                │\n╰───────────────────┴──────────────────╯");
}

#[test]
fn table_cjk() {
    let mut t = Table::new().columns(["名前", "x"]);
    t.add_row_strs(&["日本語テキスト", "😀"]);
    assert_eq!(plain(&t, 40), "╭────────────────┬────╮\n│ 名前           │ x  │\n├────────────────┼────┤\n│ 日本語テキスト │ 😀 │\n╰────────────────┴────╯");
}

#[test]
fn table_title() {
    let mut t = Table::new().title("Title").caption("cap");
    t.add_column(Column::new("A").center());
    t.add_row_strs(&["abcdef"]);
    assert_eq!(
        plain(&t, 40),
        "  Title\n╭────────╮\n│   A    │\n├────────┤\n│ abcdef │\n╰────────╯\n   cap"
    );
}

#[test]
fn table_lines_noedge() {
    let mut t = Table::new()
        .columns(["A", "B"])
        .border_style(BorderStyle::Square)
        .show_row_lines(true)
        .show_edge(false);
    t.add_row_strs(&["1", "2"]);
    t.add_row_strs(&["3", "4"]);
    assert_eq!(plain(&t, 40), " A │ B\n───┼───\n 1 │ 2\n───┼───\n 3 │ 4");
}

#[test]
fn table_nobox() {
    let mut t = Table::new().columns(["A", "B"]).show_border(false);
    t.add_row_strs(&["1", "2"]);
    assert_eq!(plain(&t, 40), " A  B\n 1  2");
}

#[test]
fn table_nested_nowrap() {
    let mut t = Table::new();
    t.add_column("A");
    t.add_column(Column::new("B").no_wrap(true));
    t.add_row_obj(Row::cells([
        Cell::renderable(Panel::new("in")),
        Cell::from("a very long no wrap cell content here"),
    ]));
    assert_eq!(plain(&t, 30), "╭┬───────────────────────────╮\n││ B                         │\n├┼───────────────────────────┤\n││ a very long no wrap cell… │\n╰┴───────────────────────────╯");
}

#[test]
fn table_ratio() {
    let mut t = Table::new().expand(true);
    t.add_column(Column::new("A").ratio(1));
    t.add_column(Column::new("B").ratio(2));
    t.add_row_strs(&["x", "y"]);
    assert_eq!(plain(&t, 40), "╭─────────────┬────────────────────────╮\n│ A           │ B                      │\n├─────────────┼────────────────────────┤\n│ x           │ y                      │\n╰─────────────┴────────────────────────╯");
}

#[test]
fn table_widths() {
    let mut t = Table::new();
    t.add_column(Column::new("A").width(ColumnWidth::Fixed(10)));
    t.add_column(Column::new("B").width(ColumnWidth::Min(6)));
    t.add_column(Column::new("C").width(ColumnWidth::Max(4)));
    t.add_row_strs(&["x", "y", "zzzzzzzz"]);
    assert_eq!(plain(&t, 40), "╭────────────┬────────┬──────╮\n│ A          │ B      │ C    │\n├────────────┼────────┼──────┤\n│ x          │ y      │ zzz… │\n╰────────────┴────────┴──────╯");
}

fn raw(t: &Table, width: usize) -> String {
    render_lines(t, width, Style::new())
        .iter()
        .map(|l| l.iter().map(|s| s.text.as_ref()).collect::<String>())
        .map(|l| l.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn table_grid_padding() {
    let mut t = Table::grid().padding(1);
    for _ in 0..3 {
        t.add_column("");
    }
    t.add_row_strs(&["a", "bb", "ccc"]);
    t.add_row_strs(&["dddd", "e", "f"]);
    assert_eq!(raw(&t, 20), "a    bb ccc\ndddd e  f");
}

#[test]
fn table_grid_expand() {
    let mut t = Table::grid().padding(1).expand(true);
    for _ in 0..3 {
        t.add_column("");
    }
    t.add_row_strs(&["a", "bb", "ccc"]);
    assert_eq!(raw(&t, 20), "a    bb      ccc");
}

#[test]
fn table_grid_no_padding() {
    let mut t = Table::grid();
    t.add_column("");
    t.add_column("");
    t.add_row_strs(&["x", "y"]);
    assert_eq!(raw(&t, 20), "xy");
}

#[test]
fn table_pad_edge_false() {
    let mut t = Table::new()
        .border_style(BorderStyle::HeavyHead)
        .padding(2)
        .pad_edge(false);
    t.add_column("A");
    t.add_column("B");
    t.add_row_strs(&["1", "2"]);
    assert_eq!(
        raw(&t, 20),
        "┏━━━┳━━━┓\n┃A  ┃  B┃\n┡━━━╇━━━┩\n│1  │  2│\n└───┴───┘"
    );
}
