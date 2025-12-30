//! Tree example demonstrating hierarchical data display.

use rich_rust::prelude::*;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Tree Examples ═══[/]");
    console.println("");

    // Basic tree
    console.println("[dim]Basic tree:[/]");
    let mut tree = Tree::new("Root");
    tree.add("Child 1");
    tree.add("Child 2");
    tree.add("Child 3");
    console.print_renderable(&tree);

    console.println("");

    // Nested tree
    console.println("[dim]Nested tree:[/]");
    let child1 = TreeNode::new("src")
        .with_child("main.rs")
        .with_child("lib.rs")
        .with_child(
            TreeNode::new("modules")
                .with_child("auth.rs")
                .with_child("database.rs")
                .with_child("api.rs"),
        );

    let child2 = TreeNode::new("tests")
        .with_child("integration_tests.rs")
        .with_child("unit_tests.rs");

    let child3 = TreeNode::new("docs")
        .with_child("README.md")
        .with_child("CHANGELOG.md")
        .with_child("API.md");

    let project = Tree::new(
        TreeNode::new(":folder: my-project")
            .with_child(child1)
            .with_child(child2)
            .with_child(child3)
            .with_child("Cargo.toml")
            .with_child("Cargo.lock")
            .with_child(".gitignore"),
    );

    console.print_renderable(&project);

    console.println("");
    console.println("[dim]Different guide styles:[/]");
    console.println("");

    // ASCII guides
    console.println("[dim]ASCII:[/]");
    let tree = Tree::new(
        TreeNode::new("Root")
            .with_child("A")
            .with_child("B")
            .with_child("C"),
    )
    .guide_style(GuideStyle::Ascii);
    console.print_renderable(&tree);

    console.println("");
    console.println("[dim]Bold:[/]");
    let tree = Tree::new(
        TreeNode::new("Root")
            .with_child("A")
            .with_child("B")
            .with_child("C"),
    )
    .guide_style(GuideStyle::Bold);
    console.print_renderable(&tree);

    console.println("");
    console.println("[dim]Double:[/]");
    let tree = Tree::new(
        TreeNode::new("Root")
            .with_child("A")
            .with_child("B")
            .with_child("C"),
    )
    .guide_style(GuideStyle::Double);
    console.print_renderable(&tree);

    console.println("");
    console.println("[bold cyan]═══ Organization Chart ═══[/]");
    console.println("");

    // Organization chart example
    let engineering = TreeNode::new(":bust_in_silhouette: [bold]VP Engineering[/]")
        .with_child(
            TreeNode::new(":bust_in_silhouette: Frontend Lead")
                .with_child("Developer 1")
                .with_child("Developer 2"),
        )
        .with_child(
            TreeNode::new(":bust_in_silhouette: Backend Lead")
                .with_child("Developer 3")
                .with_child("Developer 4")
                .with_child("Developer 5"),
        )
        .with_child(
            TreeNode::new(":bust_in_silhouette: DevOps Lead")
                .with_child("SRE 1")
                .with_child("SRE 2"),
        );

    let org = Tree::new(
        TreeNode::new(":star: [bold yellow]CEO[/]")
            .with_child(engineering)
            .with_child(
                TreeNode::new(":bust_in_silhouette: [bold]VP Sales[/]")
                    .with_child("Sales Rep 1")
                    .with_child("Sales Rep 2"),
            )
            .with_child(
                TreeNode::new(":bust_in_silhouette: [bold]VP Marketing[/]")
                    .with_child("Content Manager")
                    .with_child("Social Media"),
            ),
    );

    console.print_renderable(&org);

    console.println("");
    console.println("[bold cyan]═══ Menu Structure ═══[/]");
    console.println("");

    // Menu/navigation example
    let menu = Tree::new(
        TreeNode::new(":hamburger: [bold]Main Menu[/]")
            .with_child(
                TreeNode::new(":file_folder: File")
                    .with_child("New")
                    .with_child("Open")
                    .with_child("Save")
                    .with_child("Exit"),
            )
            .with_child(
                TreeNode::new(":pencil: Edit")
                    .with_child("Undo")
                    .with_child("Redo")
                    .with_child("Cut")
                    .with_child("Copy")
                    .with_child("Paste"),
            )
            .with_child(
                TreeNode::new(":eye: View")
                    .with_child("Zoom In")
                    .with_child("Zoom Out")
                    .with_child("Full Screen"),
            )
            .with_child(
                TreeNode::new(":question: Help")
                    .with_child("Documentation")
                    .with_child("About"),
            ),
    );

    console.print_renderable(&menu);
}
