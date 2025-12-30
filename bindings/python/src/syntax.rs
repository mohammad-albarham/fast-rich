use pyo3::prelude::*;
use rich_rust::syntax::{Syntax, Theme};

#[pyclass(name = "Syntax")]
pub struct PySyntax {
    pub(crate) inner: Syntax,
}

#[pymethods]
impl PySyntax {
    #[new]
    #[pyo3(signature = (code, lexer=None, theme=None, line_numbers=false))]
    fn new(code: &str, lexer: Option<String>, theme: Option<String>, line_numbers: bool) -> Self {
        let mut s = if let Some(l) = lexer {
            Syntax::new(code, &l)
        } else {
            Syntax::new(code, "python") // Default to python for now if generic
        };

        if let Some(t_str) = theme {
            let t = match t_str.to_lowercase().as_str() {
                "monokai" => Theme::Monokai,
                "github" | "inspired_github" => Theme::InspiredGitHub,
                "solarized_dark" => Theme::SolarizedDark,
                "solarized_light" => Theme::SolarizedLight,
                "ocean_dark" | "base16_ocean_dark" => Theme::Base16OceanDark,
                _ => Theme::Monokai,
            };
            s = s.theme(t);
        }

        if line_numbers {
            s = s.line_numbers(true);
        }

        PySyntax { inner: s }
    }
}
