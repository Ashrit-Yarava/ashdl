use crate::spotify::client;
use crate::spotify::song::Song;
use crate::utils::run;
use futures::pin_mut;
use futures_util::TryStreamExt;
use rspotify::model::{PlayableItem, PlaylistId};
use rspotify::prelude::{BaseClient, Id};
use std::process::exit;
use crate::utils::run::error;

/// Converts a playlist into a vector of Song structs.
pub async fn get_playlist_items(client_id: String, client_secret: String, id: String) -> Vec<Song> {
    let mut songs: Vec<Song> = vec![];
    let cli = client::get_client(client_id, client_secret).await;

    let r_id = match PlaylistId::from_id(&*id) {
        Ok(r) => r,
        Err(..) => {
            run::error("Invalid playlist id.", 2);
            exit(2);
        }
    };

    // Checks that the playlist is valid.
    let playlist_check = cli.playlist(&r_id, Option::from(""), None).await;
    match playlist_check {
        Ok(..) => {}
        Err(_) => error("Invalid Playlist Id.", 2),
    }


    let items = cli.playlist_items(&r_id, Option::from(""), None);
    pin_mut!(items);

    while let Some(item) = items.try_next().await.unwrap() {
        let track: Song = match item.track.unwrap() {
            PlayableItem::Track(t) => Song::new(t).await,
            PlayableItem::Episode(..) => Song::blank(),
        };
        if track.title != "" {
            songs.push(track);
        }
    }
    return songs;
}

