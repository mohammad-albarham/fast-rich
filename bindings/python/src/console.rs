use pyo3::prelude::*;
use rich_rust::Console;
use rich_rust::text::Text;
use rich_rust::style::Style;
use crate::table::PyTable;
use crate::text::PyText;
use crate::panel::PyPanel;
use crate::rule::PyRule;
use crate::tree::PyTree;

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
}
