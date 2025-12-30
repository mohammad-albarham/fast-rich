use pyo3::prelude::*;
use rich_rust::style::Color;
use rich_rust::style::Style;

#[pyclass(name = "Style")]
#[derive(Clone)]
pub struct PyStyle {
    pub inner: Style,
}

#[pymethods]
impl PyStyle {
    #[new]
    #[pyo3(signature = (
        color = None,
        bgcolor = None,
        bold = false,
        dim = false,
        italic = false,
        underline = false,
        blink = false,
        reverse = false,
        hidden = false,
        strikethrough = false
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        color: Option<&str>,
        bgcolor: Option<&str>,
        bold: bool,
        dim: bool,
        italic: bool,
        underline: bool,
        blink: bool,
        reverse: bool,
        hidden: bool,
        strikethrough: bool,
    ) -> Self {
        let mut style = Style::new();

        if let Some(c) = color {
            if let Some(parsed) = Color::parse(c) {
                style = style.foreground(parsed);
            }
        }
        if let Some(c) = bgcolor {
            if let Some(parsed) = Color::parse(c) {
                style = style.background(parsed);
            }
        }

        if bold {
            style = style.bold();
        }
        if dim {
            style = style.dim();
        }
        if italic {
            style = style.italic();
        }
        if underline {
            style = style.underline();
        }
        if blink {
            style = style.blink();
        }
        if reverse {
            style = style.reverse();
        }
        if hidden {
            style = style.hidden();
        }
        if strikethrough {
            style = style.strikethrough();
        }

        PyStyle { inner: style }
    }

    #[staticmethod]
    fn parse(s: &str) -> Self {
        PyStyle {
            inner: Style::parse(s),
        }
    }
}
