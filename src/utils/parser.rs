use std::path::Path;

use clap::{arg, ArgMatches, Command};

use crate::utils::run;

pub fn get_parser() -> ArgMatches {
    let command = Command::new("ashdl")
        .arg_required_else_help(true)
        .disable_version_flag(true)
        .arg(arg!(-v - -verbose))
        .arg(arg!(-y - -ytdlp <VALUE>))
        .arg(arg!(-i - -clientid <VALUE>))
        .arg(arg!(-s - -clientsecret <VALUE>))
        .arg(arg!([ID]))
        .get_matches();
    return command;
}

pub fn parse_args() -> (bool, String, String, String, String) {
    let command = get_parser();
    if !Path::new(command.value_of("ytdlp").unwrap()).exists() {
        run::error("YT-DLP not found.", 5);
    }
    return (
        command.is_present("verbose"),
        command.value_of("ytdlp").unwrap().to_string(),
        command.value_of("clientid").unwrap().to_string(),
        command.value_of("clientsecret").unwrap().to_string(),
        command.value_of("ID").unwrap().to_string(),
    );
}

pub fn print_parser_details() {
    let parser = get_parser();
    println!("YT-DLP: {}", parser.value_of("ytdlp").unwrap());
    println!("Client Id: {}", parser.value_of("clientid").unwrap());
    println!(
        "Client Secret: {}",
        parser.value_of("clientsecret").unwrap()
    );
    println!("Playlist Id: {}", parser.value_of("ID").unwrap());
}

