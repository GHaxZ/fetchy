use crate::{config, helper};
use battery::units::time::second;
use battery::State;
use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wgpu::DeviceType;

const FIRST_SYMBOL: &str = "└─";
const OTHER_SYMBOL: &str = "├─";

pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

pub struct Gpu {
    pub name: String,
    pub gpu_type: DeviceType,
    pub driver: String,
}

pub struct Drive {
    pub drive_path: String,
    pub storage_total: u64,
    pub storage_used: u64,
}

pub struct Network {
    pub network_name: String,
    pub network_up: u64,
    pub network_down: u64,
}

pub struct Battery {
    pub current_load: u64,
    pub max_load: u64,
    pub current_state: State,
    pub until_empty_or_full: Option<battery::units::Time>,
}

#[derive(Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct SystemInfo {
    pub user: String,
    pub current_path: String,
    pub time: String,
    pub os: String,
    pub os_version: String,
    pub os_bits: String,
    pub host: String,
    pub uptime: String,
    pub networks: Vec<Network>,
    pub screen_resolutions: Vec<Dimension>,
    pub batteries: Option<Vec<Battery>>,
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub cpu_base_frequency: u64,
    pub cpu_utilization: f32,
    pub gpus: Vec<Gpu>,
    pub storage_drives: Vec<Drive>,
    pub ram_total: u64,
    pub ram_used: u64,
    pub ram_swap: u64,
}

impl Display for SystemInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = config::get_accent_color();

        let mut str = String::new();

        str.push_str(
            format!(
                "{}{}{} | {}",
                self.user.as_str().with(color).bold(),
                "@".bold(),
                self.current_path.as_str().with(color).bold(),
                self.time.as_str()
            )
            .as_str(),
        );

        str.push_str(
            format!(
                "\n{}",
                "─".repeat(self.user.len() + self.current_path.len() + self.time.len() + 4)
            )
            .as_str(),
        );

        str.push_str(
            format!(
                "\n{}: {} {} {}",
                "OS".with(color).bold(),
                self.os,
                self.os_version,
                self.os_bits
            )
            .as_str(),
        );

        str.push_str(format!("\n{}: {}", "Host".with(color).bold(), self.host).as_str());

        str.push_str(format!("\n{}: {}", "Uptime".with(color).bold(), self.uptime).as_str());

        for i in 0..self.networks.len() {
            if let Some(network) = self.networks.get(i) {
                str.push_str(
                    format!(
                        "\n{}: {}",
                        "Network".with(color).bold(),
                        network.network_name
                    )
                    .as_str(),
                );

                str.push_str(
                    format!(
                        "\n{}: {}MB ↑ {}MB ↓",
                        "└─ Total usage".with(color).bold(),
                        network.network_up / 1_000_100,
                        network.network_down / 1_000_000
                    )
                    .as_str(),
                );
            }
        }

        str.push_str(&format!(
            "\n{}: {}",
            "Resolution".with(color).bold(),
            self.screen_resolutions
                .iter()
                .map(|screen_res| format!("{}x{}", screen_res.width, screen_res.height))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        if let Some(batteries) = &self.batteries {
            for battery in batteries.iter() {
                let mut charge_info = String::new();

                if battery.until_empty_or_full.is_some() {
                    let time = helper::sec_to_h_m_str(
                        battery.until_empty_or_full.unwrap().get::<second>() as i64,
                    );
                    if battery.current_state == State::Charging {
                        charge_info.push_str(format!("{} until full", time).as_str());
                    } else if battery.current_state == State::Discharging {
                        charge_info.push_str(format!("{} until empty", time).as_str());
                    }
                }

                str.push_str(
                    format!(
                        "\n{}: {}Wh / {}Wh - {:.0}%",
                        "Battery".with(color).bold(),
                        battery.current_load,
                        battery.max_load,
                        (battery.current_load as f64 / battery.max_load as f64) * 100.0
                    )
                    .as_str(),
                );

                str.push_str(
                    format!(
                        "\n{}: {:?}{}",
                        "└─ State".with(color).bold(),
                        battery.current_state,
                        if !charge_info.is_empty() {
                            format!(", {}", charge_info)
                        } else {
                            "".to_string()
                        }
                    )
                    .as_str(),
                );
            }
        }

        str.push_str(format!("\n{}: {}", "CPU".with(color).bold(), self.cpu_name).as_str());

        str.push_str(
            format!(
                "\n{}: {}C {}T @ {:.2}GHz",
                "├─ Details".with(color).bold(),
                self.cpu_cores,
                self.cpu_threads,
                self.cpu_base_frequency as f32 / 1000.0,
            )
            .as_str(),
        );

        str.push_str(
            format!(
                "\n{}: {:.1}%",
                "└─ Utilization".with(color).bold(),
                self.cpu_utilization
            )
            .as_str(),
        );

        for gpu in self.gpus.iter() {
            let mut type_str = String::new();

            if gpu.gpu_type == DeviceType::DiscreteGpu {
                type_str.push_str("Discrete GPU");
            } else if gpu.gpu_type == DeviceType::IntegratedGpu {
                type_str.push_str("Integrated GPU");
            }

            str.push_str(
                format!(
                    "\n{}: {} ({})",
                    "GPU".with(color).bold(),
                    gpu.name,
                    type_str
                )
                .as_str(),
            );

            str.push_str(format!("\n{}: {}", "└─ Driver".with(color).bold(), gpu.driver).as_str());
        }

        str.push_str(
            format!(
                "\n{}: {}GB",
                "RAM".with(color).bold(),
                self.ram_total / 1_000_000_000
            )
            .as_str(),
        );

        str.push_str(
            format!(
                "\n{}: {}GB - {:.1}%",
                "├─ Used".with(color).bold(),
                self.ram_used / 1_000_000_000,
                (self.ram_used as f64 / self.ram_total as f64) * 100.0
            )
            .as_str(),
        );

        str.push_str(
            format!(
                "\n{}: {}GB",
                "└─ Swap".with(color).bold(),
                self.ram_swap / 1_000_000_000
            )
            .as_str(),
        );

        str.push_str(format!("\n{}", "Storage".with(color).bold()).as_str());

        for i in 0..self.storage_drives.len() {
            let symbol = if i == self.storage_drives.len() - 1 {
                FIRST_SYMBOL
            } else {
                OTHER_SYMBOL
            };

            if let Some(drive) = self.storage_drives.get(i) {
                str.push_str(
                    format!(
                        "\n{} {} {}GB / {}GB - {:.1}%",
                        symbol.with(color).bold(),
                        drive.drive_path.as_str().with(color).bold(),
                        drive.storage_used / 1_000_000_000,
                        drive.storage_total / 1_000_000_000,
                        (drive.storage_used as f64 / drive.storage_total as f64) * 100.0
                    )
                    .as_str(),
                );
            }
        }

        write!(f, "{str}")
    }
}
