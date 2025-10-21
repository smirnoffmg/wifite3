use pyo3::prelude::*;

// Module declarations
mod network;
mod scanner;
mod parser;
mod builder;
mod utils;

// Re-exports for Python bindings
pub use network::WiFiNetwork;
pub use scanner::NetworkScanner;

/// A Python module implemented in Rust.
#[pymodule]
fn _wifite3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WiFiNetwork>()?;
    m.add_class::<NetworkScanner>()?;
    Ok(())
}
