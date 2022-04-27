use std::path::Path;

use clap::{arg, Arg, ArgMatches, Command};

use crate::utils::run;

pub fn get_parser() -> ArgMatches {
    let command = Command::new("ashdl")
        .arg_required_else_help(true)
        .disable_version_flag(true)
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .help("Display verbose output."),
        )
        .arg(
            Arg::new("ytdlp")
                .long("ytdlp")
                .short('y')
                .takes_value(true)
                .help("YT DLP Path."),
        )
        .arg(
            Arg::new("clientid")
                .long("clientid")
                .short('i')
                .takes_value(true)
                .help("Client ID from Spotify API."),
        )
        .arg(
            Arg::new("clientsecret")
                .long("clientsecret")
                .short('s')
                .takes_value(true)
                .help("Client Secret from Spotify API."),
        )
        .arg(
            Arg::new("soundcloud")
                .long("soundcloud")
                .short('c')
                .takes_value(false)
                .help("Whether to use soundcloud instead of youtube music.")
        )
        .arg(arg!([TYPE]).help("Type of Spotify Object [playlist/album]"))
        .arg(arg!([ID]).help("Playlist ID. (NOT THE URL)"))
        .get_matches();
    return command;
}

pub fn parse_args() -> (bool, bool, String, String, String, String, String) {
    let command = get_parser();
    if !Path::new(command.value_of("ytdlp").unwrap()).exists() {
        run::error("YT-DLP not found.", 5);
    }
    let spotify_type = command.value_of("TYPE").unwrap().to_string();

    if spotify_type != "playlist" && spotify_type != "album" {
        run::error("Invalid type for spotify id.", 10);
    }
    return (
        command.is_present("verbose"),
        command.is_present("soundcloud"),
        command.value_of("ytdlp").unwrap().to_string(),
        command.value_of("clientid").unwrap().to_string(),
        command.value_of("clientsecret").unwrap().to_string(),
        spotify_type,
        command.value_of("ID").unwrap().to_string(),
    );
}

pub fn print_parser_details() {
    let parser = get_parser();
    println!("Use Soundcloud: {}",
             parser.is_present("soundcloud")
    );
    println!("YT-DLP: {}", parser.value_of("ytdlp").unwrap());
    println!("Client Id: {}", parser.value_of("clientid").unwrap());
    println!(
        "Client Secret: {}",
        parser.value_of("clientsecret").unwrap()
    );
    println!("Playlist Id: {}", parser.value_of("ID").unwrap());
}
