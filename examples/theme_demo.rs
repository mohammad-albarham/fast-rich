use fast_rich::prelude::*;
use fast_rich::theme::Theme;

fn main() {
    let console = Console::new().width(70);
    console.rule("[bold blue]Theme Demo[/]");

    // Default theme
    console.print("[bold]Default Theme:[/]");
    let theme = Theme::default_theme();
    console.print_renderable(&Text::plain("Primary").style(theme.get_style("primary")));
    console.print_renderable(&Text::plain("Success").style(theme.get_style("success")));
    console.print_renderable(&Text::plain("Warning").style(theme.get_style("warning")));
    console.print_renderable(&Text::plain("Error").style(theme.get_style("error")));
    console.newline();

    // Monokai theme
    console.print("[bold]Monokai Theme:[/]");
    let monokai = Theme::monokai();
    console.print_renderable(&Text::plain("Primary").style(monokai.get_style("primary")));
    console.print_renderable(&Text::plain("Success").style(monokai.get_style("success")));
    console.print_renderable(&Text::plain("Warning").style(monokai.get_style("warning")));
    console.print_renderable(&Text::plain("Error").style(monokai.get_style("error")));
    console.newline();

    // Night Owl theme
    console.print("[bold]Night Owl Theme:[/]");
    let night_owl = Theme::night_owl();
    console.print_renderable(&Text::plain("Primary").style(night_owl.get_style("primary")));
    console.print_renderable(&Text::plain("Success").style(night_owl.get_style("success")));
    console.print_renderable(&Text::plain("Warning").style(night_owl.get_style("warning")));
    console.print_renderable(&Text::plain("Error").style(night_owl.get_style("error")));

    console.rule("[bold blue]End Demo[/]");
}
