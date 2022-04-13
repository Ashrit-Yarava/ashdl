mod spotify;
mod utils;
mod youtube;

use async_std::task::block_on;
use colored::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;

#[async_std::main]
async fn main() {
    let (yt_dlp_str, client_id, client_secret, playlist_id) = utils::parser::parse();
    let yt_dlp = &yt_dlp_str;

    println!("Beginning Playlist Download...");
    let songs = spotify::get_playlist(client_id, client_secret, playlist_id).await;
    songs.into_par_iter().for_each(|song| {
        let song_name = format!("{}", song).replace("/", "");
        let s_name = format!("{}", song).replace("/", "");
        let file_name_temp = format!("{}.mp3", song_name);
        let file_name = file_name_temp
            .chars()
            .filter(|c| c.is_ascii())
            .collect::<String>();
        let f_name_temp = format!("{}.mp3", song_name);
        let f_name = f_name_temp
            .chars()
            .filter(|c| c.is_ascii())
            .collect::<String>();

        if !Path::new(&file_name).exists() {
            youtube::download_song(yt_dlp, s_name, file_name);
            block_on(utils::tag::generate_id3_tag(song, f_name));
            println!("+ {}", song_name.green());
        } else {
            println!("- {}", song_name.red());
        }
    })
}
