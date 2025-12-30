use pyo3::prelude::*;
use rich_rust::Console;
use rich_rust::text::Text;
use rich_rust::style::Style;

#[pyclass(name = "Console")]
pub struct PyConsole {
    inner: Console,
}

#[pymethods]
impl PyConsole {
    #[new]
    fn new() -> Self {
        PyConsole {
            inner: Console::new(),
        }
    }

    /// Print text with optional markup style.
    #[pyo3(signature = (text, style = None))]
    fn print(&self, text: &str, style: Option<&str>) {
        if let Some(style_str) = style {
            let style = Style::parse(style_str);
            // Create owned string to ensure lifetime safety within function
            let content = text.to_string();
            let mut t = Text::from(content); 
            t.spans[0].style = style;
            self.inner.print_renderable(&t);
        } else {
            self.inner.print(text);
        }
    }
}
