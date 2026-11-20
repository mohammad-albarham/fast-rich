//! Embed another program's colored output in a Panel and a Table with
//! `Text::from_ansi` (Rich's `AnsiDecoder`, G-TX2).
//!
//! The escape codes become styles, so borders line up: widths are measured
//! on the visible text only. Pasting the raw string instead would count
//! every escape byte as a column and push the right border out.
//!
//! Run: cargo run --example ansi_demo

use fast_rich::prelude::*;

/// What `cargo build` prints in a color terminal.
const CARGO: &str = "\x1b[0m\x1b[1m\x1b[38;5;9merror[E0425]\x1b[0m\x1b[0m\x1b[1m: cannot find value `x` in this scope\x1b[0m\n\
\x1b[0m \x1b[0m\x1b[0m\x1b[1m\x1b[38;5;12m--> \x1b[0m\x1b[0msrc/main.rs:2:5\x1b[0m\n\
\x1b[0m  \x1b[0m\x1b[0m\x1b[1m\x1b[38;5;12m|\x1b[0m\n\
\x1b[0m\x1b[1m\x1b[38;5;12m2\x1b[0m\x1b[0m \x1b[0m\x1b[0m\x1b[1m\x1b[38;5;12m|\x1b[0m\x1b[0m     x\x1b[0m\n\
\x1b[0m  \x1b[0m\x1b[0m\x1b[1m\x1b[38;5;12m|\x1b[0m\x1b[0m     \x1b[0m\x1b[0m\x1b[1m\x1b[38;5;9m^\x1b[0m\x1b[0m \x1b[0m\x1b[0m\x1b[1m\x1b[38;5;9mnot found in this scope\x1b[0m\n\
\x1b[1m\x1b[92m    Finished\x1b[0m `dev` profile [unoptimized + debuginfo] target(s) in 0.42s";

fn main() {
    let console = Console::new();

    // 1. A panel around the decoded output.
    let output = Text::from_ansi(CARGO);
    console.print_renderable(&Panel::new(output.clone()).title("cargo build"));

    // 2. Decoded snippets in table cells.
    let mut table = Table::new();
    table.add_column("Tool");
    table.add_column("Output");
    table.add_row([
        Text::plain("ls --color"),
        Text::from_ansi("\x1b[01;34msrc\x1b[0m  \x1b[01;32mbuild.sh\x1b[0m  Cargo.toml"),
    ]);
    table.add_row([
        Text::plain("git diff"),
        Text::from_ansi("\x1b[31m-    let x = 1;\x1b[m\n\x1b[32m+    let x = 2;\x1b[m"),
    ]);
    table.add_row([
        Text::plain("link"),
        Text::from_ansi(
            "\x1b]8;;https://docs.rs/fast-rich\x1b\\\x1b[4mdocs.rs\x1b[0m\x1b]8;;\x1b\\",
        ),
    ]);
    console.print_renderable(&table);

    // 3. Widths are visible cells, not bytes.
    let first = Text::from_ansi(CARGO.lines().next().unwrap_or_default());
    console.println(&format!(
        "first line: {} bytes of input, {} cells on screen",
        CARGO.lines().next().unwrap_or_default().len(),
        first.width()
    ));
}
