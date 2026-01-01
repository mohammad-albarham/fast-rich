//! RTL (Right-to-Left) language support demo - Plain text version.
//!
//! This version has NO ANSI colors, designed to work with fribidi:
//!   cargo run --example rtl_demo_plain --features rtl | fribidi --nopad
//!
//! For colored output in a BiDi-supporting terminal, use rtl_demo instead.

use fast_rich::bidi::{is_rtl, mirror_string, reorder_for_display, TextDirection};

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("          Fast-Rich RTL Demo - عرض دعم اللغة العربية");
    println!("═══════════════════════════════════════════════════════");
    println!();

    // Section 1: Basic Arabic Text
    println!("1. النص العربي الأساسي (Basic Arabic Text)");
    println!("   مرحباً بكم في مكتبة Fast-Rich");
    println!("   أهلاً وسهلاً - Welcome!");
    println!();

    // Section 2: Direction Detection
    println!("2. اكتشاف الاتجاه (Direction Detection)");

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
    println!();

    // Section 3: BiDi Reordering
    println!("3. إعادة ترتيب النص (BiDi Reordering)");

    let mixed_text = "السعر 100 دولار";
    println!("   نص منطقي: {}", mixed_text);
    let visual = reorder_for_display(mixed_text, TextDirection::Auto);
    println!("   نص مرئي: {}", visual);
    println!();

    // Section 4: Character Mirroring
    println!("4. عكس الأقواس (Character Mirroring)");

    let brackets = "(hello) [world] {test}";
    println!("   أصلي: {}", brackets);
    println!("   معكوس: {}", mirror_string(brackets));
    println!();

    // Section 5: Arabic Table (simple)
    println!("5. جدول عربي (Arabic Table)");
    println!();
    println!("┌─────────┬───────┬─────────┐");
    println!("│ الاسم   │ العمر │ المدينة  │");
    println!("├─────────┼───────┼─────────┤");
    println!("│ أحمد    │ 25    │ القاهرة │");
    println!("│ محمد    │ 22    │     دبي │");
    println!("│ عائشة   │ 28    │   بيروت │");
    println!("└─────────┴───────┴─────────┘");
    println!();
    // Section 6: Arabic Tree (simple)
    println!("6. شجرة عربية (Arabic Tree)");
    println!();
    println!("المشروع");
    println!("├── المصادر");
    println!("│   ├── main.rs");
    println!("│   └── lib.rs");
    println!("├── الوثائق");
    println!("│   ├── README.md");
    println!("│   └── API.md");
    println!("└── الاختبارات");
    println!();

    // Section 7: Mixed Content
    println!("7. محتوى مختلط (Mixed Content)");
    println!();
    println!("   Fast-Rich هي مكتبة Rust لتنسيق الطرفية");
    println!("   تدعم Unicode و الألوان و الجداول");
    println!();

    // Footer
    println!("═══════════════════════════════════════════════════════");
    println!("تم بنجاح! - Demo completed successfully!");
    println!("═══════════════════════════════════════════════════════");
}
