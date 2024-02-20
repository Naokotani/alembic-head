use crate::types::content::{Details, Summary, Dimensions};

pub struct Map {
    pub content: Details,
    pub dimensions: Dimensions,
    file: String,
}

impl Map {
    pub fn new(dimensions: Dimensions, content: Details, file: String) -> Self {
        Map {
            dimensions,
            content,
            file,
        }
    }
}

struct MapPack {
    pub content: Details,
    pub maps: Vec<Map>,
    directory: String,
}

impl MapPack {
    pub fn new(content: Details, maps: Vec<Map>, directory: String) -> Self {
        MapPack {
            content,
            maps,
            directory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log;


    #[test_log::test]
    pub fn map() {
        let content = Details::new(
            1,
            String::from("Cool Book"),
            String::from("thumb.jpg"),
            String::from("so great book"),
            String::from("Frank Hayes"),
        );
        let map = Map::new(
            Dimensions {
                width: 500,
                height: 500,
            },
            content,
            String::from("dungeon.jpg"),
        );

        assert_eq!(map.content.title, "Cool Book");
        assert_eq!(map.content.thumb, "thumb.jpg");
        assert_eq!(map.content.author, "Frank Hayes");
        assert_eq!(map.content.summary, "so great book");
        assert_eq!(
            map.dimensions,
            Dimensions {
                width: 500,
                height: 500
            }
        );
        assert_eq!(map.file, "dungeon.jpg");
    }
}
