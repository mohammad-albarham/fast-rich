use pyo3::prelude::*;
use rich_rust::console::Console;
use rich_rust::panel::Panel;

#[pyfunction]
#[pyo3(signature = (obj, methods=false, help=false))]
pub fn inspect(_py: Python<'_>, obj: Bound<'_, PyAny>, methods: bool, help: bool) -> PyResult<()> {
    let console = Console::new();
    let type_name = obj.get_type().name()?.to_string();
    let repr = obj.repr()?.to_string();

    let mut report = format!("Type: [green]{}[/]\n\n[bold]Value:[/]\n{}", type_name, repr);

    if help {
        if let Ok(doc) = obj.getattr("__doc__") {
            let doc_str = doc.extract::<String>().unwrap_or_default();
            if !doc_str.is_empty() {
                report.push_str(&format!("\n\n[bold]Docstring:[/]\n[dim]{}[/]", doc_str));
            }
        }
    }

    if methods {
        report.push_str("\n\n[bold]Attributes/Methods:[/]");
        let dir_list = obj.dir()?;
        for name in dir_list {
            let name_str = name.extract::<String>()?;
            if !name_str.starts_with('_') {
                // Simple filter
                report.push_str(&format!("\n- {}", name_str));
            }
        }
    }

    let panel = Panel::new(rich_rust::markup::parse(&report))
        .title(&format!("Inspect: [bold cyan]{}[/]", type_name))
        .style(rich_rust::style::Style::new().foreground(rich_rust::style::Color::Magenta));

    console.print_renderable(&panel);
    Ok(())
}
