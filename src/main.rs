mod spotify;
mod utils;
mod youtube;

#[async_std::main]
async fn main() {
    // Parser functions.
    let (verbose, dlp, sid, ssecret, id) = utils::parser::parse_args();

    if verbose {
        // Program functions.
        utils::screen::clear_screen();
        utils::screen::print_banner();

        // Parser Details
        println!("\nProgram Arguments:");
        utils::parser::print_parser_details();
    }

    // Spotify functions.
    let playlist_items = spotify::playlist::get_playlist_items(sid, ssecret, id).await;

    if verbose {
        println!("\nPlaylist Items:");
        spotify::song::print_songs(&playlist_items);
    }

    // Download songs.
    if verbose {
        println!("\nSong download:");
    }
    youtube::download::download_songs(verbose, dlp, playlist_items).await;
}
