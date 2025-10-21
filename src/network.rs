use pyo3::prelude::*;

/// `WiFi` network information
#[pyclass]
#[derive(Debug, Clone)]
pub struct WiFiNetwork {
    #[pyo3(get)]
    pub ssid: String,
    #[pyo3(get)]
    pub bssid: String,
    #[pyo3(get)]
    pub channel: u8,
    #[pyo3(get)]
    pub rssi: i8,
    #[pyo3(get)]
    pub encryption: String,
}

/// Network data extracted from beacon frames
#[derive(Debug, Clone)]
pub struct NetworkData {
    pub ssid: String,
    pub bssid: String,
    pub channel: u8,
    pub rssi: i8,
    pub encryption: String,
}

#[pymethods]
impl WiFiNetwork {
    #[new]
    #[must_use]
    pub fn new(ssid: String, bssid: String, channel: u8, rssi: i8, encryption: String) -> Self {
        Self {
            ssid,
            bssid,
            channel,
            rssi,
            encryption,
        }
    }
}
