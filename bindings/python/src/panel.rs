use crate::style::PyStyle;
use crate::text::PyText;
use pyo3::prelude::*;
use rich_rust::panel::Panel;

#[pyclass(name = "Panel")]
pub struct PyPanel {
    pub(crate) inner: Panel,
}

#[pymethods]
impl PyPanel {
    #[new]
    #[pyo3(signature = (content, title=None, border_style=None))]
    #[allow(unused_variables)]
    fn new(content: &PyText, title: Option<String>, border_style: Option<PyStyle>) -> Self {
        // Clone content text - Panel takes ownership
        let mut p = Panel::new(content.inner.clone());
        if let Some(t) = title {
            p = p.title(&t);
        }
        // TODO: Map BorderStyle enum from string or object
        // if let Some(bs) = border_style { ... }

        PyPanel { inner: p }
    }
}
