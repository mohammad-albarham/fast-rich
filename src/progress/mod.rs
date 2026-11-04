//! Progress bars, spinners, and status indicators.
//!
//! This module provides:
//! - Progress bars with percentage, speed, ETA
//! - Spinners for indeterminate progress
//! - Status context for showing work in progress
//! - `track()` helper for iterating with progress

mod bar;
mod columns;
mod spinner;
mod spinner_data;
mod status;
mod track;

pub use bar::{Progress, ProgressBar, Task};

pub use columns::*;
pub use spinner::{Spinner, SpinnerStyle};
pub use status::{with_status, Status};
pub use track::track;

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_reexports_exist() {
        // Compile-time guards that the progress prelude surface stays intact.
        let _: Option<crate::progress::Progress> = None;
        let _: Option<crate::progress::ProgressBar> = None;
        let _: Option<crate::progress::Spinner> = None;
        let _: Option<crate::progress::SpinnerStyle> = None;
        let _: Option<crate::progress::Status> = None;
        let _: Option<crate::progress::Task> = None;
    }
}
