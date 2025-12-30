//! Tree rendering for hierarchical data.
//!
//! Renders tree-like structures with guide lines.

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::style::Style;
use crate::text::{Span, Text};

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
    fn chars(&self) -> TreeGuideChars {
        match self {
            GuideStyle::Ascii => TreeGuideChars {
                vertical: '|',
                horizontal: '-',
                branch: '+',
                last_branch: '\\',
                space: ' ',
            },
            GuideStyle::Unicode => TreeGuideChars {
                vertical: '│',
                horizontal: '─',
                branch: '├',
                last_branch: '└',
                space: ' ',
            },
            GuideStyle::Bold => TreeGuideChars {
                vertical: '┃',
                horizontal: '━',
                branch: '┣',
                last_branch: '┗',
                space: ' ',
            },
            GuideStyle::Double => TreeGuideChars {
                vertical: '║',
                horizontal: '═',
                branch: '╠',
                last_branch: '╚',
                space: ' ',
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TreeGuideChars {
    vertical: char,
    horizontal: char,
    branch: char,
    last_branch: char,
    #[allow(dead_code)]
    space: char,
}

/// A node in a tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Label for this node
    label: Text,
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
        TreeNode {
            label: label.into(),
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

    fn render_node(
        &self,
        node: &TreeNode,
        prefix: &str,
        is_last: bool,
        is_root: bool,
        chars: &TreeGuideChars,
        segments: &mut Vec<Segment>,
    ) {
        if !is_root || !self.hide_root {
            let mut spans = Vec::new();

            if !is_root {
                // Add the prefix and branch character
                spans.push(Span::styled(prefix.to_string(), self.style));

                let branch_char = if is_last {
                    chars.last_branch
                } else {
                    chars.branch
                };

                spans.push(Span::styled(
                    format!("{}{}{} ", branch_char, chars.horizontal, chars.horizontal),
                    self.style,
                ));
            }

            // Add the label
            for span in &node.label.spans {
                let combined_style = node.style.combine(&span.style);
                spans.push(Span::styled(span.text.to_string(), combined_style));
            }

            segments.push(Segment::line(spans));
        }

        // Render children
        if node.expanded {
            let child_count = node.children.len();
            for (i, child) in node.children.iter().enumerate() {
                let is_last_child = i == child_count - 1;

                let new_prefix = if is_root {
                    String::new()
                } else {
                    let connector = if is_last {
                        "    ".to_string()
                    } else {
                        format!("{}   ", chars.vertical)
                    };
                    format!("{}{}", prefix, connector)
                };

                self.render_node(child, &new_prefix, is_last_child, false, chars, segments);
            }
        }
    }
}

impl Renderable for Tree {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        let chars = self.guide_style.chars();
        let mut segments = Vec::new();

        self.render_node(&self.root, "", true, true, &chars, &mut segments);

        segments
    }
}

/// Create a tree from a directory path (example helper).
#[cfg(feature = "std")]
pub fn from_directory(path: &std::path::Path) -> std::io::Result<Tree> {
    fn build_node(path: &std::path::Path) -> std::io::Result<TreeNode> {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let mut node = TreeNode::new(name);

        if path.is_dir() {
            let mut entries: Vec<_> = std::fs::read_dir(path)?.filter_map(|e| e.ok()).collect();

            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let child = build_node(&entry.path())?;
                node.children.push(child);
            }
        }

        Ok(node)
    }

    let root = build_node(path)?;
    Ok(Tree::new(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_simple() {
        let tree = Tree::new("root").guide_style(GuideStyle::Unicode);

        let context = RenderContext { width: 40 };
        let segments = tree.render(&context);

        assert_eq!(segments.len(), 1);
        assert!(segments[0].plain_text().contains("root"));
    }

    #[test]
    fn test_tree_with_children() {
        let mut tree = Tree::new("root");
        tree.add(TreeNode::new("child1"));
        tree.add(TreeNode::new("child2"));

        let context = RenderContext { width: 40 };
        let segments = tree.render(&context);

        assert_eq!(segments.len(), 3);
    }

    #[test]
    fn test_tree_nested() {
        let child1 = TreeNode::new("child1")
            .with_child("grandchild1")
            .with_child("grandchild2");

        let tree = Tree::new(TreeNode::new("root").with_child(child1));

        let context = RenderContext { width: 40 };
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

        let context = RenderContext { width: 40 };
        let segments = tree.render(&context);

        // Should not contain root
        let text: String = segments.iter().map(|s| s.plain_text()).collect();
        assert!(!text.contains("root"));
        assert!(text.contains("child1"));
    }
}
