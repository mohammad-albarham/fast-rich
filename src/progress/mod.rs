//! Progress bars, spinners, and status indicators.
//!
//! This module provides:
//! - Progress bars with percentage, speed, ETA
//! - Spinners for indeterminate progress
//! - Status context for showing work in progress
//! - `track()` helper for iterating with progress

mod bar;
mod spinner;
mod status;
mod track;

pub use bar::{Progress, ProgressBar, ProgressColumn, Task};
pub use spinner::{Spinner, SpinnerStyle};
pub use status::{with_status, Status};
pub use track::track;
