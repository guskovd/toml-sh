extern crate clap;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App, SubCommand};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("Toml-sh")
        .version("1.0")
        .author("Danil Guskov <guskovd86@mail.ru>")
        .about("My Simple Cli")
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("export")
                    .about("export env. variables")
                    .arg(Arg::with_name("toml")
                         .help("Sets the toml file to use")
                         .required(true)
                         .index(1))
        )
        .subcommand(SubCommand::with_name("unset")
                    .about("unset env. variables")
                    .arg(Arg::with_name("toml")
                         .help("Sets the toml file to use")
                         .required(true)
                         .index(1))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("export") {
        let mut toml_file = File::open(matches.value_of("toml").unwrap()).expect("file not found");

        let mut contents = String::new();
        toml_file.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let a = toml::decode_str(contents);

    }

    if let Some(matches) = matches.subcommand_matches("unset") {
        println!("unset");
    }
}
