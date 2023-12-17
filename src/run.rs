use crossterm::execute;
use crate::{printer, system};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveUp};
pub fn run() {
    println!("Fetching data ...");

    let sysinfo = system::get_info();

    execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap_or(());
    execute!(std::io::stdout(), MoveUp(1)).unwrap_or(());


    printer::print_sysinfo(sysinfo);
}