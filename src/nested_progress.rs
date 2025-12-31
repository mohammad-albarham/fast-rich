//! Nested progress bars support.
//!
//! Simplified version that demonstrates the concept.

/// A simple nested progress structure.
pub struct NestedProgress {
    description: String,
    total: u64,
    current: u64,
    children: Vec<NestedProgress>,
}

impl NestedProgress {
    /// Create a new nested progress.
    pub fn new(description: impl Into<String>, total: u64) -> Self {
        NestedProgress {
            description: description.into(),
            total,
            current: 0,
            children: Vec::new(),
        }
    }

    /// Add a child progress.
    pub fn add_child(&mut self, description: impl Into<String>, total: u64) -> &mut NestedProgress {
        self.children.push(NestedProgress::new(description, total));
        self.children.last_mut().unwrap()
    }

    /// Update progress.
    pub fn update(&mut self, amount: u64) {
        self.current = (self.current + amount).min(self.total);
    }

    /// Get completion percentage.
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }

    /// Get the number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_progress_creation() {
        let nested = NestedProgress::new("Root task", 100);
        assert_eq!(nested.child_count(), 0);
    }

    #[test]
    fn test_add_child() {
        let mut nested = NestedProgress::new("Parent", 100);
        nested.add_child("Child task", 50);
        assert_eq!(nested.child_count(), 1);
    }

    #[test]
    fn test_update() {
        let mut nested = NestedProgress::new("Task", 100);
        nested.update(50);
        assert_eq!(nested.percent(), 50.0);
    }
}

