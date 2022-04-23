use std::{path::Path, process::Command};
use futures::executor::block_on;

use crate::spotify::song::Song;
use crate::spotify::tag::generate_id3_tag;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// YT-DLP download command.
fn download_command_youtube(yt_dlp: &str, song: &Song) {
    let _command = Command::new(yt_dlp)
        .args([
            "-x",
            "-f",
            "bestaudio",
            "--audio-format",
            "mp3",
            "--playlist-items",
            "1",
            "--default-search",
            "https://music.youtube.com/search?q=",
            "scsearch:",
            &song.get_query(),
            "-o",
            &song.get_filename(),
        ])
        .output()
        .expect("yt-dlp download failed.");
}

fn download_command_soundcloud(yt_dlp: &str, song: &Song) {
    let _command = Command::new(yt_dlp)
        .args([
            "-x",
            "-f",
            "bestaudio",
            "--audio-format",
            "mp3",
            "--playlist-items",
            "1",
            "--default-search",
            "scsearch:",
            &song.get_query(),
            "-o",
            &song.get_filename(),
        ])
        .output()
        .expect("yt-dlp download failed.");
}

/// Check if the file is already present, if not then download the song.
pub async fn download_songs(verbose: bool, yt_dlp: String, songs: Vec<Song>, usesoundcloud: bool) {
    songs.into_par_iter().for_each(|song| {
        if !Path::new(&song.get_filename()).exists() {
            if usesoundcloud {
                download_command_soundcloud(&yt_dlp, &song);
            } else {
                download_command_youtube(&yt_dlp, &song);
            }
            block_on(generate_id3_tag(&song));
            if verbose {
                println!("* {}", song);
            }
        } else {
            if verbose {
                println!("- {}", song);
            }
        }
    });
}
