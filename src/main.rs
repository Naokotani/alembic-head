#![allow(dead_code)]
mod types {
    pub mod asset;
    pub mod user;
}

mod handlers {
    pub mod album;
    pub mod book;
    pub mod connect;
    pub mod creator;
    pub mod map;
    pub mod stl;
    pub mod tokens;
    pub mod user;
    pub mod ownership {
        pub mod books;
        pub mod albums;
        pub mod maps;
        pub mod stls;
        pub mod tokens;
    }
}

mod schema;

use crate::handlers::user::UserNew;
use crate::handlers::creator::CreatorNew;
use crate::types::user::DisplayName;
use crate::handlers::connect;

fn main() {
        let conn = &mut connect::establish_connection();

        let user1 = UserNew::create(
            conn,
            String::from("naokotani"),
            String::from("nao@gmail.com"),
            String::from("logo.svg"),
        );

        CreatorNew::create(
            conn,
            user1.id,
            Some(String::from("Chris")),
            Some(String::from("Hughes")),
            Some(String::from("naokotani")),
            Some(String::from("Random House")),
            DisplayName::Name,
        );

        let user2 = UserNew::create(
            conn,
            String::from("Galator"),
            String::from("gal@gmail.com"),
            String::from("logo.svg"),
        );

        CreatorNew::create(
            conn,
            user2.id,
            Some(String::from("Tommy")),
            Some(String::from("Gendron")),
            Some(String::from("Galator")),
            Some(String::from("Random House")),
            DisplayName::Name,
        );
}
