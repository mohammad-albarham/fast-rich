use fast_rich::console::{Console, RenderContext};
use fast_rich::layout::Layout;
use fast_rich::panel::Panel;
use fast_rich::renderable::Renderable;

fn main() {
    let console = Console::new();
    
    // Create a 3-row layout: Header (Fixed), Body (Flexible), Footer (Fixed)
    let mut root = Layout::new();
    root.split_column(vec![
        Layout::new().with_size(3).with_name("Header"), // 3 lines high
        Layout::new().with_ratio(1).with_name("Body"),  // Fills remaining
        Layout::new().with_size(3).with_name("Footer"), // 3 lines high
    ]);

    // Add content
    let header_panel = Panel::new("Rich Rust TUI Demo")
        .title("Welcome");
    
    // Body will be split horizontally
    let mut body_layout = Layout::new();
    body_layout.split_row(vec![
        Layout::new().with_ratio(1).with_name("Left"),
        Layout::new().with_ratio(2).with_name("Right"),
    ]);

    let left_panel = Panel::new("Sidebar\nMenu Item 1\nMenu Item 2");
    let right_panel = Panel::new("Main Content Area\n\nThis area should fill the remaining height vertically.");

    body_layout.children_mut()[0].update(left_panel);
    body_layout.children_mut()[1].update(right_panel);

    let footer_panel = Panel::new("Status: Ready");

    root.children_mut()[0].update(header_panel);
    root.children_mut()[1].update(body_layout);
    root.children_mut()[2].update(footer_panel);

    console.print_raw("Rendering with Fixed Height of 20 lines:\n");
    console.print_raw("----------------------------------------\n");

    // Manually render with a constrained context to simulate a TUI frame
    // We expect:
    // Header: 3 lines
    // Footer: 3 lines
    // Body: 20 - 3 - 3 = 14 lines
    let context = RenderContext { 
        width: 80, 
        height: Some(20) 
    };
    
    let segments = root.render(&context);
    
    // Print the result line by line to verify structure
    for segment in segments {
        console.print_raw(&segment.plain_text());
        console.newline();
    }
    
    console.print_raw("----------------------------------------\n");
}
