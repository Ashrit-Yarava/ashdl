use crate::utils::run;
use rspotify::{ClientCredsSpotify, Credentials};

/// Returns a spotify client after handling possible errors.
pub async fn get_client(client_id: String, client_secret: String) -> ClientCredsSpotify {
    let credentials = Credentials {
        id: client_id,
        secret: Option::from(client_secret),
    };
    let mut spotify = ClientCredsSpotify::new(credentials);
    return match spotify.request_token().await {
        Ok(..) => spotify,
        Err(..) => {
            run::error("Failed to authorize client.", 1);
            spotify
        }
    };
}

