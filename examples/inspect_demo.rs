use rich_rust::prelude::*;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub roles: Vec<String>,
}

fn main() {
    let console = Console::new();

    // Use Rule directly
    console.print_renderable(&Rule::new("Inspect Demo"));
    console.println(
        "The [bold]inspect[/] function shows internal details of Rust objects (via Debug).",
    );
    console.println("");

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        active: true,
        roles: vec!["admin".to_string(), "editor".to_string()],
    };

    console.println("[bold yellow]Inspecting a User struct:[/]");
    rich_rust::inspect::inspect(&user, InspectConfig::default());

    console.println("");
    console.println("[bold yellow]Inspecting a standard Vector:[/]");
    let numbers = vec![1, 2, 3, 4, 5];
    rich_rust::inspect::inspect(&numbers, InspectConfig::default());
}
