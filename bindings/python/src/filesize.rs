use pyo3::prelude::*;
use rich_rust::filesize;

#[pyfunction]
pub fn decimal(size: u64) -> String {
    filesize::decimal(size)
}

#[pyfunction]
pub fn binary(size: u64) -> String {
    filesize::binary(size)
}
