use aspotify::{Client, ClientCredentials};

pub async fn generate_client(client_id: String, client_secret: String) -> Client {
    let credentials = ClientCredentials {
        id: client_id,
        secret: client_secret
    };
    return Client::new(credentials);
}
