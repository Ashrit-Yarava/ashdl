// Process For Playlists
use aspotify::{Client, PlaylistItemType, Track};

use crate::spotify::Song;

fn generate_blank() -> Song {
    return Song {
        title: "".to_string(),
        artists: vec![],
        album: "".to_string(),
        img: "".to_string()
    }
}

fn make_song(track: Track) -> Song {
    let artists = track.artists;
    let image = track.album.images[0].url.to_string();
    let album_name = track.album.name;
    let mut str_artists: Vec<String> = vec![];
    for artist in artists.iter() {
        str_artists.push(artist.to_owned().name);
    }
    return Song {
        title: track.name,
        artists: str_artists,
        album: album_name,
        img: image
    }
}

pub async fn get_songs(client: Client, playlist_id: String) -> Vec<Song> {
    let tracks = client
        .playlists()
        .get_playlist(&playlist_id, None)
        .await
        .unwrap()
        .data
        .tracks;

    let mut songs: Vec<Song> = vec![];
    for track in tracks.items.iter() {
        let song = match track.to_owned().item {
            Some(track_item) => match track_item {
                PlaylistItemType::Track(r) => make_song(r),
                PlaylistItemType::Episode(_r) => generate_blank(),
            },
            None => generate_blank()
        };
        if song.title != "" {
            songs.push(song);
        }
    };
    return songs;
}
