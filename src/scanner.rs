use pyo3::prelude::*;
use pcap::{Device, Capture};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::network::WiFiNetwork;
use crate::parser::BeaconParser;
use crate::builder::NetworkBuilder;
use crate::utils::create_runtime_error;
use crate::pmkid::{PMKIDCapture, PMKIDParser};

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
            .map_err(|e| create_runtime_error(&format!("Failed to list devices: {e}")))?;
        
        devices.iter()
            .find(|d| d.name == self.interface)
            .ok_or_else(|| create_runtime_error(&format!("Interface '{}' not found", self.interface)))
            .cloned()
    }

    pub fn list_interfaces() -> PyResult<Vec<String>> {
        let devices = Device::list()
            .map_err(|e| create_runtime_error(&format!("Failed to list devices: {e}")))?;
        
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
                .map_err(|e| create_runtime_error(&format!("Failed to list devices: {e}")))?;
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

/// Network scanner for `WiFi` networks
#[pyclass]
pub struct NetworkScanner {
    interface: String,
    ssid_cache: Mutex<HashMap<String, String>>, // BSSID -> SSID mapping
}

#[pymethods]
impl NetworkScanner {
    #[new]
    #[must_use]
    pub fn new(interface: String) -> Self {
        Self { 
            interface,
            ssid_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Test method to verify `PyO3` bindings work
    ///
    /// # Errors
    ///
    /// This function will never return an error.
    pub fn test_method(&self) -> PyResult<String> {
        Ok("test".to_string())
    }

    /// Scan for `WiFi` networks
    ///
    /// # Errors
    ///
    /// Returns an error if the network interface cannot be found or if packet capture fails.
    pub fn scan(&self) -> PyResult<Vec<WiFiNetwork>> {
        let device_manager = DeviceManager::new(self.interface.clone());
        let device = device_manager.find_device()?;
        
        let mut cap = Capture::from_device(device)
            .map_err(|e| create_runtime_error(&format!("Failed to create capture: {e}")))?
            .promisc(true)
            .timeout(1000)
            .open()
            .map_err(|e| create_runtime_error(&format!("Failed to open capture: {e}")))?;
        
        let mut network_builder = NetworkBuilder::new();
        let _ = BeaconParser;
        
        // Capture packets for a short duration
        let mut packet_count = 0;
        let max_packets = 100; // Limit to prevent infinite scanning
        
        while let Ok(packet) = cap.next_packet() {
            packet_count += 1;
            if packet_count > max_packets {
                break;
            }
            
            // Parse WiFi beacon frames using the dedicated parser
            if let Some(network_data) = BeaconParser::parse_beacon_frame(packet.data) {
                network_builder.add_network(network_data);
            }
        }
        
        Ok(network_builder.get_networks())
    }


    /// Get SSID cache for correlation
    ///
    /// # Errors
    ///
    /// Returns an error if the cache lock cannot be acquired.
    pub fn get_ssid_cache(&self) -> PyResult<HashMap<String, String>> {
        let cache = self.ssid_cache.lock()
            .map_err(|e| create_runtime_error(&format!("Failed to lock cache: {e}")))?;
        Ok(cache.clone())
    }

    /// Clear SSID cache
    ///
    /// # Errors
    ///
    /// Returns an error if the cache lock cannot be acquired.
    pub fn clear_ssid_cache(&self) -> PyResult<()> {
        let mut cache = self.ssid_cache.lock()
            .map_err(|e| create_runtime_error(&format!("Failed to lock cache: {e}")))?;
        cache.clear();
        Ok(())
    }

    /// Capture PMKID from EAPOL frames with SSID correlation
    ///
    /// # Errors
    ///
    /// Returns an error if the network interface cannot be found or if packet capture fails.
    pub fn capture_pmkid_with_correlation(&self, duration_seconds: u32) -> PyResult<Vec<PMKIDCapture>> {
        let device_manager = DeviceManager::new(self.interface.clone());
        let device = device_manager.find_device()?;
        
        let mut cap = Capture::from_device(device)
            .map_err(|e| create_runtime_error(&format!("Failed to create capture: {e}")))?
            .promisc(true)
            .timeout(1000)
            .open()
            .map_err(|e| create_runtime_error(&format!("Failed to open capture: {e}")))?;
        
        let _ = PMKIDParser;
        let _ = BeaconParser;
        let mut pmkid_captures = Vec::new();
        
        // Capture packets for specified duration
        let start_time = std::time::Instant::now();
        let duration = std::time::Duration::from_secs(u64::from(duration_seconds));
        
        while start_time.elapsed() < duration {
            if let Ok(packet) = cap.next_packet() {
                // First, try to parse as beacon frame to update SSID cache
                if let Some(network_data) = BeaconParser::parse_beacon_frame(packet.data) {
                    if let Ok(mut cache) = self.ssid_cache.lock() {
                        cache.insert(network_data.bssid.clone(), network_data.ssid.clone());
                    }
                }
                
                // Then, try to parse as EAPOL frame for PMKID
                if let Some(pmkid_data) = PMKIDParser::parse_eapol_frame(packet.data) {
                    // Use cached SSID if available, otherwise use "Unknown"
                    let ssid = if let Ok(cache) = self.ssid_cache.lock() {
                        cache.get(&pmkid_data.bssid)
                            .cloned()
                            .unwrap_or_else(|| "Unknown".to_string())
                    } else {
                        "Unknown".to_string()
                    };
                    
                    let pmkid_capture = PMKIDCapture::new(
                        ssid,
                        pmkid_data.bssid,
                        pmkid_data.client_mac,
                        pmkid_data.pmkid,
                    );
                    pmkid_captures.push(pmkid_capture);
                }
            }
        }
        
        Ok(pmkid_captures)
    }

    /// Capture PMKID from EAPOL frames (legacy method)
    ///
    /// # Errors
    ///
    /// Returns an error if the network interface cannot be found or if packet capture fails.
    pub fn capture_pmkid(&self, duration_seconds: u32) -> PyResult<Vec<PMKIDCapture>> {
        let device_manager = DeviceManager::new(self.interface.clone());
        let device = device_manager.find_device()?;
        
        let mut cap = Capture::from_device(device)
            .map_err(|e| create_runtime_error(&format!("Failed to create capture: {e}")))?
            .promisc(true)
            .timeout(1000)
            .open()
            .map_err(|e| create_runtime_error(&format!("Failed to open capture: {e}")))?;
        
        let _ = PMKIDParser;
        let mut pmkid_captures = Vec::new();
        
        // Capture packets for specified duration
        let start_time = std::time::Instant::now();
        let duration = std::time::Duration::from_secs(u64::from(duration_seconds));
        
        while start_time.elapsed() < duration {
            if let Ok(packet) = cap.next_packet() {
                // Parse EAPOL frames for PMKID
                if let Some(pmkid_data) = PMKIDParser::parse_eapol_frame(packet.data) {
                    let pmkid_capture = PMKIDCapture::new(
                        pmkid_data.ssid,
                        pmkid_data.bssid,
                        pmkid_data.client_mac,
                        pmkid_data.pmkid,
                    );
                    pmkid_captures.push(pmkid_capture);
                }
            }
        }
        
        Ok(pmkid_captures)
    }

    /// Get available network interfaces
    ///
    /// # Errors
    ///
    /// Returns an error if the device list cannot be retrieved.
    #[staticmethod]
    pub fn get_interfaces() -> PyResult<Vec<String>> {
        DeviceManager::list_interfaces()
    }
}
