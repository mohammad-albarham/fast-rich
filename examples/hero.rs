//! Simple hero example for documentation screenshots
use fast_rich::prelude::*;

fn main() {
    let console = Console::new().force_color(true);

    console.print("[bold cyan]Fast-Rich[/] - Beautiful terminal output for :crab: Rust programming language :crab:!\n");

    // Styled text
    console.print("[bold]Bold[/], [italic]italic[/], [red]colors[/], [underline]underline[/], [dim]dim[/], [reverse]reverse[/], [strike]strikethrough[/], emojis :rocket: :sparkling_heart:! and more!\n");
}
