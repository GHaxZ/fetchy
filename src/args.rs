
use clap::{Arg, Command, Error};
use clap::error::{ErrorKind};
use crate::run;

pub fn parse() {
    let command = Command::new("fetchy")
        .author("GHaxZ")
        .version("0.1.1")
        .about("A small command line system information tool")
        .arg(Arg::new("color")
            .short('c')
            .long("color")
            .value_name("R,G,B")
            .help("Sets the accent color of the tool."));

    let args = command.clone().get_matches();

    if !args.args_present() {
        run::run_normal()
    }

    if args.contains_id("color") {
        match args.get_one::<String>("color") {
            None => {}
            Some(color_arg) => {
                let arg_list: Vec<&str> = color_arg.split(',').collect();

                if arg_list.len() != 3 {
                    Error::new(ErrorKind::WrongNumberOfValues).with_cmd(&command).exit();
                }

                for arg in arg_list.iter() {
                    match arg.parse::<u8>() {
                        Ok(_) => {}
                        Err(_) => {
                            Error::new(ErrorKind::InvalidValue).with_cmd(&command).exit();
                        }
                    }
                }

                println!("Set the color to {},{},{}", arg_list.get(0).unwrap(), arg_list.get(1).unwrap(), arg_list.get(2).unwrap());
            }
        }
    }
}