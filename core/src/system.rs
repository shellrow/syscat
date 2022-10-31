use serde::{Deserialize, Serialize};

use crate::cpu::{CpuInfo, LoadAverage};
use crate::memory::MemoryInfo;
use crate::disk::DiskInfo;
use crate::network::NetworkInterfaceInfo;
use crate::user::UserInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OsInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub long_os_version: String,
    pub host_name: String,
    pub uptime: u64,
    pub boot_time: u64,
    pub load_average: LoadAverage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemOverview {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub network_interfaces: Vec<NetworkInterfaceInfo>,
    pub os: OsInfo,
    pub users: Vec<UserInfo>,
}
