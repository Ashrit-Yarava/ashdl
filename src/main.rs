use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    
    // The output directory for the song files.
    #[clap(short, long)]
    output: Option<String>,

    // The playlist to get the songs from.
    playlist_url: Option<String>

}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.output.as_deref());
    println!("{:?}", args.playlist_url.as_deref());

}
