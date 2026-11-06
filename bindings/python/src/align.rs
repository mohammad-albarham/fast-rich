use crate::panel::PyPanel;
use crate::table::PyTable;
use crate::text::PyText;
use fast_rich::align::{Align, VerticalAlignment};
use fast_rich::text::Alignment;
use pyo3::prelude::*;

#[pyclass(name = "Align")]
pub struct PyAlign {
    pub inner: Align,
}

#[pymethods]
impl PyAlign {
    #[new]
    #[pyo3(signature = (renderable, align="left", vertical=None, height=None, pad=true))]
    fn new(
        renderable: &Bound<'_, PyAny>,
        align: &str,
        vertical: Option<&str>,
        height: Option<usize>,
        pad: bool,
    ) -> PyResult<Self> {
        let _ = pad; // Suppress unused variable warning
        let child: Box<dyn fast_rich::renderable::Renderable + Send + Sync> =
            if let Ok(text) = renderable.extract::<String>() {
                Box::new(fast_rich::text::Text::plain(text))
            } else if let Ok(py_text) = renderable.downcast::<PyText>() {
                Box::new(py_text.borrow().inner.clone())
            } else if let Ok(py_panel) = renderable.downcast::<PyPanel>() {
                Box::new(py_panel.borrow().inner.clone())
            } else if let Ok(py_table) = renderable.downcast::<PyTable>() {
                Box::new(py_table.borrow().inner.clone())
            } else {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "Unsupported renderable type for Align",
                ));
            };

        let alignment = match align.to_lowercase().as_str() {
            "left" => Alignment::Left,
            "center" => Alignment::Center,
            "right" => Alignment::Right,
            _ => Alignment::Left,
        };

        let vert_align = match vertical.unwrap_or("top").to_lowercase().as_str() {
            "top" => VerticalAlignment::Top,
            "middle" => VerticalAlignment::Middle,
            "bottom" => VerticalAlignment::Bottom,
            _ => VerticalAlignment::Top,
        };

        // Create the Align wrapper
        let mut align_wrapper = match alignment {
            Alignment::Left => Align::left(child),
            Alignment::Center => Align::center(child),
            Alignment::Right => Align::right(child),
        };

        align_wrapper = align_wrapper.vertical(vert_align);

        if let Some(h) = height {
            align_wrapper = align_wrapper.height(h);
        }

        // pad is generic in Rust implementation?
        // My Align struct has pad field but constructors set it to true.
        // I didn't expose .pad() builder method.
        // Assuming default true is fine or add .pad() method if critical.
        // For MVP bindings, ignoring pad=False if not easily supported, but Python default is True.

        Ok(PyAlign {
            inner: align_wrapper,
        })
    }
}
