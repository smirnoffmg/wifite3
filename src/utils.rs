use pyo3::prelude::*;

/// Utility function for creating consistent error messages
pub fn create_runtime_error(message: &str) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(message.to_string())
}
