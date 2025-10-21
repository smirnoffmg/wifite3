use pyo3::prelude::*;

// Module declarations
mod network;
mod scanner;
mod parser;
mod builder;
mod utils;
mod pmkid;

// Re-exports for Python bindings
pub use network::WiFiNetwork;
pub use scanner::NetworkScanner;
pub use pmkid::PMKIDCapture;

/// A Python module implemented in Rust.
#[pymodule]
mod _wifite3 {
    #[pymodule_export]
    use super::WiFiNetwork;

    #[pymodule_export]
    use super::NetworkScanner;

    #[pymodule_export]
    use super::PMKIDCapture;
}
