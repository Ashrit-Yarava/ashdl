use crate::spotify::client;
use crate::spotify::song::Song;
use crate::utils::run;
use futures::pin_mut;
use futures_util::TryStreamExt;
use rspotify::model::AlbumId;
use rspotify::prelude::{BaseClient, Id};
use std::process::exit;
use crate::utils::run::error;

/// Converts a playlist into a vector of Song structs.
pub async fn get_album_items(client_id: String, client_secret: String, id: String) -> Vec<Song> {
    let mut songs: Vec<Song> = vec![];
    let cli = client::get_client(client_id, client_secret).await;

    let r_id = match AlbumId::from_id(&*id) {
        Ok(r) => r,
        Err(..) => {
            run::error("Invalid playlist id.", 2);
            exit(2);
        }
    };

    let album_check = cli.album(&r_id).await;

    match album_check {
        Ok(..) => {}
        Err(_) => {
            run::error("Invalid Album Id.", 2);
            exit(2);
        }
    }

    let items = cli.album_track(&r_id);
    pin_mut!(items);

    while let Some(item) = items.try_next().await.unwrap() {

        let track_id = item.id.unwrap();

        let track = match cli.track(&track_id).await {
            Ok(t) => t,
            Err(_) => {
                error("Invalid Track Id", 7);
                exit(7);
            }
        };
        let track: Song = Song::new(track).await;
        if track.title != "" {
            songs.push(track);
        }
    }
    return songs;
}

