use crate::style::PyStyle;
use fast_rich::style::Style;
use fast_rich::text::{Span, Text};
use pyo3::prelude::*;

#[pyclass(name = "Span")]
#[derive(Clone)]
pub struct PySpan {
    pub inner: Span,
}

#[pymethods]
impl PySpan {
    #[new]
    #[pyo3(signature = (text, style=None))]
    fn new(text: String, style: Option<PyStyle>) -> Self {
        let s = if let Some(ps) = style {
            ps.inner
        } else {
            Style::new()
        };
        PySpan {
            inner: Span::styled(text, s),
        }
    }
}

#[pyclass(name = "Text")]
pub struct PyText {
    pub(crate) inner: Text,
}

#[pymethods]
impl PyText {
    #[new]
    #[pyo3(signature = (text="", style=None))]
    fn new(text: &str, style: Option<PyStyle>) -> Self {
        let mut t = Text::from(text.to_string());
        if let Some(ps) = style {
            t.spans[0].style = ps.inner;
        }
        PyText { inner: t }
    }

    #[staticmethod]
    fn from_markup(markup: &str) -> Self {
        PyText {
            inner: fast_rich::markup::parse(markup),
        }
    }

    #[pyo3(signature = (text, style=None))]
    fn append(&mut self, text: &str, style: Option<PyStyle>) {
        let s = if let Some(ps) = style {
            ps.inner
        } else {
            Style::new()
        };
        self.inner.push_styled(text.to_string(), s);
    }

    fn set_style(&mut self, _start: usize, _end: usize, _style: PyStyle) {
        // Simple approximation: apply style to spans that overlap?
        // fast-rich native Text doesn't support range-based styling post-construction easily yet without split_at
        // For now, this might be a no-op or limited implementation.
        // TODO: Implement proper span splitting for arbitrary range styling in core lib first?
    }
}
