fn main() {
    let v: Vec<String> = vec!["admin".to_string(), "editor".to_string()];
    let formatted = format!("roles: {:?}", v);
    println!("Direct Rust print: {}", formatted);

    // Now through rich-rust
    use rich_rust::Console;
    let console = Console::new();
    console.print("Through console.println: ");
    console.println(&formatted);
}
