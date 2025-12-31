use pyo3::prelude::*;
use rich_rust::live::Live;
// Note: Passing Python objects as "Renderable" to Rust Live is complex because
// we'd need to wrap the PyObject in a struct implementing Renderable.
// For now, allow simple updates or text.

#[pyclass(name = "Live")]
pub struct PyLive {
    inner: Live,
}

#[pymethods]
impl PyLive {
    #[new]
    fn new() -> Self {
        PyLive { inner: Live::new() }
    }

    fn start(&mut self) -> PyResult<()> {
        self.inner
            .start()
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    fn stop(&mut self) -> PyResult<()> {
        self.inner
            .stop()
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    fn refresh(&mut self) -> PyResult<()> {
        self.inner
            .refresh()
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
}
