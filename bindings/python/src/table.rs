use pyo3::prelude::*;
use rich_rust::Table;

#[pyclass(name = "Table")]
pub struct PyTable {
    pub(crate) inner: Table,
}

#[pymethods]
impl PyTable {
    #[new]
    fn new() -> Self {
        PyTable {
            inner: Table::new(),
        }
    }

    fn add_column(&mut self, header: &str) {
        self.inner.add_column(header);
    }

    fn add_row(&mut self, cells: Vec<String>) {
        let refs: Vec<&str> = cells.iter().map(|s| s.as_str()).collect();
        self.inner.add_row_strs(&refs);
    }

    // TODO: Add render method or hook into Console
}
