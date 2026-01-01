use fast_rich::console::Console;
use fast_rich::bidi::TextDirection;
use fast_rich::style::{Style, Color};
use fast_rich::text::{Text, Span};

fn main() {
    let console = Console::new();
    let rtl_console = Console::new().direction(TextDirection::Rtl);

    console.print_raw("RTL Styling Demo:\n\n");

    // 1. Mixed Styles in Arabic
    // "Peace be upon you" -> "As-salamu alaykum"
    // We will color "Peace" (Salam) in Green and "You" (Alaykum) in Blue.
    // Arabic: السلام عليكم
    // Salam: السلام
    // Alaykum: عليكم
    
    let salam = Span::styled("السلام", Style::new().foreground(Color::Green).bold());
    let space = Span::raw(" ");
    let alaykum = Span::styled("عليكم", Style::new().foreground(Color::Blue));
    
    let mut text = Text::from_spans(vec![salam, space, alaykum]);
    // Set text direction to RTL explicitly (or let Console handle it)
    text.direction = TextDirection::Rtl;
    
    console.print_raw("1. Styled Arabic Spans (Green Bold 'Peace', Blue 'Upon You'):\n");
    rtl_console.print_renderable(&text);

    // 2. Mixed English/Arabic with Styles
    console.print_raw("\n2. Mixed English (Red) and Arabic (Yellow):\n");
    let eng = Span::styled("Hello", Style::new().foreground(Color::Red));
    let ar = Span::styled("مرحبا", Style::new().foreground(Color::Yellow));
    
    let mixed = Text::from_spans(vec![
        eng,
        Span::raw(" | "),
        ar
    ]);
    
    rtl_console.print_renderable(&mixed);



    // 3. Background Colors and Decorations
    console.print_raw("\n3. Backgrounds & Decorations (Blue BG, Underlined):\n");
    // "Important Note:" in Arabic
    let important = Span::styled("ملاحظة هامة:", Style::new().background(Color::Blue).underline().bold().foreground(Color::White));
    let content = Span::styled(" يجب الانتباه إلى التنسيق.", Style::new().italic());
    let note = Text::from_spans(vec![important, content]);
    rtl_console.print_renderable(&note);

    // 4. Numbers and Punctuation
    console.print_raw("\n4. Numbers & Punctuation (Should be ordered correct in RTL context):\n");
    // "Year 2024" -> "سنة 2024"
    // Numbers usually stay LTR (Left Most Digit is Most Significant), but RTL paragraph might place them?
    // In RTL: [Text] [Number]. 
    // "Page 1 of 10" -> "صفحة 1 من 10"
    let page_text = Text::from_spans(vec![
        Span::raw("صفحة "),
        Span::styled("1", Style::new().foreground(Color::Cyan).bold()),
        Span::raw(" من "),
        Span::styled("10", Style::new().foreground(Color::Cyan).bold()),
        Span::raw("."),
    ]);
    rtl_console.print_renderable(&page_text);

    // 5. Complex Sentence with Embedded English
    console.print_raw("\n5. Complex Sentence with Embedded English:\n");
    // "The library [Fast-Rich] supports [RTL] correctly."
    // Arabic: "تدعم مكتبة Fast-Rich الكتابة من اليمين RTL بشكل صحيح."
    let complex = Text::from_spans(vec![
        Span::raw("تدعم مكتبة "),
        Span::styled("Fast-Rich", Style::new().foreground(Color::Magenta).bold()),
        Span::raw(" الكتابة من اليمين "),
        Span::styled("RTL", Style::new().foreground(Color::Yellow).bold()),
        Span::raw(" بشكل صحيح."),
    ]);
    rtl_console.print_renderable(&complex);

    console.print_raw("\nDone.\n");
}
