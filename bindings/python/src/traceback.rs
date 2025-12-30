use pyo3::prelude::*;
use rich_rust::traceback::{Traceback, TracebackConfig};

#[pyclass(name = "Traceback")]
pub struct PyTraceback {
    pub(crate) inner: Traceback,
}

#[pymethods]
impl PyTraceback {
    #[new]
    #[pyo3(signature = (message, show_locals=false, _width=None))]
    fn new(message: String, show_locals: bool, _width: Option<usize>) -> Self {
        // Simple wrapper to create a traceback from a message string.
        // In full parity we might want to parse Python tracebacks, but for now this enables the visual component.
        let config = TracebackConfig {
            show_locals,
            ..Default::default()
        };

        let tb = Traceback::from_error(&message).with_config(config);

        PyTraceback { inner: tb }
    }
}
