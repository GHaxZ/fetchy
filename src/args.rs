use clap::{Arg, Command, Error};
use clap::error::{ErrorKind};
use crate::model::RGB;
use crate::{config, run};

pub fn parse() {
    let command = Command::new("fetchy")
        .author("GHaxZ")
        .version("0.1.1")
        .about("A small command line system information tool")
        .arg(Arg::new("color")
            .short('c')
            .long("color")
            .value_name("R,G,B")
            .help("Sets the accent color of the tool. Can be reset using the \"default\" value."));

    let args = command.clone().get_matches();

    if !args.args_present() {
        run::run_normal()
    }

    if args.contains_id("color") {
        match args.get_one::<String>("color") {
            None => {}
            Some(color_arg) => {
                if color_arg == "default" {
                    match config::reset_accent_color() {
                        Ok(_) => {
                            println!("Successfully reset the accent color to its default value.");
                            return;
                        }
                        Err(_) => {
                            Error::new(ErrorKind::InvalidValue).with_cmd(&command).exit();
                        }
                    }
                }

                let arg_list: Vec<&str> = color_arg.split(',').collect();

                if arg_list.len() != 3 {
                    Error::new(ErrorKind::WrongNumberOfValues).with_cmd(&command).exit();
                }

                let mut arg_parse: Vec<u8> = vec![];
                for arg in arg_list.iter() {
                    match arg.parse::<u8>() {
                        Ok(a) => { arg_parse.push(a) }
                        Err(_) => {
                            Error::new(ErrorKind::InvalidValue).with_cmd(&command).exit();
                        }
                    }
                }

                let color: (u8, u8, u8) = (arg_parse.get(0).unwrap().to_owned(),
                                           arg_parse.get(1).unwrap().to_owned(),
                                           arg_parse.get(2).unwrap().to_owned());

                match run::update_color_config(RGB {
                    r: color.0,
                    g: color.1,
                    b: color.2
                }) {
                    Ok(_) => {
                        println!("Successfully changed the accent color to {},{},{}", color.0, color.1, color.2)
                    }
                    Err(_) => {
                        Error::new(ErrorKind::Io).with_cmd(&command).exit();
                    }
                }
            }
        }
    }
}