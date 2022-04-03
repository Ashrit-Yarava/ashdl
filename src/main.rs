mod spotify;
mod utils;
mod youtube;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;
use async_std::task::block_on;
use colored::*;

#[async_std::main]
async fn main() {
    let (yt_dlp_str, client_id, client_secret, playlist_id) = utils::parser::parse();
    let yt_dlp = &yt_dlp_str;
    // println!("Input Information:");
    // println!("YT-DLP Path: {}", yt_dlp);
    // println!("Client ID: {}", client_id);
    // println!("Client Secret: {}", client_secret);
    // println!("Playlist ID: {}", playlist_id);

    println!("Beginning Playlist Download...");
    let songs = spotify::get_playlist(client_id, client_secret, playlist_id).await;
    songs.into_par_iter().for_each(|song| {
        let song_name = format!("{}", song);
        let file_name = format!("{}.mp3", song_name);

        if !Path::new(&file_name).exists() {
            youtube::download_song(yt_dlp, format!{"{}", song}, file_name);
            block_on(utils::tag::generate_id3_tag(song));  
            println!("+ {}", song_name.green());
        } else {
            println!("- {}", song_name.red());
        }
    })
}
