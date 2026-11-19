//! Before/after tour of pretty printing (G-PR1): `Pretty`, `pretty_repr`
//! and `pprint`, a port of Rich's `rich.pretty`.
//!
//! Run: cargo run --example pretty_demo
//! With section 7 (exact structure via serde / valuable):
//!     cargo run --example pretty_demo --features serde,valuable

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

/// A matrix with a hand-written Debug that hides its structure.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
struct Matrix {
    rows: Vec<Vec<i32>>,
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.rows.len();
        write!(f, "Matrix {n}x{n} {:?}", self.rows)
    }
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

    // 7. Exact structure: serde / valuable ----------------------------------
    console.println("\n[bold]7. Custom Debug vs. serde / valuable (width 24)[/]");
    let matrix = Matrix {
        rows: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    };
    // Before: the hand-written Debug text is one opaque atom; it can't expand.
    console.println("[dim]Pretty::new (Debug):[/]");
    console.print_renderable(&Constrain::new(Pretty::new(&matrix), Some(24)));
    // After: the real fields, laid out like any other struct.
    #[cfg(feature = "serde")]
    {
        console.println("[dim]Pretty::from_serialize:[/]");
        console.print_renderable(&Constrain::new(Pretty::from_serialize(&matrix), Some(24)));
    }
    #[cfg(feature = "valuable")]
    {
        console.println("[dim]Pretty::from_valuable:[/]");
        console.print_renderable(&Constrain::new(Pretty::from_valuable(&matrix), Some(24)));
    }
    #[cfg(not(any(feature = "serde", feature = "valuable")))]
    console.println("[dim](enable --features serde,valuable for the structured versions)[/]");
}
