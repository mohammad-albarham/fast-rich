use fast_rich::measure::Measurable;
use fast_rich::prelude::*;

fn main() {
    let console = Console::new().width(70);
    console.rule("[bold blue]Measure API Demo[/]");

    // Example 1: Simple text measurement
    console.print("[bold]Measuring text:[/]");
    let text = Text::plain("The quick brown fox jumps over the lazy dog");
    let m1 = text.measure(60);
    console.print(&format!("  Minimum width: {}", m1.minimum));
    console.print(&format!("  Maximum width: {}", m1.maximum));
    console.print(&format!("  Lines: {}", m1.lines));
    console.print(&format!("  Aspect ratio: {:.2}", m1.aspect_ratio()));
    console.newline();

    // Example 2: Panel measurement
    console.print("[bold]Measuring panel:[/]");
    let panel = Panel::new(Text::plain("Panel content here")).title("Test Panel");
    let m2 = panel.measure(50);
    console.print(&format!("  Minimum width: {}", m2.minimum));
    console.print(&format!("  Lines: {}", m2.lines));
    console.print(&format!("  Area: {} characters", m2.area()));
    console.newline();

    // Example 3: Table measurement
    console.print("[bold]Measuring table:[/]");
    let mut table = Table::new();
    table.add_column("Name");
    table.add_column("Value");
    table.add_row_strs(&["Item1", "Value1"]);
    table.add_row_strs(&["Item2", "Value2"]);

    let m3 = table.measure(70);
    console.print(&format!("  Minimum width: {}", m3.minimum));
    console.print(&format!("  Lines: {}", m3.lines));
    console.print(&format!("  Fits in 60x10? {}", m3.fits(60, 10)));
    console.print(&format!("  Fits in 30x10? {}", m3.fits(30, 10)));
    console.newline();

    // Example 4: Comparing measurements
    console.print("[bold]Comparing measurements:[/]");
    let short_text = Text::plain("Short");
    let long_text = Text::plain("This is a much longer piece of text that will likely wrap");

    let m_short = short_text.measure(40);
    let m_long = long_text.measure(40);

    console.print(&format!(
        "  Short text: {} lines, {} chars wide",
        m_short.lines, m_short.maximum
    ));
    console.print(&format!(
        "  Long text:  {} lines, {} chars wide",
        m_long.lines, m_long.maximum
    ));
    console.newline();

    // Example 5: Fitting content
    console.print("[bold]Fitting content in constraints:[/]");
    let content = Panel::new(Text::plain("Dynamic content")).title("Flexible");

    for width in [30, 40, 50, 60] {
        let m = content.measure(width);
        console.print(&format!(
            "  At width {}: {} lines ({})",
            width,
            m.lines,
            if m.fits(width, 5) { "FITS" } else { "TOO TALL" }
        ));
    }

    console.rule("[bold blue]End Demo[/]");
}
