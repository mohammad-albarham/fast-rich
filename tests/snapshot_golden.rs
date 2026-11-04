//! Golden-file snapshot tests.
//!
//! Unlike `snapshots.rs` (substring assertions), these compare complete
//! rendered output against reference files in `tests/snapshots/`. To
//! regenerate the reference files after an intentional rendering change, run:
//!
//! ```sh
//! FAST_RICH_UPDATE_SNAPSHOTS=1 cargo test --test snapshot_golden
//! ```

use fast_rich::console::Console;
use fast_rich::prelude::*;
use std::fs;
use std::path::Path;

const DIR: &str = "tests/snapshots";

fn render_plain(renderable: &dyn Renderable, width: usize) -> String {
    let console = Console::capture().width(width);
    console.export_text(renderable)
}

fn assert_snapshot(name: &str, actual: &str) {
    let path = format!("{DIR}/{name}.snap");
    if std::env::var("FAST_RICH_UPDATE_SNAPSHOTS").is_ok() {
        fs::create_dir_all(DIR).expect("create snapshot dir");
        fs::write(&path, actual).expect("write snapshot");
        eprintln!("updated {path}");
        return;
    }

    match fs::read_to_string(&path) {
        Ok(expected) => assert_eq!(
            actual, expected,
            "snapshot {name} drifted; rerun with FAST_RICH_UPDATE_SNAPSHOTS=1 if intended"
        ),
        Err(_) if !Path::new(&path).exists() => {
            panic!("snapshot {path} missing — generate with FAST_RICH_UPDATE_SNAPSHOTS=1")
        }
        Err(e) => panic!("failed to read {path}: {e}"),
    }
}

#[test]
fn snapshot_table_basic() {
    let mut table = Table::new().title("Inventory");
    table.add_column("Item");
    table.add_column(Column::new("Qty").align(Alignment::Right));
    table.add_row_strs(&["Apples", "12"]);
    table.add_row_strs(&["Bananas", "7"]);
    assert_snapshot("table_basic", &render_plain(&table, 60));
}

#[test]
fn snapshot_panel_fit() {
    let panel = Panel::new("Hello\nWorld").title("Panel");
    assert_snapshot("panel_fit", &render_plain(&panel, 40));
}

#[test]
fn snapshot_tree_nested() {
    let mut a = TreeNode::new("a");
    a.add(TreeNode::new("a1"));
    let mut root = TreeNode::new("root");
    root.add(a);
    root.add(TreeNode::new("b"));
    assert_snapshot("tree_nested", &render_plain(&Tree::new(root), 40));
}

#[test]
fn snapshot_rule_titled() {
    assert_snapshot("rule_titled", &render_plain(&Rule::new("Section"), 30));
}

#[test]
fn snapshot_multiline_table_cells() {
    let mut table = Table::new();
    table.add_column("A");
    table.add_column("B");
    table.add_row_strs(&["one\ntwo", "x"]);
    assert_snapshot("table_multiline", &render_plain(&table, 40));
}
