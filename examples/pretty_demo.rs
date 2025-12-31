use rich_rust::prelude::*;

#[allow(dead_code)] // Fields are accessed via Debug trait
#[derive(Debug)]
struct User {
    id: usize,
    name: String,
    roles: Vec<String>,
    active: bool,
    metadata: std::collections::HashMap<String, String>,
}

fn main() {
    let console = Console::new();
    console.rule("[bold magenta]Pretty Print Demo[/]");

    let mut metadata = std::collections::HashMap::new();
    metadata.insert("login".to_string(), "2023-01-01".to_string());
    metadata.insert("ip".to_string(), "127.0.0.1".to_string());

    let user = User {
        id: 42,
        name: "Alice".to_string(),
        roles: vec!["admin".to_string(), "editor".to_string()],
        active: true,
        metadata,
    };

    console.print("[bold]Standard Debug:[/]");
    console.println(&format!("{:?}", user));

    console.newline();
    console.print("[bold]Pretty Debug (pprint):[/]");
    console.print_debug(&user);

    console.rule("[bold magenta]End Demo[/]");
}
