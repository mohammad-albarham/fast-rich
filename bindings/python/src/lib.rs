mod console;
mod style;
mod table;

mod columns;
mod filesize;
mod inspect;
mod layout;
mod live;
mod markdown;
mod panel;
mod progress;
mod prompt;
mod rule;
mod syntax;
mod text;
mod traceback;
mod tree;

use crate::columns::PyColumns;
use crate::console::PyConsole;
use crate::markdown::PyMarkdown;
use crate::panel::PyPanel;
use crate::progress::PyProgress;
use crate::rule::PyRule;
use crate::style::PyStyle;
use crate::syntax::PySyntax;
use crate::table::PyTable;
use crate::text::PyText;
use crate::traceback::PyTraceback;
use crate::tree::PyTree;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
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
    m.add_class::<crate::prompt::PyPrompt>()?;
    m.add_class::<crate::layout::PyLayout>()?;
    m.add_class::<crate::live::PyLive>()?;

    // Functions
    m.add_function(wrap_pyfunction!(filesize::decimal, m)?)?;
    m.add_function(wrap_pyfunction!(filesize::binary, m)?)?;
    m.add_function(wrap_pyfunction!(inspect::inspect, m)?)?;
    Ok(())
}
