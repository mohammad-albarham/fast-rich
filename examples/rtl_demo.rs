//! RTL (Right-to-Left) language support demo.
//!
//! This example demonstrates Fast-Rich's support for Arabic text
//! using the Unicode Bidirectional Algorithm (UAX #9).
//!
//! Run with: cargo run --example rtl_demo --features rtl

use fast_rich::bidi::{is_rtl, mirror_string, reorder_for_display, TextDirection};
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    console.println("[bold blue]═══════════════════════════════════════════════════════[/]");
    console.println("[bold cyan]          Fast-Rich RTL Demo - عرض دعم اللغة العربية[/]");
    console.println("[bold blue]═══════════════════════════════════════════════════════[/]");
    console.println("");

    // Section 1: Basic Arabic Text
    console.println("[bold green]1. النص العربي الأساسي (Basic Arabic Text)[/]");
    console.println("   مرحباً بكم في مكتبة Fast-Rich");
    console.println("   أهلاً وسهلاً - Welcome!");
    console.println("");

    // Section 2: Direction Detection
    console.println("[bold green]2. اكتشاف الاتجاه (Direction Detection)[/]");

    let arabic_text = "السلام عليكم";
    let english_text = "Hello World";
    let mixed_ar_first = "مرحبا Hello";
    let mixed_en_first = "Hello مرحبا";

    println!(
        "   '{}' → RTL: {}",
        arabic_text,
        if is_rtl(arabic_text) { "نعم" } else { "لا" }
    );
    println!(
        "   '{}' → RTL: {}",
        english_text,
        if is_rtl(english_text) { "نعم" } else { "لا" }
    );
    println!(
        "   '{}' → RTL: {}",
        mixed_ar_first,
        if is_rtl(mixed_ar_first) { "نعم" } else { "لا" }
    );
    println!(
        "   '{}' → RTL: {}",
        mixed_en_first,
        if is_rtl(mixed_en_first) { "نعم" } else { "لا" }
    );
    console.println("");

    // Section 3: BiDi Reordering
    console.println("[bold green]3. إعادة ترتيب النص (BiDi Reordering)[/]");

    let mixed_text = "السعر 100 دولار";
    console.println(&format!("   نص منطقي: {}", mixed_text));
    let visual = reorder_for_display(mixed_text, TextDirection::Auto);
    console.println(&format!("   نص مرئي: {}", visual));
    console.println("");

    // Section 4: Character Mirroring
    console.println("[bold green]4. عكس الأقواس (Character Mirroring)[/]");

    let brackets = "(hello) [world] {test}";
    console.println(&format!("   أصلي: {}", brackets));
    console.println(&format!("   معكوس: {}", mirror_string(brackets)));
    console.println("");

    // Section 5: Arabic in Styled Components
    console.println("[bold green]5. المكونات المنسقة (Styled Components)[/]");
    console.println("");

    // Panel with Arabic
    let panel = Panel::new(Text::plain("أهلاً وسهلاً بكم في مكتبة Fast-Rich للتنسيق الجميل"))
        .title("رسالة ترحيبية")
        .border_style(BorderStyle::Rounded);
    console.print_renderable(&panel);
    console.println("");

    // Rule with Arabic
    let rule = Rule::new("قسم جديد").style(Style::new().foreground(Color::Cyan));
    console.print_renderable(&rule);
    console.println("");

    // Section 6: Arabic Table
    console.println("[bold green]6. جدول عربي (Arabic Table)[/]");
    console.println("");

    let mut table = Table::new()
        .columns(["الاسم", "العمر", "المدينة"])
        .border_style(BorderStyle::Rounded);

    table.add_row_strs(&["أحمد", "25", "القاهرة"]);
    table.add_row_strs(&["فاطمة", "30", "الرياض"]);
    table.add_row_strs(&["محمد", "22", "دبي"]);
    table.add_row_strs(&["عائشة", "28", "بيروت"]);

    console.print_renderable(&table);
    console.println("");

    // Section 7: Arabic Tree
    console.println("[bold green]7. شجرة عربية (Arabic Tree)[/]");
    console.println("");

    let mut root = TreeNode::new("المشروع");
    
    let mut sources = TreeNode::new("المصادر");
    sources.add(TreeNode::new("main.rs"));
    sources.add(TreeNode::new("lib.rs"));
    
    let mut docs = TreeNode::new("الوثائق");
    docs.add(TreeNode::new("README.md"));
    docs.add(TreeNode::new("API.md"));
    
    root.add(sources);
    root.add(docs);
    root.add(TreeNode::new("الاختبارات"));
    
    let tree = Tree::new(root);

    console.print_renderable(&tree);
    console.println("");

    // Section 8: Mixed Content
    console.println("[bold green]8. محتوى مختلط (Mixed Content)[/]");
    console.println("");

    console.println("   [bold]Fast-Rich[/] هي مكتبة Rust لتنسيق الطرفية");
    console.println("   تدعم [italic]Unicode[/] و [bold cyan]الألوان[/] و [underline]الجداول[/]");
    console.println("");

    // Footer
    console.println("[bold blue]═══════════════════════════════════════════════════════[/]");
    console.println("[dim]تم بنجاح! - Demo completed successfully![/]");
    console.println("[bold blue]═══════════════════════════════════════════════════════[/]");
}
