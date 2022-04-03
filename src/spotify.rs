mod playlist;
mod utils;

use std::fmt;

// Define the general structure for each song.
// Attributes are mainly for the id3 tag.
pub struct Song {
    pub(crate) title: String,
    pub(crate) artists: Vec<String>,
    pub(crate) album: String,
    pub(crate) img: String,
}


impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.title, self.artists.join(", "))
    } 
}

pub async fn get_playlist(client_id: String,
                    client_secret: String, 
                    playlist_id: String) -> Vec<Song> {
    let client = utils::get_client(client_id, client_secret).await;
    let songs = playlist::songs(client, playlist_id).await;
    return songs;
}
