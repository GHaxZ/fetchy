use std::env;
use crossterm::execute;
use crate::{config, system};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveUp};
use crate::model::RGB;


pub fn run_normal() {
    println!("Fetching data ...");

    let sysinfo = system::get_info();

    execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap_or(());
    execute!(std::io::stdout(), MoveUp(1)).unwrap_or(());

    println!("{sysinfo}")
}


pub fn update_color_config(color: RGB) -> std::io::Result<()> {
    config::set_accent_color(color)
}

pub fn open_directory() -> std::io::Result<()>{
    let mut path = env::current_exe()?;
    path.pop();

    opener::open(&path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}