use pyo3::prelude::*;
use rich_rust::prompt::Prompt;

#[pyclass(name = "Prompt")]
pub struct PyPrompt;

#[pymethods]
impl PyPrompt {
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None, password=false))]
    fn ask(prompt: &str, default: Option<&str>, password: bool) -> String {
        Prompt::ask(prompt, default, password)
    }
}
