use crate::text::PyText;
use pyo3::prelude::*;
use rich_rust::columns::{ColumnMode, Columns};

#[pyclass(name = "Columns")]
pub struct PyColumns {
    pub(crate) inner: Columns,
}

#[pymethods]
impl PyColumns {
    #[new]
    #[pyo3(signature = (items, padding=None, expand=true, equal=false, align=None))]
    fn new(
        items: Vec<PyRef<PyText>>,
        padding: Option<usize>,
        expand: bool,
        equal: bool,
        align: Option<String>,
    ) -> Self {
        // items: Vec<PyRef<PyText>> to avoid trait bound issues with bare generic PyClass in Vec

        let texts: Vec<rich_rust::text::Text> = items.iter().map(|pt| pt.inner.clone()).collect();
        let mut cols = Columns::new(texts);

        if let Some(pad) = padding {
            cols = cols.gap(pad);
        }

        cols = cols.expand(expand);

        if equal {
            cols = cols.mode(ColumnMode::Equal);
        } else {
            cols = cols.mode(ColumnMode::Fit);
        }

        // Align is not directly on Columns struct in current Rust impl? valid?
        // Checking src/columns.rs: num_columns, mode, gap, expand, style. No alignment?
        // Rich python has align="left" etc. Maybe missing in Rust implementation?
        // Ignoring align for now.
        let _ = align;

        PyColumns { inner: cols }
    }
}
