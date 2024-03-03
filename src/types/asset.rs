use diesel::prelude::PgConnection;

pub trait Asset {
    fn read(conn: &mut PgConnection, id: i32) -> Self;
    fn destroy(conn: &mut PgConnection, id: i32) -> usize;
    fn update(&self, conn: &mut PgConnection) -> usize;
    //user id refers to the user viewing the content, not the owner
    fn summarize(&self, conn: &mut PgConnection, user_id: i32) -> Summary;
    //user id refers to the user viewing the content, not the owner
    fn paginate(&self, conn: &mut PgConnection, user_id: i32) -> Page;
    fn check_ownership(&self, conn: &mut PgConnection, user_id: i32) -> Ownership;
        
}

pub struct Summary {
    pub display_name: String,
    pub ownership: Ownership,
    pub asset_type: AssetType,
    pub logo: String,
}

pub struct Page {
    pub display_name: String,
    pub ownership: Ownership,
    pub asset_type: AssetType,
    pub logo: String,
    pub extra_images: Vec<String>,
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

#[derive(PartialEq, Debug)]
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

    pub fn images(&self) -> Vec<String> {
        match self {
            Self::Book => Vec::new(),
            Self::Album => Vec::new(),
            Self::Map => Vec::new(),
            Self::MapPack => Vec::new(),
            Self::Stl => Vec::new(),
            Self::TokenPack => Vec::new(),
            Self::Token => Vec::new(),
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
