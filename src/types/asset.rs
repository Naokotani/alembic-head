pub trait Asset {
    fn read(id: i32) -> Self;
    fn destroy(id: i32) -> i32;
    fn getDisplayName(&self) -> String;
    fn summarize(&self) -> Summary;
    fn paginate(user_id: i32) -> Page;
}

struct Summary {
    title: String,
    thumb: String,
    display_name: String,
    ownership: Ownership,
    summary: String,
    asset_type: AssetType,
    logo: String,
}

struct Page {
    title: String,
    thumb: String,
    display_name: String,
    ownership: Ownership,
    summary: String,
    asset_type: AssetType,
    logo: String,
    main_image: String,
    extra_images: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq, Debug)]
pub enum Ownership {
    Owned,
    Free,
    Unowned,
}

pub enum AssetType {
    Book,
    Album,
    Map,
    MapPack,
    Stl,
    TokenPack,
    Token,
}

impl AssetType {
    pub fn retrieve(str: &str) -> Self {
        match str {
            "book" => Self::Book,
            "album" => Self::Album,
            "map" => Self::Map,
            "map_pack" => Self::MapPack,
            "stl" => Self::Stl,
            "token_pack" => Self::TokenPack,
            "token" => Self::Token,
            _ => panic!("invalid ownership"),
        }
    }

    pub fn store(&self) -> &str {
        match self {
            Self::Book => "book",
            Self::Album => "album",
            Self::Map => "map",
            Self::MapPack => "map_pack",
            Self::Stl => "stl", 
            Self::TokenPack => "token_pack",
            Self::Token => "token",
        }
    }
}

impl Ownership {
    pub fn retrieve(str: &str) -> Self {
        match str {
            "owned" => Self::Owned,
            "unowned" => Self::Unowned,
            "free" => Self::Free,
            _ => panic!("invalid ownership"),
        }
    }

    pub fn store(&self) -> &str {
        match self {
            Self::Owned => "owned",
            Self::Free => "free",
            Self::Unowned => "unowned",
        }
    }
}
