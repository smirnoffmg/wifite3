use std::collections::HashMap;
use crate::network::{WiFiNetwork, NetworkData};

/// Network builder for managing discovered networks
pub struct NetworkBuilder {
    networks: HashMap<String, WiFiNetwork>,
}

impl NetworkBuilder {
    pub fn new() -> Self {
        Self {
            networks: HashMap::new(),
        }
    }

    pub fn add_network(&mut self, data: NetworkData) {
        let network = WiFiNetwork::new(
            data.ssid,
            data.bssid.clone(),
            data.channel,
            data.rssi,
            data.encryption,
        );
        
        self.networks.insert(data.bssid, network);
    }

    pub fn get_networks(self) -> Vec<WiFiNetwork> {
        self.networks.into_values().collect()
    }
}
