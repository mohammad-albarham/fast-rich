use fast_rich::traceback::{Traceback, TracebackConfig};
use pyo3::prelude::*;

#[pyclass(name = "Traceback")]
pub struct PyTraceback {
    pub(crate) inner: Traceback,
}

#[pymethods]
impl PyTraceback {
    #[new]
    #[pyo3(signature = (message))]
    fn new(message: String) -> Self {
        // Wraps a message string; the Python layer (fast_rich.traceback) renders
        // real Python frames itself.
        let tb = Traceback::from_error(&message).with_config(TracebackConfig::default());

        PyTraceback { inner: tb }
    }
}
