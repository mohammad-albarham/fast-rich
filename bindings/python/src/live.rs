use pyo3::prelude::*;
// use fast_rich::live::Live; // Cannot use easily because of lifetime/'a issue in binding

#[pyclass(name = "Live")]
pub struct PyLive {
    // inner: Live<'static>, 
}

#[pymethods]
impl PyLive {
    #[new]
    fn new() -> Self {
        println!("Warning: Python Live binding is currently disabled due to API changes.");
        PyLive {}
    }

    fn start(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn stop(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn refresh(&mut self) -> PyResult<()> {
        Ok(())
    }
}
