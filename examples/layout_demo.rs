use rich_rust::prelude::*;

pub fn run(console: &Console) {
    console.clear();

    // 1. Create the root layout (Full Screen)
    let mut layout = rich_rust::layout::Layout::new().with_name("root");

    // 2. Split into Header, Main, Footer
    layout.split_column(vec![
        // Header: Fixed height 3
        rich_rust::layout::Layout::new()
            .with_name("header")
            .with_size(3),
        // Main: Auto height (ratio 1)
        rich_rust::layout::Layout::new()
            .with_name("main")
            .with_ratio(1),
        // Footer: Fixed height 3
        rich_rust::layout::Layout::new()
            .with_name("footer")
            .with_size(3),
    ]);

    // 3. Populate Header
    layout.children_mut()[0].update(
        Panel::new(
            Text::from("Rich Rust Layout Engine")
                .alignment(Alignment::Center)
                .style(
                    Style::new()
                        .bold()
                        .foreground(Color::White)
                        .background(Color::Blue),
                ),
        )
        .border_style(BorderStyle::Heavy),
    );

    // 4. Split Main into Sidebar and Content
    layout.children_mut()[1].split_row(vec![
        // Sidebar: Ratio 1
        rich_rust::layout::Layout::new()
            .with_name("sidebar")
            .with_ratio(1),
        // Content: Ratio 3
        rich_rust::layout::Layout::new()
            .with_name("content")
            .with_ratio(3),
    ]);

    // 5. Populate Sidebar
    layout.children_mut()[1].children_mut()[0]
        .update(Panel::new(Text::from("Sidebar\n\n- Item 1\n- Item 2\n- Item 3")).title("Menu"));

    // 6. Populate Content
    layout.children_mut()[1].children_mut()[1].update(
        Panel::new(
            Text::from(
                "Main Content Area\n\nThis layout is split recursively.\nThe header and footer are fixed height.\nThe middle section splits the remaining space.\nThe middle section is further split into a sidebar (25%) and content (75%)."
            )
        )
        .title("Dashboard")
        .border_style(BorderStyle::Double)
    );

    // 7. Populate Footer
    layout.children_mut()[2].update(Panel::new(
        Text::from("Status: Online | CPU: 45% | Mem: 1.2GB").alignment(Alignment::Center),
    ));

    // Render
    console.print_renderable(&layout);
}

fn main() {
    let console = Console::new();
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_demo_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();

        // Debug output
        // eprintln!("{}", output);

        assert!(output.contains("Rich Rust Layout Engine"));
        assert!(output.contains("Sidebar"));
        assert!(output.contains("Main Content Area"));
        assert!(output.contains("Status: Online"));
    }
}
