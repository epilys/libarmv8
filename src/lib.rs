// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later

use pyo3::prelude::*;

mod shared;
mod shared_memory;
mod shared_mpam;
mod shared_translation;
mod shared_vmsa;

mod translation64;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn libarmv8(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
