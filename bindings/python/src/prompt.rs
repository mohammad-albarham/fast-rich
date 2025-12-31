use pyo3::prelude::*;
use fast_rich::prompt::Prompt;

#[pyclass(name = "Prompt")]
pub struct PyPrompt;

#[pymethods]
impl PyPrompt {
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None, password=false))]
    fn ask(prompt: &str, default: Option<&str>, password: bool) -> String {
        let mut p = Prompt::<String>::new(prompt);
        if let Some(def) = default {
            p = p.default(def.to_string());
        }
        if password {
            p = p.secret();
        }
        p.ask()
    }
}

#[pyclass(name = "Confirm")]
pub struct PyConfirm;

#[pymethods]
impl PyConfirm {
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None))]
    fn ask(prompt: &str, default: Option<bool>) -> bool {
        fast_rich::prompt::Confirm::ask(prompt, default)
    }
}

#[pyclass(name = "IntPrompt")]
pub struct PyIntPrompt;

#[pymethods]
impl PyIntPrompt {
    #[staticmethod]
    #[pyo3(signature = (prompt, default=None))]
    fn ask(prompt: &str, default: Option<i64>) -> i64 {
        let mut p = fast_rich::prompt::IntPrompt::new(prompt);
        if let Some(def) = default {
            p = p.default(def);
        }
        p.ask()
    }
}
