//! Tree rendering for hierarchical data.
//!
//! Renders tree-like structures with guide lines.

use crate::console::RenderContext;
use crate::renderable::{segments_to_lines, Renderable, Segment, SharedRenderable};
use crate::style::Style;
use crate::text::{Span, Text};
use std::sync::Arc;

/// Guide style for tree connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideStyle {
    /// ASCII characters
    Ascii,
    /// Unicode box drawing (default)
    #[default]
    Unicode,
    /// Bold Unicode
    Bold,
    /// Double line
    Double,
}

impl GuideStyle {
    /// Rich's guide strings: (space, continue, fork, end).
    fn guides(&self) -> [&'static str; 4] {
        match self {
            GuideStyle::Ascii => ["    ", "|   ", "+-- ", "`-- "],
            GuideStyle::Unicode => ["    ", "│   ", "├── ", "└── "],
            GuideStyle::Bold => ["    ", "┃   ", "┣━━ ", "┗━━ "],
            GuideStyle::Double => ["    ", "║   ", "╠══ ", "╚══ "],
        }
    }
}

const SPACE: usize = 0;
const CONTINUE: usize = 1;
const FORK: usize = 2;
const END: usize = 3;

#[derive(Clone)]
pub struct TreeNode {
    /// Label for this node
    label: SharedRenderable,
    /// Child nodes
    children: Vec<TreeNode>,
    /// Style for the label
    style: Style,
    /// Whether this node is expanded
    expanded: bool,
}

impl TreeNode {
    /// Create a new tree node with a label.
    pub fn new<T: Into<Text>>(label: T) -> Self {
        Self::renderable(label.into())
    }

    /// Create a node whose label is any renderable (e.g. a panel or table).
    ///
    /// ```
    /// use fast_rich::panel::Panel;
    /// use fast_rich::tree::{Tree, TreeNode};
    /// let mut tree = Tree::new("root");
    /// tree.add(TreeNode::renderable(Panel::new("boxed")));
    /// ```
    pub fn renderable(label: impl Renderable + Send + Sync + 'static) -> Self {
        TreeNode {
            label: Arc::new(label),
            children: Vec::new(),
            style: Style::new(),
            expanded: true,
        }
    }

    /// Add a child node.
    pub fn add<T: Into<TreeNode>>(&mut self, child: T) -> &mut Self {
        self.children.push(child.into());
        self
    }

    /// Add a child node and return self (builder pattern).
    pub fn with_child<T: Into<TreeNode>>(mut self, child: T) -> Self {
        self.children.push(child.into());
        self
    }

    /// Add multiple children.
    pub fn with_children<I, T>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<TreeNode>,
    {
        for child in children {
            self.children.push(child.into());
        }
        self
    }

    /// Set the style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set whether the node is expanded.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if this node has children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("children", &self.children)
            .field("style", &self.style)
            .field("expanded", &self.expanded)
            .finish_non_exhaustive()
    }
}

impl<T: Into<Text>> From<T> for TreeNode {
    fn from(label: T) -> Self {
        TreeNode::new(label)
    }
}

/// A tree structure for hierarchical data.
#[derive(Debug, Clone)]
pub struct Tree {
    /// Root node
    root: TreeNode,
    /// Guide style
    guide_style: GuideStyle,
    /// Style for guide lines
    style: Style,
    /// Hide the root node
    hide_root: bool,
}

impl Tree {
    /// Create a new tree with a root label.
    pub fn new<T: Into<TreeNode>>(root: T) -> Self {
        Tree {
            root: root.into(),
            guide_style: GuideStyle::Unicode,
            style: Style::new(),
            hide_root: false,
        }
    }

    /// Set the guide style.
    pub fn guide_style(mut self, style: GuideStyle) -> Self {
        self.guide_style = style;
        self
    }

    /// Set the style for guide lines.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Hide the root node.
    pub fn hide_root(mut self, hide: bool) -> Self {
        self.hide_root = hide;
        self
    }

