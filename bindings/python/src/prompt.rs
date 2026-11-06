use fast_rich::prompt::{Prompt, PromptError};
use pyo3::exceptions::{PyEOFError, PyKeyboardInterrupt, PyOSError};
use pyo3::prelude::*;

/// Map a crate prompt error to the matching Python exception.
fn to_py_err(err: PromptError) -> PyErr {
    match err {
        PromptError::Eof => PyEOFError::new_err(err.to_string()),
        PromptError::Interrupted => PyKeyboardInterrupt::new_err(err.to_string()),
        PromptError::Io(_) => PyOSError::new_err(err.to_string()),
    }
}

#[pyclass(name = "Prompt")]
pub struct PyPrompt;

#[pymethods]
impl PyPrompt {
    /// Ask for a string. Raises EOFError when stdin is closed.
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None, password=false))]
    fn ask(py: Python<'_>, prompt: &str, default: Option<&str>, password: bool) -> PyResult<String> {
        let mut p = Prompt::<String>::new(prompt);
        if let Some(def) = default {
            p = p.default(def.to_string());
        }
        if password {
            p = p.secret();
        }
        py.allow_threads(|| p.try_ask()).map_err(to_py_err)
    }
}

#[pyclass(name = "Confirm")]
pub struct PyConfirm;

#[pymethods]
impl PyConfirm {
    /// Ask a yes/no question. On EOF the default (or False) is returned.
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None))]
    fn ask(py: Python<'_>, prompt: &str, default: Option<bool>) -> bool {
        py.allow_threads(|| fast_rich::prompt::Confirm::ask(prompt, default))
    }
}

#[pyclass(name = "IntPrompt")]
pub struct PyIntPrompt;

#[pymethods]
impl PyIntPrompt {
    /// Ask for an integer. Raises EOFError when stdin is closed.
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None))]
    fn ask(py: Python<'_>, prompt: &str, default: Option<i64>) -> PyResult<i64> {
        let mut p = fast_rich::prompt::IntPrompt::new(prompt);
        if let Some(def) = default {
            p = p.default(def);
        }
        py.allow_threads(|| p.try_ask()).map_err(to_py_err)
    }
}
