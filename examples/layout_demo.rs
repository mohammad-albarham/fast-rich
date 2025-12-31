use rich_rust::console::Console;
use rich_rust::layout::Layout;
use rich_rust::panel::Panel;
use rich_rust::text::Text;

fn main() {
    let console = Console::new();

    // Create a complex layout
    // Root: Header (fixed 3), Body (flex), Footer (fixed 3)
    let mut root = Layout::new();
    
    let header = Layout::new()
        .with_size(3)
        .with_name("header");
    // We need to update content. 
    // Wait, Layout::update takes a Renderable. Panel is Renderable.
    let mut header = header;
    header.update(Panel::new(Text::from("Header")));

    let footer = Layout::new()
        .with_size(3)
        .with_name("footer");
    let mut footer = footer;
    footer.update(Panel::new(Text::from("Footer")));

    let mut body = Layout::new().with_ratio(1).with_name("body");
    
    // Body Split: Sidebar (fixed 20), Main (ratio 2), Right (ratio 1, min 15)
    let sidebar = Layout::new().with_size(20).with_name("sidebar");
    let mut sidebar = sidebar;
    sidebar.update(Panel::new(Text::from("Sidebar\nFixed 20")));

    let main_content = Layout::new().with_ratio(2).with_name("main");
    let mut main_content = main_content;
    main_content.update(Panel::new(Text::from("Main Content\nRatio 2")));

    let right = Layout::new().with_ratio(1).with_minimum_size(15).with_name("right");
    let mut right = right;
    right.update(Panel::new(Text::from("Right\nRatio 1\nMin 15")));

    body.split_row(vec![sidebar, main_content, right]);

    root.split_column(vec![header, body, footer]);

    console.print_renderable(&root);
}
