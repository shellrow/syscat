use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInterfaceInfo {
    pub index: u32,
    pub name: String,
    pub mac: String,
    pub ipv4_addr: Vec<String>,
    pub ipv6_addr: Vec<String>,
    pub gateway_ip: String,
    pub gateway_mac: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkUsageInfo {
    pub interface_name: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64, 
    pub packets_transmitted: u64, 
    pub total_packets_transmitted: u64,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
}

impl NetworkInterfaceInfo {
    pub fn new() -> NetworkInterfaceInfo {
        NetworkInterfaceInfo { 
            index: 0, 
            name: String::new(), 
            mac: String::new(), 
            ipv4_addr: vec![], 
            ipv6_addr: vec![], 
            gateway_ip: String::new(), 
            gateway_mac: String::new() 
        }
    }

    pub fn default() -> NetworkInterfaceInfo {
        let interface: NetworkInterfaceInfo = NetworkInterfaceInfo::new();
        // TODO
        interface
    }

    pub fn get_default_ipv4_addr() -> String {
        // TODO
        String::new()
    }

    pub fn get_default_ipv6_addr() -> String {
        // TODO
        String::new()
    }

}
