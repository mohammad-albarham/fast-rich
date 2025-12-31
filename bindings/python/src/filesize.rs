use fast_rich::filesize;
use pyo3::prelude::*;

#[pyfunction]
pub fn decimal(size: u64) -> String {
    filesize::decimal(size)
}

#[pyfunction]
pub fn binary(size: u64) -> String {
    filesize::binary(size)
}
