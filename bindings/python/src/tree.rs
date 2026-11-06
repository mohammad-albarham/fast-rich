use crate::style::PyStyle;
use fast_rich::style::Style;
use fast_rich::tree::{GuideStyle, Tree, TreeNode};
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

/// A node shared between the Python `Tree` handles that refer to it.
#[derive(Default)]
struct Node {
    label: String,
    style: Style,
    expanded: bool,
    children: Vec<Arc<Mutex<Node>>>,
}

fn lock(node: &Arc<Mutex<Node>>) -> std::sync::MutexGuard<'_, Node> {
    node.lock().unwrap_or_else(|e| e.into_inner())
}

fn build(node: &Arc<Mutex<Node>>) -> TreeNode {
    let n = lock(node);
    let mut out = TreeNode::new(n.label.clone())
        .style(n.style)
        .expanded(n.expanded);
    for child in &n.children {
        out.add(build(child));
    }
    out
}

/// A tree node. `add` returns the child, so children can be nested:
/// `t.add("child").add("leaf")`.
#[pyclass(name = "Tree")]
pub struct PyTree {
    node: Arc<Mutex<Node>>,
    guide_style: GuideStyle,
}

impl PyTree {
    /// Build the crate tree for rendering.
    pub(crate) fn to_tree(&self) -> Tree {
        Tree::new(build(&self.node)).guide_style(self.guide_style)
    }
}

#[pymethods]
impl PyTree {
    #[new]
    #[pyo3(signature = (label, style=None, guide_style=None, expanded=true))]
    fn new(label: &str, style: Option<PyStyle>, guide_style: Option<String>, expanded: bool) -> Self {
        let guide = match guide_style.as_deref().map(str::to_lowercase).as_deref() {
            Some("ascii") => GuideStyle::Ascii,
            Some("bold") => GuideStyle::Bold,
            Some("double") => GuideStyle::Double,
            _ => GuideStyle::Unicode,
        };
        PyTree {
            node: Arc::new(Mutex::new(Node {
                label: label.to_string(),
                style: style.map(|s| s.inner).unwrap_or_default(),
                expanded,
                children: Vec::new(),
            })),
            guide_style: guide,
        }
    }

    /// Add a child node and return it.
    #[pyo3(signature = (label, style=None, expanded=true))]
    fn add(&self, label: &str, style: Option<PyStyle>, expanded: bool) -> PyTree {
        let child = Arc::new(Mutex::new(Node {
            label: label.to_string(),
            style: style.map(|s| s.inner).unwrap_or_default(),
            expanded,
            children: Vec::new(),
        }));
        lock(&self.node).children.push(child.clone());
        PyTree {
            node: child,
            guide_style: self.guide_style,
        }
    }

    /// Add a leaf (a child whose handle is not needed).
    fn add_leaf(&self, label: &str) {
        let _ = self.add(label, None, true);
    }
}
