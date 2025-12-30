mod console;
mod style;
mod table;

mod text;
mod panel;
mod rule;
mod progress;
mod tree;
mod markdown;
mod syntax;
mod columns;
mod traceback;

use pyo3::prelude::*;
use crate::console::PyConsole;
use crate::style::PyStyle;
use crate::table::PyTable;
use crate::progress::PyProgress;
use crate::text::PyText;
use crate::panel::PyPanel;
use crate::rule::PyRule;
use crate::tree::PyTree;
use crate::markdown::PyMarkdown;
use crate::syntax::PySyntax;
use crate::columns::PyColumns;
use crate::traceback::PyTraceback;

/// A Python module implemented in Rust.
#[pymodule]
fn rich_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyConsole>()?;
    m.add_class::<PyStyle>()?;
    m.add_class::<PyTable>()?;
    m.add_class::<PyProgress>()?;
    m.add_class::<PyText>()?;
    m.add_class::<PyPanel>()?;
    m.add_class::<PyRule>()?;
    m.add_class::<PyTree>()?;
    m.add_class::<PyMarkdown>()?;
    m.add_class::<PySyntax>()?;
    m.add_class::<PyColumns>()?;
    m.add_class::<PyTraceback>()?;
    Ok(())
}
