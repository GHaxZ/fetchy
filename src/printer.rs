use crate::model::SystemInfo;
use crossterm::style::{Stylize};
use crate::config;

pub fn print_sysinfo(sys: SystemInfo) {
    let color = config::get_accent_color();

    println!("{}{}{} | {}",
        sys.user.as_str().with(color).bold(),
        "@".bold(),
        sys.current_path.as_str().with(color).bold(),
        sys.time.as_str()
    );

    println!("{}", "─".repeat(sys.user.len() + sys.current_path.len() + sys.time.len() + 4));

    println!("{}: {} {} {}",
        "OS".with(color).bold(),
        sys.os,
        sys.os_version,
        sys.os_bits
    );

    println!("{}: {}",
        "Host".with(color).bold(),
        sys.host
    );

    println!("{}: {}",
        "Uptime".with(color).bold(),
        sys.uptime
    );

    println!("{}: {}x{}",
             "Resolution".with(color).bold(),
             sys.screen_res.width,
             sys.screen_res.height
    );

    println!("{}: {}",
        "CPU".with(color).bold(),
        sys.cpu_name
    );

    println!("{}: {}C {}T @ {:.2}GHz",
        "├ Details".with(color).bold(),
        sys.cpu_cores,
        sys.cpu_threads,
        sys.cpu_base_frequency as f32 / 1000.0
    );

    println!("{}: {:.1}%",
        "└ Utilization".with(color).bold(),
        sys.cpu_utilization
    );

    println!("{}: {}MB",
        "RAM".with(color).bold(),
        sys.ram_total / 1_000_000
    );

    println!("{}: {}MB",
        "├ Swap".with(color).bold(),
        sys.ram_swap / 1_000_000
    );

    println!("{}: {}MB - {:.1}%",
        "└ Used".with(color).bold(),
        sys.ram_used / 1_000_000,
        (sys.ram_used as f64 / sys.ram_total as f64) * 100.0
    );

    println!("{}",
        "Storage".with(color).bold()
    );

    for drive in sys.storage_drives {
        println!("{} {} {}GB / {}GB - {:.1}%",
            "└".with(color).bold(),
            drive.drive_path.with(color).bold(),
            drive.storage_used / 1_000_000_000,
            drive.storage_total / 1_000_000_000,
            (drive.storage_used as f64 / drive.storage_total as f64) * 100.0
        );
    }
}