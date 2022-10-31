use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
    pub cpu_usage: f32,
    pub physical_core_count: usize,
    pub logical_processor_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoadAverage {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}
