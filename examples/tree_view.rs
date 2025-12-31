use fast_rich::prelude::*;

fn run(console: &Console) {
    console.rule("[bold yellow]Tree View Demo[/]");
    console.newline();

    // 1. File System Tree
    console.print("[bold]1. File System Hierarchy[/]");

    let mut root = Tree::new(TreeNode::new(fast_rich::markup::parse(
        "[bold blue]project_root/[/]",
    )));

    let mut src = TreeNode::new(fast_rich::markup::parse("[blue]src/[/]"));
    let mut core = TreeNode::new(fast_rich::markup::parse("[bold]core/[/]"));
    core.add("lib.rs");
    core.add("main.rs");
    src.add(core);
    src.add("utils.rs");

    let mut assets = TreeNode::new(fast_rich::markup::parse("[yellow]assets/[/]"));
    assets.add("logo.png");
    assets.add("styles.css");

    root.add(src);
    root.add(assets);
    root.add("README.md");
    root.add("Cargo.toml");

    console.print_renderable(&root);
    console.newline();

    // 2. Guide Styles
    console.print("[bold]2. Guide Styles[/]");
    let styles = [
        ("Unicode (Default)", fast_rich::tree::GuideStyle::Unicode),
        ("ASCII", fast_rich::tree::GuideStyle::Ascii),
        ("Bold", fast_rich::tree::GuideStyle::Bold),
        ("Double", fast_rich::tree::GuideStyle::Double),
    ];

    for (name, style) in styles {
        console.print(&format!("[dim]{}[/]", name));
        let mut t = Tree::new("Root").guide_style(style);
        t.add("Child 1");
        let mut c2 = TreeNode::new("Child 2");
        c2.add("Grandchild A");
        t.add(c2);
        console.print_renderable(&t);
        console.newline();
    }

    console.rule("[bold yellow]End Tree Demo[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_view_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();

        assert!(output.contains("Tree View Demo"));
        assert!(output.contains("project_root/"));
        assert!(output.contains("lib.rs"));
        assert!(output.contains("Guide"));
        assert!(output.contains("Styles"));
        assert!(output.contains("Unicode"));
        assert!(output.contains("Default"));
        assert!(output.contains("End Tree Demo"));
    }
}
