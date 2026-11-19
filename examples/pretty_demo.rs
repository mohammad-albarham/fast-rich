//! Before/after tour of pretty printing (G-PR1): `Pretty`, `pretty_repr`
//! and `pprint`, a port of Rich's `rich.pretty`.
//!
//! Run: cargo run --example pretty_demo

use std::collections::BTreeMap;

use fast_rich::prelude::*;
use fast_rich::Constrain;

#[allow(dead_code)] // Fields are read through Debug.
#[derive(Debug)]
struct User {
    id: usize,
    name: String,
    roles: Vec<String>,
    active: bool,
    metadata: BTreeMap<String, String>,
}

fn main() {
    let console = Console::new();

    let mut metadata = BTreeMap::new();
    metadata.insert("login".to_string(), "2023-01-01".to_string());
    metadata.insert("ip".to_string(), "127.0.0.1".to_string());
    let user = User {
        id: 42,
        name: "Alice".to_string(),
        roles: vec!["admin".to_string(), "editor".to_string()],
        active: true,
        metadata,
    };

    // 1. print_debug -------------------------------------------------------
    console.println("[bold]1. console.print_debug(&user)[/]");

    // Before: print_debug printed `{:#?}`: one field per line at any width,
    // even for short lists, and no width limit for long ones.
    console.println("[dim]before ({:#?}):[/]");
    console.println_raw(&format!("{user:#?}"));

    // After: Rich's layout. A container stays on one line when it fits.
    console.println("[dim]after (Pretty):[/]");
    console.print_debug(&user);

    // 2. The same data at two widths --------------------------------------
    console.println("\n[bold]2. Nested data at 64 and 30 cells[/]");
    let mut inventory = BTreeMap::new();
    inventory.insert("fruit", vec![("apple", 3), ("banana", 12), ("cherry", 40)]);
    inventory.insert("veg", vec![("leek", 1)]);
    for width in [64, 30] {
        console.println(&format!("[dim]width {width}:[/]"));
        console.print_renderable(&Constrain::new(Pretty::new(&inventory), Some(width)));
    }

    // 3. pretty_repr: the layout as a plain String ------------------------
    console.println("\n[bold]3. pretty_repr (no styles)[/]");
    let options = PrettyOptions::default().max_width(20);
    console.println_raw(&pretty_repr(&vec![(1, "one"), (2, "two")], &options));

    // 4. Limits -------------------------------------------------------------
    console.println("\n[bold]4. max_length, max_string, max_depth[/]");
    let numbers: Vec<u32> = (1..=100).collect();
    console.println("[dim]max_length(5):[/]");
    console.print_renderable(&Pretty::new(&numbers).max_length(5));

    let quote = vec!["Lorem ipsum dolor sit amet, consectetur adipiscing elit"];
    console.println("[dim]max_string(11):[/]");
    console.print_renderable(&Pretty::new(&quote).max_string(11));

    let deep = vec![vec![vec![vec![1, 2], vec![3]]], vec![]];
    console.println("[dim]max_depth(2):[/]");
    console.print_renderable(&Pretty::new(&deep).max_depth(2));

    // 5. Indent guides and expand_all -------------------------------------
    console.println("\n[bold]5. indent_guides + expand_all[/]");
    console.print_renderable(&Pretty::new(&inventory).indent_guides(true).expand_all(true));

    // 6. pprint ---------------------------------------------------------------
    console.println("\n[bold]6. pprint(&value, options)[/]");
    pprint(
        &Some(Ok::<_, String>((1.5, 'x', "done"))),
        PrettyOptions::default(),
    );
}
