use serde::{Deserialize, Serialize};

pub struct Dimension {
    pub width: i32,
    pub height: i32
}

pub struct Drive {
    pub drive_path: String,
    pub storage_total: u64,
    pub storage_used: u64
}

#[derive(Serialize, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct SystemInfo {
    pub user: String,
    pub current_directory: String,
    pub current_path: String,
    pub time: String,
    pub os: String,
    pub os_version: String,
    pub os_bits: String,
    pub host: String,
    pub uptime: String,
    pub screen_res: Dimension,
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub cpu_base_frequency: u64,
    pub cpu_utilization: f32,
    pub storage_drives: Vec<Drive>,
    pub ram_total: u64,
    pub ram_swap: u64,
    pub ram_used: u64,
}
