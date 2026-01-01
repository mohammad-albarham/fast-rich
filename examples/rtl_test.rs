//! Interactive Arabic text testing
//!
//! Run with: cargo run --example rtl_test --features rtl
//! Or pipe through fribidi: cargo run --example rtl_test --features rtl | fribidi --nopad

use fast_rich::bidi::{is_rtl, mirror_string, reorder_for_display, TextDirection};

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("                   RTL Text Tester");
    println!("═══════════════════════════════════════════════════════\n");

    // Test various Arabic sentences - ADD YOUR OWN TEXT HERE!
    let test_cases = [
        // Basic greetings
        "السلام عليكم ورحمة الله وبركاته",
        "أهلاً وسهلاً بكم في مكتبتنا",
        "صباح الخير - Good morning",
        
        // Numbers and mixed content
        "السعر: 100 دولار أمريكي",
        "العدد الإجمالي: ١٢٣٤٥",
        "Version 2.0 - الإصدار الثاني",
        
        // Technical terms
        "تثبيت البرنامج باستخدام npm install",
        "قاعدة البيانات PostgreSQL جاهزة",
        "الخطأ: Error 404 - الصفحة غير موجودة",
        
        // Long paragraph
        "هذا نص طويل لاختبار كيفية عمل خوارزمية ثنائية الاتجاه مع النصوص العربية الطويلة التي تحتوي على كلمات إنجليزية مثل Fast-Rich و Rust",
        
        // Poetry / Quotes
        "قال الشاعر: الصبر مفتاح الفرج",
        "الحكمة ضالة المؤمن",
        
        // Brackets and punctuation
        "(مثال) [اختبار] {تجربة}",
        "السؤال: ما هو Rust؟ الجواب: لغة برمجة!",
    ];

    for (i, text) in test_cases.iter().enumerate() {
        println!("─────────────────────────────────────────────────────");
        println!("Test #{}", i + 1);
        println!("─────────────────────────────────────────────────────");
        println!("Original:   {}", text);
        println!("Is RTL:     {}", if is_rtl(text) { "نعم (Yes)" } else { "لا (No)" });
        println!("Reordered:  {}", reorder_for_display(text, TextDirection::Auto));
        if text.contains('(') || text.contains('[') || text.contains('{') {
            println!("Mirrored:   {}", mirror_string(text));
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════");
    println!("Add your own text by editing examples/rtl_test.rs!");
    println!("═══════════════════════════════════════════════════════");
}
