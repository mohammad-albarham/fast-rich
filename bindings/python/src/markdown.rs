use pyo3::prelude::*;
use rich_rust::markdown::Markdown;

#[pyclass(name = "Markdown")]
pub struct PyMarkdown {
    pub(crate) inner: Markdown,
}

#[pymethods]
impl PyMarkdown {
    #[new]
    #[pyo3(signature = (markup, code_theme=None, justify=None))]
    fn new(markup: &str, code_theme: Option<String>, justify: Option<bool>) -> Self {
        // Rust Markdown struct primarily takes string content.
        // TODO: Pass theme and justify to Rust Markdown if supported.
        let _ = code_theme;
        let _ = justify;
        PyMarkdown {
            inner: Markdown::new(markup),
        }
    }
}
