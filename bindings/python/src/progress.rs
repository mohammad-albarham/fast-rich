use pyo3::prelude::*;
use rich_rust::progress::Progress;

#[pyclass(name = "Progress")]
pub struct PyProgress {
    inner: Progress,
}

#[pymethods]
impl PyProgress {
    #[new]
    fn new() -> Self {
        PyProgress {
            inner: Progress::new(),
        }
    }

    /// Add a new task to the progress bar.
    #[pyo3(signature = (description, total = None))]
    fn add_task(&mut self, description: &str, total: Option<u64>) -> usize {
        self.inner.add_task(description, total)
    }

    /// Update a task's progress.
    fn update(&mut self, task_id: usize, completed: u64) {
        self.inner.update(task_id, completed);
    }

    /// Advance a task's progress.
    fn advance(&mut self, task_id: usize, amount: u64) {
        self.inner.advance(task_id, amount);
    }

    /// Print the progress bar (manual).
    fn print(&self) {
        self.inner.print();
    }
}
