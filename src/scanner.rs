use pyo3::prelude::*;
use pcap::{Device, Capture};
use crate::network::{WiFiNetwork, NetworkData};
use crate::parser::BeaconParser;
use crate::builder::NetworkBuilder;
use crate::utils::create_runtime_error;

/// Device manager for handling network interface operations
pub struct DeviceManager {
    interface: String,
}

impl DeviceManager {
    pub fn new(interface: String) -> Self {
        Self { interface }
    }

    pub fn find_device(&self) -> PyResult<Device> {
        let devices = Device::list()
            .map_err(|e| create_runtime_error(&format!("Failed to list devices: {}", e)))?;
        
        devices.iter()
            .find(|d| d.name == self.interface)
            .ok_or_else(|| create_runtime_error(&format!("Interface '{}' not found", self.interface)))
            .map(|d| d.clone())
    }

    pub fn list_interfaces() -> PyResult<Vec<String>> {
        let devices = Device::list()
            .map_err(|e| create_runtime_error(&format!("Failed to list devices: {}", e)))?;
        
        let mut interfaces = Vec::new();
        
        for device in devices {
            let name = device.name;
            if Self::is_wireless_interface(&name) {
                interfaces.push(name);
            }
        }
        
        // If no wireless interfaces found, return all available interfaces
        if interfaces.is_empty() {
            let devices = Device::list()
                .map_err(|e| create_runtime_error(&format!("Failed to list devices: {}", e)))?;
            for device in devices {
                interfaces.push(device.name);
            }
        }
        
        Ok(interfaces)
    }

    fn is_wireless_interface(name: &str) -> bool {
        name.starts_with("wlan") || 
        name.starts_with("wlp") || 
        name.starts_with("en") ||
        name.starts_with("eth")
    }
}

/// Network scanner for WiFi networks
#[pyclass]
pub struct NetworkScanner {
    interface: String,
}

#[pymethods]
impl NetworkScanner {
    #[new]
    pub fn new(interface: String) -> Self {
        Self { interface }
    }

    /// Scan for WiFi networks
    pub fn scan(&self) -> PyResult<Vec<WiFiNetwork>> {
        let device_manager = DeviceManager::new(self.interface.clone());
        let device = device_manager.find_device()?;
        
        let mut cap = Capture::from_device(device)
            .map_err(|e| create_runtime_error(&format!("Failed to create capture: {}", e)))?
            .promisc(true)
            .timeout(1000)
            .open()
            .map_err(|e| create_runtime_error(&format!("Failed to open capture: {}", e)))?;
        
        let mut network_builder = NetworkBuilder::new();
        let beacon_parser = BeaconParser;
        
        // Capture packets for a short duration
        let mut packet_count = 0;
        let max_packets = 100; // Limit to prevent infinite scanning
        
        while let Ok(packet) = cap.next_packet() {
            packet_count += 1;
            if packet_count > max_packets {
                break;
            }
            
            // Parse WiFi beacon frames using the dedicated parser
            if let Some(network_data) = beacon_parser.parse_beacon_frame(&packet.data) {
                network_builder.add_network(network_data);
            }
        }
        
        Ok(network_builder.get_networks())
    }

    /// Get available network interfaces
    #[staticmethod]
    pub fn get_interfaces() -> PyResult<Vec<String>> {
        DeviceManager::list_interfaces()
    }
}
