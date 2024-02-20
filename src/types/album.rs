use crate::types::content::{Details, Summary, Content};

pub struct Album {
    pub content: Details,
    pub tracks: Vec<Track>,
    directory: String,
}

impl Album {
    pub fn new(content: Details, tracks: Vec<Track>, directory: String) -> Self {
        Album {
            content,
            tracks,
            directory,
        }
    }
}

pub struct Track {
    pub content: Details,
    file: String,
}

impl Track {
    pub fn new(content: Details, file: String) -> Self {
        Track { content, file }
    }
}

mod tests {
    use super::*;
    use test_log;


    #[test_log::test]
    pub fn album() {
        let content = Details::new(
            1,
            String::from("Cool Book"),
            String::from("thumb.jpg"),
            String::from("so great book"),
            String::from("Frank Hayes"),
        );

        let content_track = Details::new(
            1,
            String::from("great track"),
            String::from("thumb.jpg"),
            String::from("great track"),
            String::from("Frank Hayes"),
        );
        let track = Track::new(content_track, String::from("track.mp3"));
        let tracks = vec![track];
        let album = Album::new(content, tracks, String::from("/album"));

        assert_eq!(album.content.title, "Cool Book");
        assert_eq!(album.content.thumb, "thumb.jpg");
        assert_eq!(album.content.author, "Frank Hayes");
        assert_eq!(album.content.summary, "so great book");
        assert_eq!(album.tracks[0].file, "track.mp3");
        assert_eq!(album.tracks[0].content.title, "great track");
        assert_eq!(album.directory, "/album");
    }

}