    /// Add a child to the root.
    pub fn add<T: Into<TreeNode>>(&mut self, child: T) -> &mut Self {
        self.root.children.push(child.into());
        self
    }

    /// Render `node` and its expanded children, following Rich's
    /// `Tree.__rich_console__`: `conts` holds the continuation guides of the
    /// ancestors below the root; labels wrap at the width left after the
    /// guides, and label styles are inherited from ancestors.
    #[allow(clippy::too_many_arguments)]
    fn render_node(
        &self,
        node: &TreeNode,
        depth: usize,
        last: bool,
        conts: &mut Vec<usize>,
        inherited: Style,
        width: usize,
        out: &mut Vec<Segment>,
    ) {
        let guides = self.guide_style.guides();
        let style = inherited.combine(&node.style);

        // Rich's `levels[1:]` (or `levels[2:]` with hide_root).
        let mut prefix: Vec<usize> = conts.clone();
        if depth > 0 {
            prefix.push(if last { END } else { FORK });
        }
        if self.hide_root && !prefix.is_empty() {
            prefix.remove(0);
        }

        if !(depth == 0 && self.hide_root) {
            let prefix_width = prefix.len() * 4;
            let context = RenderContext {
                width: width.saturating_sub(prefix_width).max(1),
                height: None,
            };
            let lines = segments_to_lines(node.label.render(&context));
            for (i, line) in lines.into_iter().enumerate() {
                let mut spans = Vec::with_capacity(line.len() + prefix.len());
                if !prefix.is_empty() {
                    let guide: String = prefix.iter().map(|&g| guides[g]).collect();
                    spans.push(Span::styled(guide, self.style));
                }
                spans.extend(line.into_iter().map(|mut span| {
                    span.style = style.combine(&span.style);
                    span
                }));
                out.push(Segment::line(spans));
                if i == 0 {
                    if let Some(own) = prefix.last_mut() {
                        *own = if last { SPACE } else { CONTINUE };
                    }
                }
            }
        }

        if node.expanded && !node.children.is_empty() {
            if depth > 0 {
                conts.push(if last { SPACE } else { CONTINUE });
            }
            let count = node.children.len();
            for (i, child) in node.children.iter().enumerate() {
                self.render_node(child, depth + 1, i + 1 == count, conts, style, width, out);
            }
            if depth > 0 {
                conts.pop();
            }
        }
    }

    /// Rich's tree measurement: widest label plus 4 cells per level.
    fn measure(&self, node: &TreeNode, level: usize, acc: &mut (usize, usize)) {
        let indent = level * 4;
        acc.0 = acc.0.max(node.label.min_width().saturating_add(indent));
        acc.1 = acc.1.max(node.label.max_width().saturating_add(indent));
        if node.expanded {
            for child in &node.children {
                self.measure(child, level + 1, acc);
            }
        }
    }
}

impl Renderable for Tree {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let mut segments = Vec::new();
        let mut conts = Vec::new();
        self.render_node(
            &self.root,
            0,
            true,
            &mut conts,
            Style::new(),
            context.width,
            &mut segments,
        );
        segments
    }

    fn min_width(&self) -> usize {
        let mut acc = (0, 0);
        self.measure(&self.root, 0, &mut acc);
        acc.0
    }

    fn max_width(&self) -> usize {
        let mut acc = (0, 0);
        self.measure(&self.root, 0, &mut acc);
        acc.1
    }
}

/// Options for [`from_directory_with`].
#[derive(Debug, Clone, Default)]
pub struct DirectoryTreeOptions {
    /// Maximum recursion depth (1 = only the root's entries). `None` means
    /// unlimited (subject to cycle detection).
    pub max_depth: Option<usize>,
    /// Include hidden entries (dotfiles). Default: false.
    pub include_hidden: bool,
    /// Follow directory symlinks. Default: false — symlinks are shown as
    /// entries but never recursed into, which also prevents symlink cycles.
    pub follow_symlinks: bool,
}

