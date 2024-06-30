use crate::helper::{self};
use crate::model::{Battery, Dimension, Drive, Gpu, Network, SystemInfo};
use battery::units::energy::watt_hour;
use battery::units::Time;
use battery::State;
use chrono::Local;
use os_info::Bitness;
use std::env;
use sysinfo::{Disks, Networks, System};
use wgpu::{Backends, InstanceDescriptor};
use winit::event_loop::EventLoop;

pub fn get_info() -> SystemInfo {
    let mut sysinfo = System::new_all();
    sysinfo.refresh_all();
    sysinfo.refresh_cpu();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sysinfo.refresh_cpu();

    SystemInfo {
        user: get_user(),
        current_directory: get_current_directory(),
        current_path: get_current_path(),
        time: get_time(),
        os: get_os(),
        os_version: get_os_version(),
        os_bits: get_os_bits(),
        host: get_host(),
        networks: get_networks(),
        uptime: get_uptime(),
        screen_res: get_screen_res(),
        batteries: get_battery(),
        cpu_name: get_cpu_name(&sysinfo),
        cpu_cores: get_cpu_cores(&sysinfo),
        cpu_threads: get_cpu_threads(&sysinfo),
        cpu_base_frequency: get_cpu_base_frequency(&sysinfo),
        cpu_utilization: get_cpu_utilization(&sysinfo),
        gpus: get_gpus(),
        storage_drives: get_storage_drives(),
        ram_total: get_ram_total(&sysinfo),
        ram_swap: get_ram_swap(&sysinfo),
        ram_used: get_ram_used(&sysinfo),
    }
}

fn get_user() -> String {
    whoami::realname()
}

fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(dir) => match dir.into_os_string().into_string() {
            Ok(dir_str) => dir_str,
            Err(_) => "/".to_string(),
        },
        Err(_) => "/".to_string(),
    }
}

fn get_current_path() -> String {
    match env::current_dir() {
        Ok(dir) => match dir.file_name() {
            None => "/".to_string(),
            Some(path) => match path.to_str() {
                None => "/".to_string(),
                Some(path_str) => path_str.to_string(),
            },
        },
        Err(_) => "/".to_string(),
    }
}

fn get_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn get_os() -> String {
    match System::name() {
        None => "/".to_string(),
        Some(os_name) => os_name,
    }
}

fn get_os_version() -> String {
    match System::os_version() {
        None => "".to_string(),
        Some(os_version) => os_version,
    }
}

fn get_os_bits() -> String {
    let bits = os_info::get().bitness();

    match bits {
        Bitness::Unknown => "/".to_string(),
        Bitness::X32 => bits.to_string(),
        Bitness::X64 => bits.to_string(),
        _ => "/".to_string(),
    }
}

fn get_host() -> String {
    match System::host_name() {
        None => "/".to_string(),
        Some(host) => host,
    }
}

fn get_uptime() -> String {
    let duration = System::uptime();
    helper::sec_to_h_m_s_str(duration as i64)
}

fn get_networks() -> Vec<Network> {
    let mut vec: Vec<Network> = Vec::new();

    let networks = Networks::new_with_refreshed_list();

    for (interface_name, data) in &networks {
        if data.total_received() > 0 && data.total_transmitted() > 0 {
            vec.push(Network {
                network_name: interface_name.to_string(),
                network_up: data.total_transmitted(),
                network_down: data.total_received(),
            })
        }
    }

    vec
}

fn get_screen_res() -> Dimension {
    match EventLoop::new() {
        Ok(event) => match event.primary_monitor() {
            None => Dimension {
                width: 0,
                height: 0,
            },
            Some(monitor) => Dimension {
                width: monitor.size().width as i32,
                height: monitor.size().height as i32,
            },
        },
        Err(_) => Dimension {
            width: 0,
            height: 0,
        },
    }
}

fn get_battery() -> Option<Vec<Battery>> {
    let mut vec: Vec<Battery> = Vec::new();

    if let Ok(manager) = battery::Manager::new() {
        if let Ok(batteries) = manager.batteries() {
            for battery in batteries.flatten() {
                let mut time: Option<Time> = None;

                if battery.state() == State::Discharging {
                    time = battery.time_to_empty()
                } else if battery.state() == State::Charging {
                    time = battery.time_to_full()
                }

                vec.push(Battery {
                    current_load: battery.energy().get::<watt_hour>() as u64,
                    max_load: battery.energy_full().get::<watt_hour>() as u64,
                    current_state: battery.state(),
                    until_empty_or_full: time,
                })
            }
        }
    }

    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

fn get_cpu_name(sys: &System) -> String {
    sys.global_cpu_info().brand().trim().to_string()
}

fn get_cpu_base_frequency(sys: &System) -> u64 {
    match sys
        .cpus()
        .iter()
        .max_by(|a, b| a.frequency().cmp(&b.frequency()))
    {
        None => 0,
        Some(cpu) => cpu.frequency() - 1,
    }
}

fn get_cpu_cores(sys: &System) -> u32 {
    match sys.physical_core_count() {
        None => 0,
        Some(cores) => cores as u32,
    }
}

fn get_cpu_threads(sys: &System) -> u32 {
    sys.cpus().len() as u32
}

fn get_cpu_utilization(sys: &System) -> f32 {
    sys.global_cpu_info().cpu_usage()
}

fn get_gpus() -> Vec<Gpu> {
    let mut vec: Vec<Gpu> = Vec::new();

    let ins = wgpu::Instance::new(InstanceDescriptor::default());

    for adapter in ins.enumerate_adapters(Backends::VULKAN) {
        let info = adapter.get_info();

        vec.push(Gpu {
            name: info.name,
            gpu_type: info.device_type,
            driver: info.driver_info,
        });
    }

    vec
}

fn get_storage_drives() -> Vec<Drive> {
    let mut vec: Vec<Drive> = Vec::new();

    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        vec.push(Drive {
            drive_path: match disk.mount_point().to_str() {
                None => "/".to_string(),
                Some(disk_str) => disk_str.to_string(),
            },
            storage_total: disk.total_space(),
            storage_used: disk.total_space() - disk.available_space(),
        });
    }

    vec
}

fn get_ram_total(sys: &System) -> u64 {
    sys.total_memory()
}

fn get_ram_used(sys: &System) -> u64 {
    sys.used_memory()
}

fn get_ram_swap(sys: &System) -> u64 {
    sys.total_swap()
}
