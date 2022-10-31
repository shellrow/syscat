use serde::{Deserialize, Serialize};
use crate::disk::DiskUsageInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmd: String,
    pub full_path: String,
    pub environ: String,
    pub cw_dir: String,
    pub root_dir: String,
    pub memory: u64,
    pub virtual_memory: u64,
    pub parent: u32,
    pub status: String,
    pub start_time: u64,
    pub cpu_usage: f32,
    pub disk_usage: DiskUsageInfo,
}