/// Create a tree from a directory path with default options.
pub fn from_directory(path: &std::path::Path) -> std::io::Result<Tree> {
    from_directory_with(path, DirectoryTreeOptions::default())
}

/// Create a tree from a directory path.
///
/// Symlinked directories are never followed (unless `follow_symlinks` is
/// set), so a symlink pointing at an ancestor can no longer recurse until
/// the stack overflows. Recursion is also capped by `max_depth` when set.
pub fn from_directory_with(
    path: &std::path::Path,
    options: DirectoryTreeOptions,
) -> std::io::Result<Tree> {
    use std::collections::HashSet;

    fn build_node(
        path: &std::path::Path,
        depth: usize,
        visited: &mut HashSet<std::path::PathBuf>,
        options: &DirectoryTreeOptions,
    ) -> std::io::Result<TreeNode> {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let mut node = TreeNode::new(name);

        let metadata = if options.follow_symlinks {
            std::fs::metadata(path)
        } else {
            std::fs::symlink_metadata(path)
        }?;

        if metadata.is_dir() {
            // Depth limit (root itself is depth 0; its children are depth 1).
            if let Some(max) = options.max_depth {
                if depth >= max {
                    return Ok(node);
                }
            }

            // Cycle guard: canonical path of a directory must not already be
            // on the way down.
            let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
            if !visited.insert(canonical.clone()) {
                return Ok(node);
            }

            let mut entries: Vec<_> = match std::fs::read_dir(path) {
                Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
                Err(_) => Vec::new(),
            };
            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let entry_path = entry.path();
                if !options.include_hidden {
                    if let Some(name) = entry_path.file_name() {
                        if name.to_string_lossy().starts_with('.') {
                            continue;
                        }
                    }
                }
                let child = build_node(&entry_path, depth + 1, visited, options)?;
                node.children.push(child);
            }

            visited.remove(&canonical);
        }

        Ok(node)
    }

    let mut visited = HashSet::new();
    let root = build_node(path, 0, &mut visited, &options)?;
    Ok(Tree::new(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label_plain(node: &TreeNode) -> String {
        let context = RenderContext {
            width: 200,
            height: None,
        };
        node.label
            .render(&context)
            .iter()
            .map(|s| s.plain_text())
            .collect()
    }

    #[test]
    fn test_from_directory_symlink_cycle_terminates() {
        let dir = std::env::temp_dir().join(format!("fr_tree_{}", std::process::id()));
        let inner = dir.join("inner");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&inner).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&dir, inner.join("loop")).unwrap();

        // Must terminate and not blow the stack.
        let tree = from_directory(&dir);
        assert!(tree.is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_from_directory_depth_limit() {
        let dir = std::env::temp_dir().join(format!("fr_treed_{}", std::process::id()));
        let deep = dir.join("a").join("b").join("c");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&deep).unwrap();

        let tree = from_directory_with(
            &dir,
            DirectoryTreeOptions {
                max_depth: Some(1),
                ..Default::default()
            },
        )
        .unwrap();
        // Only "a" should appear, not "b"/"c".
        fn collect(node: &TreeNode, out: &mut Vec<String>) {
            out.push(label_plain(node));
            for c in &node.children {
                collect(c, out);
            }
        }
        let mut labels = Vec::new();
        collect(&tree.root, &mut labels);
        assert!(labels.iter().any(|l| l == "a"));
        assert!(
            !labels.iter().any(|l| l == "b"),
            "depth limit violated: {labels:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_from_directory_hidden_files_excluded() {
        let dir = std::env::temp_dir().join(format!("fr_treeh_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(".secret"), "x").unwrap();
        std::fs::write(dir.join("visible.txt"), "x").unwrap();

        let tree = from_directory(&dir).unwrap();
        fn collect(node: &TreeNode, out: &mut Vec<String>) {
            out.push(label_plain(node));
            for c in &node.children {
                collect(c, out);
            }
        }
        let mut labels = Vec::new();
        collect(&tree.root, &mut labels);
        assert!(labels.iter().any(|l| l == "visible.txt"));
        assert!(!labels.iter().any(|l| l == ".secret"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_tree_simple() {
        let tree = Tree::new("root").guide_style(GuideStyle::Unicode);

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = tree.render(&context);

        assert_eq!(segments.len(), 1);
        assert!(segments[0].plain_text().contains("root"));
    }

    #[test]
    fn test_tree_with_children() {
        let mut tree = Tree::new("root");
        tree.add(TreeNode::new("child1"));
        tree.add(TreeNode::new("child2"));

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = tree.render(&context);

        assert_eq!(segments.len(), 3);
    }

    #[test]
    fn test_tree_nested() {
        let child1 = TreeNode::new("child1")
            .with_child("grandchild1")
            .with_child("grandchild2");

        let tree = Tree::new(TreeNode::new("root").with_child(child1));

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = tree.render(&context);

        assert_eq!(segments.len(), 4);

        // Check guide characters are present
        let text: String = segments.iter().map(|s| s.plain_text()).collect();
        assert!(text.contains("├"));
        assert!(text.contains("└"));
    }

    #[test]
    fn test_tree_hide_root() {
        let mut tree = Tree::new("root").hide_root(true);
        tree.add("child1");
        tree.add("child2");

        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = tree.render(&context);

        // Should not contain root
        let text: String = segments.iter().map(|s| s.plain_text()).collect();
        assert!(!text.contains("root"));
        assert!(text.contains("child1"));
    }

    fn plain(tree: &Tree, width: usize) -> String {
        let context = RenderContext {
            width,
            height: None,
        };
        tree.render(&context)
            .iter()
            .map(|s| s.plain_text().trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_matches_rich_nested() {
        let mut a = TreeNode::new("a");
        a.add(TreeNode::new("a1").with_child("deep"));
        a.add("a2");
        let mut tree = Tree::new("root");
        tree.add(a);
        tree.add("b");
        assert_eq!(
            plain(&tree, 30),
            "root\n├── a\n│   ├── a1\n│   │   └── deep\n│   └── a2\n└── b"
        );
    }

    #[test]
    fn test_matches_rich_hide_root() {
        let mut tree = Tree::new("root").hide_root(true);
        tree.add(TreeNode::new("a").with_child("a1"));
        tree.add("b");
        assert_eq!(plain(&tree, 30), "a\n└── a1\nb");
    }

    #[test]
    fn test_matches_rich_wrapped_and_renderable_labels() {
        let mut tree = Tree::new("root");
        tree.add("a long label that has to wrap around");
        tree.add(TreeNode::renderable(crate::panel::Panel::new("box")));
        assert_eq!(
            plain(&tree, 30),
            "root\n├── a long label that has to\n│   wrap around\n└── ╭─────╮\n    │ box │\n    ╰─────╯"
        );
    }

    #[test]
    fn test_bold_guides_and_measure() {
        let mut tree = Tree::new("root").guide_style(GuideStyle::Bold);
        tree.add(TreeNode::new("x").with_child("y"));
        assert_eq!(plain(&tree, 30), "root\n┗━━ x\n    ┗━━ y");
        assert_eq!(tree.max_width(), 9);
    }

    #[test]
    fn test_label_style_inherited() {
        use crate::style::Color;
        let mut tree = Tree::new(TreeNode::new("root").style(Style::new().foreground(Color::Red)));
        tree.add("child");
        let segs = tree.render(&RenderContext {
            width: 30,
            height: None,
        });
        let child = segs[1].spans.iter().find(|s| s.text == "child").unwrap();
        assert_eq!(child.style.foreground, Some(Color::Red));
    }
}
