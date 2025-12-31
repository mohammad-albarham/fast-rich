use fast_rich::prelude::*;

fn run(console: &Console) {
    console.rule("[bold magenta]Text & Styles Demo[/]");
    console.newline();

    // Basic Colors
    console.print("  [bold]Standard Colors:[/]");
    console.print("  [black]black[/] [red]red[/] [green]green[/] [yellow]yellow[/] [blue]blue[/] [magenta]magenta[/] [cyan]cyan[/] [white]white[/]");
    console.print("  [bright_black]bright_black[/] [bright_red]bright_red[/] [bright_green]bright_green[/] [bright_yellow]bright_yellow[/] [bright_blue]bright_blue[/] [bright_magenta]bright_magenta[/] [bright_cyan]bright_cyan[/] [bright_white]bright_white[/]");
    console.newline();

    // Attributes
    console.print("  [bold]Attributes:[/]");
    console.print("  [bold]bold [/] [italic]italic [/] [underline]underline [/] [dim]dim [/] [reverse]reverse [/] [strike]strike [/] [blink]blink [/] [hidden]hidden[/]");
    console.newline();

    // Parsing specific
    console.print("  [bold]Nested & Parsing:[/]");
    console.print("  Current style: [bold]bold[/] [red]red[/] [underline]underline[/] no-underline no-red no-bold");
    console.print("  Escaped tags: [[bold]] is not bold");
    console.newline();

    // Alignment
    console.print("  [bold]Text Alignment (20 chars):[/]");
    console.print(&format!("  |{:^20}| (Center)", "Center"));
    console.print(&format!("  |{:<20}| (Left)", "Left"));
    console.print(&format!("  |{:>20}| (Right)", "Right"));
    console.newline();

    // Truecolor
    console.print("  [bold]Truecolor (RGB):[/]");
    for i in 0..10 {
        let r = 255;
        let g = i * 25;
        let b = 0;
        console.print(&format!(
            "  [rgb({},{},{})]RGB Gradient Step {}[/]",
            r, g, b, i
        ));
    }
    console.newline();

    // Backgrounds
    console.print("  [bold]Backgrounds:[/]");
    console.print("  [white on red] CRITICAL [/] [black on yellow] WARNING [/] [white on green] SUCCESS [/] [white on blue] INFO [/]");

    console.rule("[bold magenta]End Text Demo[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_styles_demo_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();

        assert!(output.contains("Text & Styles Demo"));
        assert!(output.contains("Standard"));
        assert!(output.contains("Colors"));
        assert!(output.contains("Attributes"));
        assert!(output.contains("RGB Gradient Step 9"));
        assert!(output.contains("End Text Demo"));
    }
}
