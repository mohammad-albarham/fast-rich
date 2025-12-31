use rich_rust::pager::Pager;

fn main() {
    println!("=== Pager Demo ===\n");
    println!("This demo shows how to page through large content.");
    println!("Press Enter to start the pager...\n");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    // Generate large content
    let mut content = String::new();
    content.push_str("Rich-Rust Interactive Pager\n");
    content.push_str("===========================\n\n");

    for i in 1..=200 {
        content.push_str(&format!(
            "Line {:3}: This is line number {} of the demonstration content.\n",
            i, i
        ));

        if i % 20 == 0 {
            content.push_str("\n--- Section Break ---\n\n");
        }
    }

    content.push_str("\n=== End of Content ===\n");
    content.push_str("You've reached the end!\n");

    // Show in pager
    let mut pager = Pager::new(content);
    match pager.show() {
        Ok(_) => println!("\nPager closed successfully."),
        Err(e) => eprintln!("\nError running pager: {}", e),
    }

    println!("Press Enter to exit...");
    std::io::stdin().read_line(&mut input).ok();
}
