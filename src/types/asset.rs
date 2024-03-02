use crate::handlers::creator::Creator;
use diesel::prelude::PgConnection;

pub trait Asset {
    fn read(conn: &mut PgConnection, id: i32) -> Self;
    fn destroy(conn: &mut PgConnection, id: i32) -> usize;
    fn update(&self, conn: &mut PgConnection) -> usize;
    fn summarize(&self,
                 conn: &mut PgConnection,
                 _user_id: i32,
                 creator_id: i32,
                 asset_type: AssetType,
                 is_free: bool
    ) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, creator_id);
        let display_name = creator.get_display_name();
        let extra_images = asset_type.images();
        let ownership = if is_free {
            Ownership::Free
        } else {
            Ownership::Unowned
        };

        Summary {
            display_name,
            ownership,
            asset_type,
            logo: user.logo,
        }
    }

    fn paginate(
        &self,
        conn: &mut PgConnection,
        _user_id: i32,
        creator_id: i32,
        asset_type: AssetType,
        is_free: bool,
    ) -> Page {
        let (creator, user) = Creator::creator_with_user(conn, creator_id);
        let display_name = creator.get_display_name();
        let extra_images = asset_type.images();
        let ownership = if is_free {
            Ownership::Free
        } else {
            Ownership::Unowned
        };

        Page {
            display_name,
            ownership,
            asset_type,
            logo: user.logo,
            extra_images,
        }
    }
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
