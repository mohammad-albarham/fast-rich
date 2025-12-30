use crate::panel::PyPanel;
use crate::rule::PyRule;
use crate::table::PyTable;
use crate::text::PyText;
use crate::tree::PyTree;
use pyo3::prelude::*;
use rich_rust::style::Style;
use rich_rust::text::Text;
use rich_rust::Console;

#[pyclass(name = "Console")]
pub struct PyConsole {
    inner: Console,
}

#[pymethods]
impl PyConsole {
    #[new]
    #[pyo3(signature = (force_terminal = None))]
    fn new(force_terminal: Option<bool>) -> Self {
        let mut console = Console::new();
        if let Some(force) = force_terminal {
            console = console.force_color(force);
        }
        PyConsole {
            inner: console,
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

    /// Print a table.
    fn print_table(&self, table: &PyTable) {
        self.inner.print_renderable(&table.inner);
    }

    /// Print a text object.
    fn print_text(&self, text: &PyText) {
        self.inner.print_renderable(&text.inner);
    }

    /// Print a panel.
    fn print_panel(&self, panel: &PyPanel) {
        self.inner.print_renderable(&panel.inner);
    }

    /// Print a rule.
    fn print_rule(&self, rule: &PyRule) {
        self.inner.print_renderable(&rule.inner);
    }

    /// Print a tree.
    fn print_tree(&self, tree: &PyTree) {
        self.inner.print_renderable(&tree.inner);
    }

    fn print_markdown(&self, markdown: &crate::markdown::PyMarkdown) {
        self.inner.print_renderable(&markdown.inner);
    }

    fn print_syntax(&self, syntax: &crate::syntax::PySyntax) {
        self.inner.print_renderable(&syntax.inner);
    }

    fn print_columns(&self, columns: &crate::columns::PyColumns) {
        self.inner.print_renderable(&columns.inner);
    }

    fn print_traceback(&self, traceback: &crate::traceback::PyTraceback) {
        self.inner.print_renderable(&traceback.inner);
    }

    // Logging methods
    fn log(&self, message: &str) {
        use rich_rust::log::ConsoleLog; // Trait
        self.inner.log(message);
    }

    fn debug(&self, message: &str) {
        use rich_rust::log::ConsoleLog;
        self.inner.debug(message);
    }

    fn warn(&self, message: &str) {
        use rich_rust::log::ConsoleLog;
        self.inner.warn(message);
    }

    fn error(&self, message: &str) {
        use rich_rust::log::ConsoleLog;
        self.inner.error(message);
    }

    /// Print JSON with syntax highlighting.
    fn print_json(&self, json_str: &str) {
        self.inner.print_json(json_str);
    }

    /// Export text representation of a renderable.
    fn export_text(&self, text: &PyText) -> String {
        self.inner.export_text(&text.inner)
    }

    /// Export HTML representation of a renderable.
    fn export_html(&self, text: &PyText) -> String {
        self.inner.export_html(&text.inner)
    }

    /// Export SVG representation of a renderable.
    fn export_svg(&self, text: &PyText) -> String {
        self.inner.export_svg(&text.inner)
    }
}
