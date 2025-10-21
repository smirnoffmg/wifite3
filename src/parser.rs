use crate::network::NetworkData;

/// Beacon frame parser for extracting network information
pub struct BeaconParser;

impl BeaconParser {
    pub fn parse_beacon_frame(data: &[u8]) -> Option<NetworkData> {
        if data.len() < 24 {
            return None;
        }

        // Check if this looks like a WiFi frame
        let frame_control = u16::from_le_bytes([data[0], data[1]]);
        let frame_type = (frame_control >> 2) & 0x03;
        let frame_subtype = (frame_control >> 4) & 0x0F;
        
        // Management frame (type 0) with beacon subtype (8)
        if frame_type != 0 || frame_subtype != 8 {
            return None;
        }

        // Extract BSSID (source address)
        if data.len() < 16 {
            return None;
        }

        let bssid = format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            data[10], data[11], data[12], data[13], data[14], data[15]);

        // Parse beacon frame for network data
        let network_data = Self::extract_network_data(data);
        
        Some(NetworkData {
            ssid: network_data.0,
            bssid,
            channel: network_data.1,
            rssi: network_data.2,
            encryption: network_data.3,
        })
    }

    fn extract_network_data(data: &[u8]) -> (String, u8, i8, String) {
        let mut ssid = "Hidden Network".to_string();
        let mut encryption = "Open".to_string();
        let mut channel = 6u8;
        
        if data.len() < 36 {
            return (ssid, channel, -50, encryption);
        }

        let mut offset = 36; // Start after fixed header
        
        // Parse information elements
        while offset + 2 < data.len() {
            let element_id = data[offset];
            let element_len = data[offset + 1] as usize;
            
            if offset + 2 + element_len > data.len() {
                break;
            }
            
            match element_id {
                0 => { // SSID element
                    if element_len > 0 {
                        let ssid_bytes = &data[offset + 2..offset + 2 + element_len];
                        if let Ok(ssid_str) = String::from_utf8(ssid_bytes.to_vec()) {
                            if !ssid_str.is_empty() {
                                ssid = ssid_str;
                            }
                        }
                    }
                },
                3 => { // DS Parameter Set (Channel)
                    if element_len >= 1 {
                        channel = data[offset + 2];
                    }
                },
                48 => { // RSN (WPA/WPA2)
                    if element_len >= 2 {
                        encryption = "WPA2".to_string();
                    }
                },
                _ => {} // Ignore other elements
            }
            
            offset += 2 + element_len;
        }
        
        // Calculate RSSI (simplified simulation)
        let rssi = -50 - i8::try_from(data.len() % 30).unwrap_or(0);
        
        (ssid, channel, rssi, encryption)
    }
}
