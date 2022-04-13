use std::process::Command;

fn download_command(yt_dlp: &str, search_query: String, file_name: String) {
    let _command = Command::new(yt_dlp)
        .args([
            "-x",
            "--audio-format",
            "mp3",
            "--playlist-items",
            "1",
            "--default-search",
            "https://music.youtube.com/search?q=",
            &*format!("\"{} (Lyrics)\"", search_query),
            "-o",
            &file_name,
        ])
        .output()
        .expect("IDK");
}

pub fn download_song(yt: &str, song_name: String, file_name: String) {
    download_command(yt, song_name, file_name);
}
