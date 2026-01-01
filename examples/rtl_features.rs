use fast_rich::bidi::TextDirection;
use fast_rich::console::Console;
use fast_rich::panel::Panel;
use fast_rich::table::Table;


fn main() {
    let console = Console::new();
    console.print_raw("RTL Features Demo:\n\n");

    // 1. Basic Text
    console.print_raw("1. Basic Text Printing (RTL Context):\n");
    // Switch console to RTL
    let rtl_console = Console::new().direction(TextDirection::Rtl);
    
    // Note: Terminal might not handle RTL display correctly without library support.
    // fast-rich reorders spans for visual display.
    
    // Plain text "Hello" + "مرحبا"
    // Since this example might run in non-RTL terminal, output might look jumbled if not checked strictly.
    // But we trust our reordering logic.
    rtl_console.println("Hello مرحبا World");
    
    console.print_raw("\n2. RTL Table (Mirrored Columns & Borders):\n");
    let mut table = Table::new();
    table.add_column("English (Left)");
    table.add_column("Arabic (Right)");
    
    table.add_row_strs(&["One", "واحد"]);
    table.add_row_strs(&["Two", "اثنان"]);
    
    // Render using RTL console
    rtl_console.print_renderable(&table);

    console.print_raw("\n3. RTL Panel (Mirrored Title):\n");
    let panel = Panel::new("This content is inside an RTL panel.\nThe padding should be correct (visual right aligned implied? No, content aligns as per Text settings).")
        .title("RTL Title (Right)")
        .subtitle("Subtitle (Left)");
        
    rtl_console.print_renderable(&panel);
    
    console.print_raw("\nDone.\n");
}
