//! Track iterator with progress display.

use super::{Progress, ProgressColumn};

/// An iterator wrapper that displays progress.
pub struct TrackedIterator<I>
where
    I: Iterator,
{
    inner: I,
    progress: Progress,
    task_id: usize,
    #[allow(dead_code)]
    description: String,
    #[allow(dead_code)]
    total: Option<u64>,
    current: u64,
}

impl<I> TrackedIterator<I>
where
    I: Iterator,
{
    fn new(inner: I, description: &str, total: Option<u64>) -> Self {
        let progress = Progress::new().columns(vec![
            ProgressColumn::Description,
            ProgressColumn::Bar,
            ProgressColumn::Percentage,
            ProgressColumn::Count,
        ]);

        let task_id = progress.add_task(description, total);

        TrackedIterator {
            inner,
            progress,
            task_id,
            description: description.to_string(),
            total,
            current: 0,
        }
    }
}

impl<I> Iterator for TrackedIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Print progress before getting next item
        self.progress.print();

        match self.inner.next() {
            Some(item) => {
                self.current += 1;
                self.progress.update(self.task_id, self.current);
                Some(item)
            }
            None => {
                // Final update
                self.progress.finish(self.task_id);
                self.progress.print();
                // Move to next line
                println!();
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for TrackedIterator<I> {}

/// Wrap an iterator with progress display.
///
/// # Example
///
/// ```no_run
/// use rich_rust::progress::track;
///
/// for item in track(0..100, "Processing") {
///     // Do work with item
///     std::thread::sleep(std::time::Duration::from_millis(50));
/// }
/// ```
pub fn track<I>(iter: I, description: &str) -> TrackedIterator<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: ExactSizeIterator,
{
    let iter = iter.into_iter();
    let len = iter.len() as u64;
    TrackedIterator::new(iter, description, Some(len))
}

/// Wrap an iterator with progress display (unknown length).
///
/// Use this when the iterator doesn't implement ExactSizeIterator.
#[allow(dead_code)]
pub fn track_unknown<I>(iter: I, description: &str) -> TrackedIterator<I::IntoIter>
where
    I: IntoIterator,
{
    TrackedIterator::new(iter.into_iter(), description, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_size_hint() {
        let items = vec![1, 2, 3, 4, 5];
        let tracked = track(items.clone(), "Test");
        let (lower, upper) = tracked.size_hint();
        assert_eq!(lower, 5);
        assert_eq!(upper, Some(5));
    }

    #[test]
    fn test_tracked_iterator_count() {
        let items = vec![1, 2, 3];
        let tracked = track(items, "Test");
        let count = tracked.count();
        assert_eq!(count, 3);
    }
}
