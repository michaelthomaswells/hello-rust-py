use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// A simple hello world function.
#[pyfunction]
fn say_hello(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}! Greetings from Rust on macOS 🍎🚀", name))
}

/// This module is a python extension implemented in Rust.
#[pymodule]
fn hello_rust_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
