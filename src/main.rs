mod system;
mod printer;

fn main() {
    println!("Fetching data ...");
    let sysinfo = system::get_info();


    printer::print_sysinfo(&sysinfo);
}
