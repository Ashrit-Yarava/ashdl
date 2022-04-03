use rspotify::{ClientCredsSpotify, Credentials};

pub async fn get_client(client_id: String, client_secret: String) -> ClientCredsSpotify {
    let credentials = Credentials {
        id: client_id,
        secret: Option::from(client_secret)
    };
    let mut spotify = ClientCredsSpotify::new(credentials);
    spotify.request_token().await.unwrap();

    return spotify;
}