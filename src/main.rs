#![allow(dead_code)]
mod types {
    pub mod asset;
    pub mod user;
}

mod handlers {
    pub mod connect;
    pub mod book;
    pub mod creator;
    pub mod user;
    pub mod map;
    pub mod stl;
    pub mod tokens;
    pub mod album;
}

use crate::handlers::user;
use crate::handlers::connect;


mod schema;

fn main() {
    // let conn = &mut connect::establish_connection();

    // let user = user::UserNew::create(conn,
    //                                       String::from("foo"),
    //                                       String::from("email"),
    //                                       String::from("logo"));

    // println!("username: {}, email: {}, logo: {}", user.username, user.email, user.logo);

    // let user = user::User::read(conn, 1);

    // println!("id: {} username: {}, email: {}, logo: {}", user.id, user.username, user.email, user.logo);

}
