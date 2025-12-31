use rich_rust::prelude::*;
use rich_rust::prompt::{Prompt, Confirm, IntPrompt};

fn main() {
    let console = Console::new();

    console.rule("[bold red]Interactive Prompt Demo[/]");

    // 1. Basic Prompt
    let name: String = Prompt::new("Enter your [bold cyan]name[/]").ask();
    console.print(&format!("Hello, [bold green]{}[/]!", name));

    // 2. Default value
    let city: String = Prompt::new("Enter your city")
        .default("New York".to_string())
        .ask();
    console.print(&format!("City: [bold blue]{}[/]", city));

    // 3. Choices
    let size: String = Prompt::new("Choose a size")
        .choices(&["small".to_string(), "medium".to_string(), "large".to_string()])
        .default("medium".to_string())
        .ask();
    console.print(&format!("Size: [bold magenta]{}[/]", size));

    // 4. Secret (Password)
    console.print("Enter password (will be hidden, or just read line for now):");
    let _secret: String = Prompt::new("Password")
        .secret()
        .ask();
    
    // 5. IntPrompt
    let age = IntPrompt::new("Enter your age")
        .default(18)
        .ask();
    console.print(&format!("Age: [bold yellow]{}[/]", age));

    // 6. Confirm
    if Confirm::ask("Do you want to continue?", Some(true)) {
        console.print("[bold green]Continuing...[/]");
    } else {
        console.print("[bold red]Aborting...[/]");
    }

    console.rule("[bold red]End Demo[/]");
}
