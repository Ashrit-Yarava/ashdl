use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    ytdlp: Option<String>,

    #[clap(short, long)]
    client: Option<String>,

    #[clap(short, long)]
    secret: Option<String>,

    // The playlist to get the songs from.
    playlist_id: Option<String>,
}

fn error_playlist_id() -> &'static str {
    println!("Playlist ID is invalid");
    return "";
}

fn error_client_id() -> &'static str {
    println!("Client ID is required!");
    return "";
}

fn error_client_secret() -> &'static str {
    println!("Client Secret is required!");
    return "";
}

fn error_yt_path() -> &'static str {
    println!("Specify yt-dlp path!");
    return "";
}

pub fn parse() -> (String, String, String, String) {
    let args = Args::parse();

    let yt_dlp = match args.ytdlp.as_deref() {
        Some(x) => x,
        None => error_yt_path(),
    };

    let client_id = match args.client.as_deref() {
        Some(x) => x,
        None => error_client_id(),
    };

    let client_secret = match args.secret.as_deref() {
        Some(x) => x,
        None => error_client_secret(),
    };

    let playlist_id = match args.playlist_id.as_deref() {
        Some(x) => x,
        None => error_playlist_id(),
    };

    if (yt_dlp == "") || (client_id == "") || (client_secret == "") || (playlist_id == "") {
        process::exit(1);
    }

    return (
        yt_dlp.to_string(),
        client_id.to_string(),
        client_secret.to_string(),
        playlist_id.to_string(),
    );
}
