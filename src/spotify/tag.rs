use id3::frame::{Picture, PictureType};
use id3::{Tag, TagLike, Version};
use crate::spotify::song::Song;

pub async fn generate_id3_tag(song: &Song) {
    let mut tag = Tag::new();
    tag.set_album(song.album.to_string());
    tag.set_artist(song.artists_str());
    tag.set_title(song.title.to_string());
    tag.add_frame(Picture {
        mime_type: "image/png".to_string(),
        picture_type: PictureType::CoverFront,
        description: song.album.to_string(),
        data: song.img.to_vec()
    });
    match tag.write_to_path(song.get_filename(), Version::Id3v24) {
        Ok(_) => {}
        Err(_) => println!("Error with song id3: {}", song.title),
    };
}