//! Module-level functions the Python `Console` uses to render.

use crate::bridge::{self, style_to_string};
use fast_rich::console::RenderContext;
use fast_rich::renderable::{segments_to_lines, Renderable};
use fast_rich::text::Span;
use pyo3::prelude::*;

/// One rendered span: (text, style, link).
type PySpan = (String, String, Option<String>);

fn to_py(line: Vec<Span>) -> Vec<PySpan> {
    line.into_iter()
        .map(|s| (s.text.to_string(), style_to_string(&s.style), s.link))
        .collect()
}

/// Render a spec at `width` and return its lines of `(text, style, link)`.
///
/// Lines are not padded; callers crop/pad as Rich's `render_lines` does.
#[pyfunction]
pub fn render(
    py: Python<'_>,
    spec: &Bound<'_, PyAny>,
    console: Py<PyAny>,
    width: usize,
) -> PyResult<Vec<Vec<PySpan>>> {
    let _ = bridge::take_error();
    let native = bridge::build(py, spec, &console)?;
    if let Some(err) = bridge::take_error() {
        return Err(err);
    }
    // Like Rich's Console.render: no space, nothing to render.
    if width == 0 {
        return Ok(Vec::new());
    }
    let context = RenderContext {
        width,
        height: None,
    };
    let segments = native.render(&context);
    if let Some(err) = bridge::take_error() {
        return Err(err);
    }
    Ok(segments_to_lines(segments).into_iter().map(to_py).collect())
}

/// `(minimum, maximum)` width of a spec, like Rich's `Measurement`.
#[pyfunction]
pub fn measure(py: Python<'_>, spec: &Bound<'_, PyAny>, console: Py<PyAny>) -> PyResult<(usize, usize)> {
    let _ = bridge::take_error();
    let native = bridge::build(py, spec, &console)?;
    let result = (native.min_width(), native.max_width());
    if let Some(err) = bridge::take_error() {
        return Err(err);
    }
    Ok(result)
}
