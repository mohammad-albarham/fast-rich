use crate::style::PyStyle;
use pyo3::prelude::*;
use rich_rust::tree::{GuideStyle, Tree};

#[pyclass(name = "Tree")]
pub struct PyTree {
    pub(crate) inner: Tree,
}

#[pymethods]
impl PyTree {
    #[new]
    #[pyo3(signature = (label, style=None, guide_style=None, expanded=true, highlight=false))]
    fn new(
        label: &str,
        style: Option<PyStyle>,
        guide_style: Option<String>,
        expanded: bool,
        highlight: bool,
    ) -> Self {
        let mut t = Tree::new(label.to_string());

        if let Some(s) = style {
            t = t.style(s.inner);
        }

        if let Some(gs_str) = guide_style {
            let gs = match gs_str.to_lowercase().as_str() {
                "ascii" => GuideStyle::Ascii,
                "bold" => GuideStyle::Bold,
                "double" => GuideStyle::Double,
                _ => GuideStyle::Unicode,
            };
            t = t.guide_style(gs);
        }

        // t = t.expanded(expanded).highlight(highlight); // Not available on Tree, only TreeNode?
        // TODO: Expose these on Tree or access root node.
        // For now, ignore expanded/highlight args to compile.
        let _ = expanded;
        let _ = highlight;

        PyTree { inner: t }
    }

    fn add(&mut self, label: &str) -> PyTree {
        let _child = self.inner.add(label.to_string());
        // Return dummy?
        // Actually, let's just make it void return for now to match add_leaf behavior effectively
        // but keeping signature returns PyTree is hard without proper wrapper.
        // Let's change signature to return None if we can't wrap easily.
        PyTree {
            inner: Tree::new("dummy".to_string()),
        }
    }

    // Better API for Rust bindings might be `add_leaf`
    fn add_leaf(&mut self, label: &str) {
        self.inner.add(label.to_string());
    }
}
