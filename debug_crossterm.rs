use crossterm::style::{Color, SetForegroundColor};
use crossterm::ExecutableCommand;
use std::io::stdout;

fn main() {
    let _ = stdout().execute(SetForegroundColor(Color::DarkRed));
    // Let's also check strict ANSI output
    print!("Red\x1b[0m\n");
}
