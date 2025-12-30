use crate::style::PyStyle;
use pyo3::prelude::*;
use rich_rust::rule::Rule;

#[pyclass(name = "Rule")]
pub struct PyRule {
    pub(crate) inner: Rule,
}

#[pymethods]
impl PyRule {
    #[new]
    #[pyo3(signature = (title=None, style=None))]
    fn new(title: Option<String>, style: Option<PyStyle>) -> Self {
        let mut r = match title {
            Some(t) => Rule::new(&t),
            None => Rule::line(),
        };

        if let Some(s) = style {
            r = r.style(s.inner);
        }
        PyRule { inner: r }
    }
}
