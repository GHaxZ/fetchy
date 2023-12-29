// TODO get gpu information

use std::env;
use chrono::Local;
use os_info::Bitness;
use sysinfo::*;
use winit::event_loop::EventLoop;
use crate::model::{Dimension, Drive, GPU, SystemInfo};

pub fn get_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.refresh_cpu();
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();

    SystemInfo {
        user: get_user(),
        current_directory: get_current_directory(),
        current_path: get_current_path(),
        time: get_time(),
        os: get_os(&sys),
        os_version: get_os_version(&sys),
        os_bits: get_os_bits(),
        host: get_host(&sys),
        uptime: get_uptime(&sys),
        screen_res: get_screen_res(),
        cpu_name: get_cpu_name(&sys),
        cpu_cores: get_cpu_cores(&sys),
        cpu_threads: get_cpu_threads(&sys),
        cpu_base_frequency: get_cpu_base_frequency(&sys),
        cpu_utilization: get_cpu_utilization(&sys),
        storage_drives: get_storage_drives(&sys),
        ram_total: get_ram_total(&sys),
        ram_swap: get_ram_swap(&sys),
        ram_used: get_ram_used(&sys),
    }
}

fn get_user() -> String {
    whoami::realname()
}

fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(dir) => {
            match dir.into_os_string().into_string() {
                Ok(dir_str) => { dir_str }
                Err(_) => { "/".to_string() }
            }
        }
        Err(_) => { "/".to_string() }
    }
}

fn get_current_path() -> String {
    match env::current_dir() {
        Ok(dir) => {
            match dir.file_name() {
                None => { "/".to_string() }
                Some(path) => {
                    match path.to_str() {
                        None => { "/".to_string() }
                        Some(path_str) => { path_str.to_string() }
                    }
                }
            }
        }
        Err(_) => { "/".to_string() }
    }
}

fn get_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}


fn get_os(sys: &System) -> String {
    match sys.name() {
        None => { "/".to_string() }
        Some(os_name) => { os_name }
    }
}

fn get_os_version(sys: &System) -> String {
    match sys.os_version() {
        None => { "".to_string() }
        Some(os_version) => { os_version }
    }
}

fn get_os_bits() -> String {
    let bits = os_info::get().bitness();

    match bits {
        Bitness::Unknown => { "/".to_string() }
        Bitness::X32 => { bits.to_string() }
        Bitness::X64 => { bits.to_string() }
        _ => { "/".to_string() }
    }
}

fn get_host(sys: &System) -> String {
    match sys.host_name() {
        None => { "/".to_string() }
        Some(host) => { host }
    }
}

fn get_uptime(sys: &System) -> String {
    let duration = sys.uptime();
    format!("{}:{:02}:{:02}", duration / 3600, (duration % 3600) / 60, duration % 60)
}

fn get_screen_res() -> Dimension {
    match EventLoop::new() {
        Ok(event) => {
            match event.primary_monitor() {
                None => {
                    Dimension {
                        width: 0,
                        height: 0,
                    }
                }
                Some(monitor) => {
                    Dimension {
                        width: monitor.size().width as i32,
                        height: monitor.size().height as i32,
                    }
                }
            }
        }
        Err(_) => {
            Dimension {
                width: 0,
                height: 0,
            }
        }
    }
}

fn get_cpu_name(sys: &System) -> String {
    sys.global_cpu_info().brand().trim().to_string()
}

fn get_cpu_base_frequency(sys: &System) -> u64 {
    match sys.cpus().iter().max_by(|a, b| a.frequency().cmp(&b.frequency())) {
        None => { 0 }
        Some(cpu) => { cpu.frequency() - 1 }
    }
}

fn get_cpu_cores(sys: &System) -> u32 {
    match sys.physical_core_count() {
        None => { 0 }
        Some(cores) => { cores as u32 }
    }
}

fn get_cpu_threads(sys: &System) -> u32 {
    sys.cpus().len() as u32
}

fn get_cpu_utilization(sys: &System) -> f32 {
    sys.global_cpu_info().cpu_usage()
}

/*fn get_gpus() -> Vec<GPU> {
    let mut vec: Vec<GPU> = Vec::new();

    for device in Device::all() {
        vec.push(GPU {
            name: device.name(),
            memory: device.memory(),
        })
    }

    vec
}*/

fn get_storage_drives(sys: &System) -> Vec<Drive> {
    let mut vec: Vec<Drive> = Vec::new();

    let disks = sys.disks();

    for disk in disks {
        vec.push(Drive {
            drive_path: match disk.mount_point().to_str() {
                None => { "/".to_string() }
                Some(disk_str) => { disk_str.to_string() }
            },
            storage_total: disk.total_space(),
            storage_used: disk.total_space() - disk.available_space(),
        });
    };

    vec
}

fn get_ram_total(sys: &System) -> u64 {
    sys.total_memory()
}

fn get_ram_swap(sys: &System) -> u64 {
    sys.total_swap()
}

fn get_ram_used(sys: &System) -> u64 {
    sys.used_memory()
}



