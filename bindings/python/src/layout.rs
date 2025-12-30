use pyo3::prelude::*;
use rich_rust::layout::Layout;

#[pyclass(name = "Layout")]
pub struct PyLayout {
    #[allow(dead_code)]
    inner: Layout,
}

#[pymethods]
impl PyLayout {
    #[new]
    fn new() -> Self {
        PyLayout {
            inner: Layout::new(),
        }
    }

    fn split_row(&mut self, _others: Vec<PyRef<PyLayout>>) {
        // TODO: Implement conversion from PyLayout to layout trees
    }

    fn split_column(&mut self, _others: Vec<PyRef<PyLayout>>) {
        // TODO
    }
}
