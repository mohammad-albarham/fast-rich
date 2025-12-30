mod console;
mod style;
mod table;

mod progress;

use pyo3::prelude::*;
use crate::console::PyConsole;
use crate::style::PyStyle;
use crate::table::PyTable;
use crate::progress::PyProgress;

/// A Python module implemented in Rust.
#[pymodule]
fn rich_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyConsole>()?;
    m.add_class::<PyStyle>()?;
    m.add_class::<PyTable>()?;
    m.add_class::<PyProgress>()?;
    Ok(())
}
