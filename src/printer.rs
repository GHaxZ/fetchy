use crate::model::SystemInfo;
use crossterm::style::{Stylize};

pub fn print_sysinfo(sys: SystemInfo) {


    println!("{}{}{} | {}",
        sys.user.as_str().red().bold(),
        "@".bold(),
        sys.current_path.as_str().red().bold(),
        sys.time.as_str()
    );

    println!("{}", "─".repeat(sys.user.len() + sys.current_path.len() + sys.time.len() + 4));

    println!("{}: {} {} {}",
        "OS".red().bold(),
        sys.os,
        sys.os_version,
        sys.os_bits
    );

    println!("{}: {}",
        "Host".red().bold(),
        sys.host
    );

    println!("{}: {}",
        "Uptime".red().bold(),
        sys.uptime
    );

    println!("{}: {}x{}",
             "Resolution".red().bold(),
             sys.screen_res.width,
             sys.screen_res.height
    );

    println!("{}: {}",
        "CPU".red().bold(),
        sys.cpu_name
    );

    println!("{}: {}C {}T @ {:.2}GHz",
        "├ Details".red().bold(),
        sys.cpu_cores,
        sys.cpu_threads,
        sys.cpu_base_frequency as f32 / 1000.0
    );

    println!("{}: {:.1}%",
        "└ Utilization".red().bold(),
        sys.cpu_utilization
    );

    println!("{}: {}MB",
        "RAM".red().bold(),
        sys.ram_total / 1_000_000
    );

    println!("{}: {}MB",
        "├ Swap".red().bold(),
        sys.ram_swap / 1_000_000
    );

    println!("{}: {}MB - {:.1}%",
        "└ Used".red().bold(),
        sys.ram_used / 1_000_000,
        (sys.ram_used as f64 / sys.ram_total as f64) * 100.0
    );

    println!("{}",
        "Storage".red().bold()
    );

    for drive in sys.storage_drives {
        println!("{} {} {}GB / {}GB - {:.1}%",
            "└".red().bold(),
            drive.drive_path.red().bold(),
            drive.storage_used / 1_000_000_000,
            drive.storage_total / 1_000_000_000,
            (drive.storage_used as f64 / drive.storage_total as f64) * 100.0
        );
    }
}