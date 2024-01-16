use std::env;
use crate::{config, system};
use spinoff::{Color, Spinner, spinners};
use crate::model::Rgb;

pub fn run_normal() {
    let mut spinner = Spinner::new(spinners::Aesthetic, "Fetching data".to_string(), Color::White);

    let sysinfo = system::get_info();

    spinner.clear();

    println!("{sysinfo}")
}


pub fn update_color_config(color: Rgb) -> std::io::Result<()> {
    config::set_accent_color(color)
}

pub fn open_directory() -> std::io::Result<()>{
    let mut path = env::current_exe()?;
    path.pop();

    opener::open(&path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}