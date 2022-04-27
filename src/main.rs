mod spotify;
mod utils;
mod youtube;

#[async_std::main]
async fn main() {
    // Parser functions.
    let (verbose, soundcloud, dlp, sid, ssecret, stype, id) = utils::parser::parse_args();

    if verbose {
        // Program functions.
        utils::screen::clear_screen();
        utils::screen::print_banner();

        // Parser Details
        println!("\nProgram Arguments:");
        utils::parser::print_parser_details();
    }

    // Spotify functions.

    let mut spotify_items: Vec<spotify::song::Song> = vec![];

    if stype == "playlist" {
        spotify_items = spotify::playlist::get_playlist_items(sid, ssecret, id).await;
    } else if stype == "album" {
        spotify_items = spotify::album::get_album_items(sid, ssecret, id).await;
    }

    if verbose {
        println!("\nItems:");
        spotify::song::print_songs(&spotify_items);
    }

    // Download songs.
    if verbose {
        println!("\nSong download:");
    }
    youtube::download::download_songs(verbose, dlp, spotify_items, soundcloud).await;

}
