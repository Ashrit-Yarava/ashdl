use rspotify::model::{FullTrack, SimplifiedArtist};
use std::fmt;

/// A simplified form of the rspotify FullTrack struct.
pub struct Song {
    pub(crate) title: String,
    pub(crate) artists: Vec<String>,
    pub(crate) album: String,
    pub(crate) img: Vec<u8>,
}

impl Song {
    /// Generate a new Song struct given the rspotify track.
    pub async fn new(track: FullTrack) -> Song {
        return Song {
            title: track.name,
            artists: Song::get_artists(track.artists),
            album: track.album.name,
            img: Song::get_image(track.album.images[0].url.to_string()).await,
        };
    }

    pub fn get_query(&self) -> String {
        return format!("{}", self);
    }

    /// Clean up the title and generate a filename.
    pub fn get_filename(&self) -> String {
        let mut title = format!("{} - {}.mp3", self.artists_str(), self.title);
        // Reserved Characters: ?:\"*|/\\<>
        // These characters give problems for android.
        title = title
            .chars()
            .map(|c| match c {
                '?' => ' ',
                ':' => ' ',
                '\"' => ' ',
                '*' => ' ',
                '|' => ' ',
                '/' => ' ',
                '\\' => ' ',
                '<' => ' ',
                '>' => ' ',
                _ => c,
            })
            .collect();
        while title.contains("  ") {
            title = title.replace("  ", " ");
        }
        title = title.replace(" .", "."); // If the song name ends with a quote.

        while title.starts_with(".") {
            title = title[1..].parse().unwrap();
        }

        return title;
    }

    pub fn artists_str(&self) -> String {
        return self.artists.join(", ");
    }

    /// Generate a blank version of the Song struct.
    pub fn blank() -> Song {
        return Song {
            title: "".to_string(),
            artists: vec![],
            album: "".to_string(),
            img: vec![],
        };
    }

    /// Converts rspotify artists list to a string list of names.
    fn get_artists(given_artists: Vec<SimplifiedArtist>) -> Vec<String> {
        let mut artists: Vec<String> = vec![];
        for artist in given_artists.iter() {
            artists.push(artist.name.to_string());
        }
        return artists;
    }

    /// Download an image given a url string.
    async fn get_image(url: String) -> Vec<u8> {
        let response = reqwest::get(url).await.expect("Failed to download image.");
        return response
            .bytes()
            .await
            .expect("Failed to extract bytes.")
            .to_vec();
    }
}

/// Prints the length of the playlist.
pub fn print_length(songs: &Vec<Song>) {
    println!("Length: {}", songs.len());
}

/// Testing function to print details about the list of songs.
pub fn print_songs(songs: &Vec<Song>) {
    print_length(songs);
    for song in songs.iter() {
        println!(
            "{}\n\tQuery: {}\n\tFilename: {}",
            song,
            song.get_query(),
            song.get_filename()
        );
    }
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let artists = self.artists.join(", ");
        let title = &self.title;
        return write!(f, "{} - {}", artists, title);
    }
}

