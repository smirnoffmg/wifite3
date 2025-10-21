use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// PMKID capture data structure
#[pyclass]
#[derive(Debug, Clone, Serialize)]
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Deserialize)]
pub struct PMKIDCapture {
    #[pyo3(get)]
    pub ssid: String,
    #[pyo3(get)]
    pub bssid: String,
    #[pyo3(get)]
    pub client_mac: String,
    #[pyo3(get)]
    pub pmkid: String,
    #[pyo3(get)]
    pub hashcat_format: String,
}

/// PMKID capture result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PMKIDData {
    pub ssid: String,
    pub bssid: String,
    pub client_mac: String,
    pub pmkid: String,
    pub hashcat_format: String,
}

#[pymethods]
impl PMKIDCapture {
    #[new]
    #[must_use]
    pub fn new(ssid: String, bssid: String, client_mac: String, pmkid: String) -> Self {
        let hashcat_format = format!(
            "WPA*01*{}*{}*{}*{}",
            pmkid, bssid.replace(':', ""), client_mac.replace(':', ""), ssid
        );
        
        Self {
            ssid,
            bssid,
            client_mac,
            pmkid,
            hashcat_format,
        }
    }


    /// Get a summary of the PMKID capture
    #[must_use]
    pub fn get_summary(&self) -> String {
        format!(
            "PMKID: {} -> {} (Client: {})",
            self.ssid, self.bssid, self.client_mac
        )
    }
}

/// PMKID parser for extracting PMKID from EAPOL frames
pub struct PMKIDParser;

impl PMKIDParser {
    /// Parse EAPOL frame for PMKID
    pub fn parse_eapol_frame(data: &[u8]) -> Option<PMKIDData> {
        if data.len() < 24 {
            return None;
        }

        // Check if this is an EAPOL frame
        if !Self::is_eapol_frame(data) {
            return None;
        }

        // Extract MAC addresses
        let bssid = Self::extract_mac_address(data, 4)?; // Source address
        let client_mac = Self::extract_mac_address(data, 10)?; // Destination address

        // Parse EAPOL payload for PMKID
        let pmkid_data = Self::extract_pmkid_from_eapol(data)?;
        
        // Extract SSID from beacon frames (we'll need to correlate this)
        let ssid = "Unknown".to_string(); // TODO: Correlate with beacon frames

        Some(PMKIDData {
            ssid: ssid.clone(),
            bssid: bssid.clone(),
            client_mac: client_mac.clone(),
            pmkid: pmkid_data.clone(),
            hashcat_format: format!(
                "WPA*01*{}*{}*{}*{}",
                pmkid_data, 
                bssid.replace(':', ""), 
                client_mac.replace(':', ""), 
                ssid
            ),
        })
    }

    /// Check if packet is an EAPOL frame
    fn is_eapol_frame(data: &[u8]) -> bool {
        if data.len() < 24 {
            return false;
        }

        // Check for EAPOL frame type (0x888E)
        let ether_type = u16::from_be_bytes([data[12], data[13]]);
        ether_type == 0x888E
    }

    /// Extract MAC address from packet
    fn extract_mac_address(data: &[u8], offset: usize) -> Option<String> {
        if data.len() < offset + 6 {
            return None;
        }

        let mac_bytes = &data[offset..offset + 6];
        Some(format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac_bytes[0], mac_bytes[1], mac_bytes[2],
            mac_bytes[3], mac_bytes[4], mac_bytes[5]
        ))
    }

    /// Extract PMKID from EAPOL frame
    fn extract_pmkid_from_eapol(data: &[u8]) -> Option<String> {
        // EAPOL frame structure:
        // - Ethernet header (14 bytes)
        // - EAPOL header (4 bytes)
        // - EAPOL payload
        
        if data.len() < 18 {
            return None;
        }

        let eapol_header_start = 14;
        let _ = data[eapol_header_start];
        let packet_type = data[eapol_header_start + 1];
        let body_length = u16::from_be_bytes([
            data[eapol_header_start + 2],
            data[eapol_header_start + 3]
        ]);

        // Check if this is an EAPOL-Key frame
        if packet_type != 3 {
            return None;
        }

        // Parse EAPOL-Key frame for PMKID
        let key_data_start = eapol_header_start + 4;
        if data.len() < key_data_start + body_length as usize {
            return None;
        }

        // Look for RSN IE in the key data
        Self::extract_pmkid_from_rsn_ie(&data[key_data_start..])
    }

    /// Extract PMKID from RSN Information Element
    fn extract_pmkid_from_rsn_ie(key_data: &[u8]) -> Option<String> {
        let mut offset = 0;
        
        // Skip EAPOL-Key header (16 bytes)
        if key_data.len() < 16 {
            return None;
        }
        offset += 16;

        // Look for RSN IE (ID 48)
        while offset + 2 < key_data.len() {
            let element_id = key_data[offset];
            let element_len = key_data[offset + 1] as usize;
            
            if offset + 2 + element_len > key_data.len() {
                break;
            }

            if element_id == 48 { // RSN IE
                return Self::parse_rsn_ie_for_pmkid(&key_data[offset + 2..offset + 2 + element_len]);
            }
            
            offset += 2 + element_len;
        }

        None
    }

    /// Parse RSN IE for PMKID
    fn parse_rsn_ie_for_pmkid(rsn_data: &[u8]) -> Option<String> {
        if rsn_data.len() < 8 {
            return None;
        }

        // RSN IE structure:
        // - Version (2 bytes)
        // - Group Cipher Suite (4 bytes)
        // - Pairwise Cipher Suite Count (2 bytes)
        // - Pairwise Cipher Suites (variable)
        // - AKM Suite Count (2 bytes)
        // - AKM Suites (variable)
        // - RSN Capabilities (2 bytes)
        // - PMKID Count (2 bytes) - if present
        // - PMKID List (variable)

        let mut offset = 8; // Skip version, group cipher, pairwise count, pairwise suites
        
        // Skip AKM suite count and AKM suites
        if offset + 2 > rsn_data.len() {
            return None;
        }
        let akm_count = u16::from_le_bytes([rsn_data[offset], rsn_data[offset + 1]]);
        offset += 2 + (akm_count * 4) as usize;

        // Skip RSN capabilities
        if offset + 2 > rsn_data.len() {
            return None;
        }
        offset += 2;

        // Check for PMKID count
        if offset + 2 > rsn_data.len() {
            return None;
        }
        let pmkid_count = u16::from_le_bytes([rsn_data[offset], rsn_data[offset + 1]]);
        offset += 2;

        if pmkid_count > 0 && offset + 16 <= rsn_data.len() {
            // Extract first PMKID (16 bytes)
            let pmkid_bytes = &rsn_data[offset..offset + 16];
            return Some(format!(
                "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                pmkid_bytes[0], pmkid_bytes[1], pmkid_bytes[2], pmkid_bytes[3],
                pmkid_bytes[4], pmkid_bytes[5], pmkid_bytes[6], pmkid_bytes[7],
                pmkid_bytes[8], pmkid_bytes[9], pmkid_bytes[10], pmkid_bytes[11],
                pmkid_bytes[12], pmkid_bytes[13], pmkid_bytes[14], pmkid_bytes[15]
            ));
        }

        None
    }
}
