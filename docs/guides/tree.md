# Tree Views

Display hierarchical data with customizable guide styles using the `Tree` component.

## Quick Example

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    let mut tree = Tree::new(Text::plain("project/"));
    tree.add("Cargo.toml");
    tree.add("README.md");
    
    let mut src = TreeNode::new(Text::plain("src/"));
    src.add("main.rs");
    src.add("lib.rs");
    tree.add(src);
    
    console.print_renderable(&tree);
}
```

**Output:**
```
project/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── lib.rs
```

---

## Building Trees

### Root Node

Create a tree with a root label:

```rust
let tree = Tree::new(Text::plain("Root"));
```

### Adding Children

Add simple text children:

```rust
tree.add("Child 1");
tree.add("Child 2");
```

### Nested Nodes

Create subtrees with `TreeNode`:

```rust
let mut subtree = TreeNode::new(Text::plain("Parent"));
subtree.add("Nested Child 1");
subtree.add("Nested Child 2");

tree.add(subtree);
```

### Deep Nesting

```rust
let mut root = Tree::new(Text::plain("Root"));

let mut level1 = TreeNode::new(Text::plain("Level 1"));
let mut level2 = TreeNode::new(Text::plain("Level 2"));
level2.add("Level 3 item");

level1.add(level2);
root.add(level1);
```

---

## Guide Styles

Customize the tree guide lines:

=== "Unicode (Default)"

    ```rust
    tree.guide_style(GuideStyle::Unicode);
    ```
    ```
    Root
    ├── Child 1
    └── Child 2
        └── Grandchild
    ```

=== "ASCII"

    ```rust
    tree.guide_style(GuideStyle::Ascii);
    ```
    ```
    Root
    +-- Child 1
    \-- Child 2
        \-- Grandchild
    ```

=== "Bold"

    ```rust
    tree.guide_style(GuideStyle::Bold);
    ```
    ```
    Root
    ┣━━ Child 1
    ┗━━ Child 2
        ┗━━ Grandchild
    ```

=== "Double"

    ```rust
    tree.guide_style(GuideStyle::Double);
    ```
    ```
    Root
    ╠══ Child 1
    ╚══ Child 2
        ╚══ Grandchild
    ```

---

## Styled Trees

Apply styles to node labels:

```rust
use fast_rich::prelude::*;

let mut tree = Tree::new(
    fast_rich::markup::parse("[bold blue]project/[/]")
);

tree.add(fast_rich::markup::parse("[green]src/[/]"));
tree.add(fast_rich::markup::parse("[dim]Cargo.lock[/]"));
tree.add(fast_rich::markup::parse("[yellow]Cargo.toml[/]"));
```

---

## File System Example

```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    
    let mut tree = Tree::new(
        fast_rich::markup::parse("[bold]my_project/[/]")
    );
    
    // Source directory
    let mut src = TreeNode::new(
        fast_rich::markup::parse("[blue]src/[/]")
    );
    
    let mut core = TreeNode::new(
        fast_rich::markup::parse("[blue]core/[/]")
    );
    core.add("lib.rs");
    core.add("main.rs");
    
    src.add(core);
    src.add("utils.rs");
    tree.add(src);
    
    // Assets
    let mut assets = TreeNode::new(
        fast_rich::markup::parse("[blue]assets/[/]")
    );
    assets.add("logo.png");
    assets.add("styles.css");
    tree.add(assets);
    
    // Root files
    tree.add("README.md");
    tree.add("Cargo.toml");
    
    console.print_renderable(&tree);
}
```

**Output:**
```
my_project/
├── src/
│   ├── core/
│   │   ├── lib.rs
│   │   └── main.rs
│   └── utils.rs
├── assets/
│   ├── logo.png
│   └── styles.css
├── README.md
└── Cargo.toml
```

---

## Real Terminal Output

!!! example "Run the tree demo"

    **Command:**
    ```bash
    cargo run --example tree_view
    ```

    **What you'll see:**

    ![Tree Demo](../assets/tree.png)

---

## Tips

!!! tip "Use Icons"
    Add emoji or icons for visual distinction:
    ```rust
    tree.add("📁 src/");
    tree.add("📄 README.md");
    tree.add("⚙️ Cargo.toml");
    ```

!!! tip "Color Code File Types"
    Use different colors for different file types:
    ```rust
    // Directories in blue
    fast_rich::markup::parse("[blue]src/[/]")
    
    // Rust files in orange
    fast_rich::markup::parse("[color(255,140,0)]main.rs[/]")
    
    // Config files in yellow
    fast_rich::markup::parse("[yellow]Cargo.toml[/]")
    ```
