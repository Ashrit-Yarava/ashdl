use rspotify::ClientCredsSpotify;
use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::clients::BaseClient;
use rspotify::model::{FullTrack, Id, PlaylistId, PlayableItem};

use crate::spotify::Song;

fn generate_blank() -> Song {
    return Song {
        title: "".to_string(),
        artists: vec![],
        album: "".to_string(),
        img: "".to_string()
    }
}

// NEED TO CHANGE
fn make_song(track: FullTrack) -> Song {
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

pub async fn songs(client: ClientCredsSpotify, playlist_id: String) -> Vec<Song> {
    let id = PlaylistId::from_id(&playlist_id).unwrap();
    let playlist = client.playlist_items(&id, Option::from(""), None);
    pin_mut!(playlist);
    let mut songs: Vec<Song> = vec![];
    while let Some(item) = playlist.try_next().await.unwrap() {
        let song = match item.track.unwrap() {
            PlayableItem::Track(s) => make_song(s),
            PlayableItem::Episode(_s) => generate_blank(),
        };
        if song.title != "" {
            songs.push(song);
        }
    }
    return songs;

}