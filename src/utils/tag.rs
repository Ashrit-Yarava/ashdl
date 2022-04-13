use crate::spotify::Song;
use id3::frame::{Picture, PictureType};
use id3::{Tag, TagLike, Version};

async fn get_image(image_url: String) -> Vec<u8> {
    let response = reqwest::get(image_url)
        .await
        .expect("Something bad happened");
    let image = response.bytes().await.expect("Something bad happened");
    return image.to_vec();
}

pub async fn generate_id3_tag(song: Song, file_name: String) {
    println!("{}", file_name);
    let mut tag = Tag::new();
    let album_name = song.album.to_string();
    tag.set_title(song.title);
    tag.set_album(song.album);
    tag.set_artist(song.artists.join(", "));
    tag.add_frame(Picture {
        mime_type: "image/png".to_string(),
        picture_type: PictureType::CoverFront,
        description: album_name,
        data: get_image(song.img).await,
    });
    tag.write_to_path(file_name, Version::Id3v24)
        .expect("Failed to write id3");
}
