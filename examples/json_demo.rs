//! Example demonstrating the JSON renderer with syntax highlighting.
//!
//! Run with: cargo run --example json_demo

use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    console.print("[bold cyan]═══ JSON Renderer Demo ═══[/]\n");

    // 1. Simple JSON from string
    console.print("[bold]1. Simple JSON Object:[/]");
    let json = Json::from_str(r#"{"name": "Alice", "age": 30, "active": true}"#).unwrap();
    console.print_renderable(&json);
    console.print("");

    // 2. Nested JSON
    console.print("[bold]2. Nested JSON with Array:[/]");
    let json = Json::from_str(
        r#"{
  "user": {
    "name": "Bob",
    "email": "bob@example.com"
  },
  "scores": [95, 87, 92],
  "metadata": null
}"#,
    )
    .unwrap();
    console.print_renderable(&json);
    console.print("");

    // 3. Sorted keys
    console.print("[bold]3. Sorted Keys (z, m, a → a, m, z):[/]");
    let json = Json::from_str(r#"{"z": 26, "m": 13, "a": 1}"#)
        .unwrap()
        .sort_keys();
    console.print_renderable(&json);
    console.print("");

    // 4. Custom indent (4 spaces)
    console.print("[bold]4. Custom Indentation (4 spaces):[/]");
    let json = Json::from_str(r#"{"level1": {"level2": "value"}}"#)
        .unwrap()
        .indent(4);
    console.print_renderable(&json);
    console.print("");

    // 5. Tab indent
    console.print("[bold]5. Tab Indentation:[/]");
    let json = Json::from_str(r#"{"level1": {"level2": "value"}}"#)
        .unwrap()
        .indent_with("\t");
    console.print_renderable(&json);
    console.print("");

    // 6. Compact mode (single line)
    console.print("[bold]6. Compact Mode (no indentation):[/]");
    let json = Json::from_str(r#"{"a": 1, "b": 2, "c": [1, 2, 3]}"#)
        .unwrap()
        .compact();
    console.print_renderable(&json);
    console.print("");

    // 7. Ensure ASCII (escape unicode)
    console.print("[bold]7. Ensure ASCII (emoji escaped):[/]");
    let json = Json::from_str(r#"{"emoji": "😀🎉", "greeting": "Hello!"}"#)
        .unwrap()
        .ensure_ascii();
    console.print_renderable(&json);
    console.print("");

    // 8. From data (using serde)
    console.print("[bold]8. From Rust Struct:[/]");
    use serde::Serialize;

    #[derive(Serialize)]
    struct Config {
        version: String,
        debug: bool,
        max_connections: u32,
        features: Vec<String>,
    }

    let config = Config {
        version: "1.0.0".to_string(),
        debug: true,
        max_connections: 100,
        features: vec!["json".into(), "highlight".into(), "compact".into()],
    };

    let json = Json::from_data(&config).unwrap();
    console.print_renderable(&json);
    console.print("");

    // 9. Chained options
    console.print("[bold]9. Chained Options (sort + 4 spaces + ensure_ascii):[/]");
    let json = Json::from_str(r#"{"zzz": "你好", "aaa": "world"}"#)
        .unwrap()
        .sort_keys()
        .indent(4)
        .ensure_ascii();
    console.print_renderable(&json);
    console.print("");

    console.print("[bold green]✓[/] JSON rendering complete!");
    console.print("");
    console.print("[dim]Features demonstrated: from_str, from_data, sort_keys, indent,");
    console.print("indent_with (tab), compact, ensure_ascii, chained options[/]");
}
