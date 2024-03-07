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
        pub mod albums;
        pub mod books;
        pub mod maps;
        pub mod stls;
        pub mod tokens;
    }
}

mod schema;

use std::net::TcpListener;
use alembic_head::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080");
    let _port = listener.local_addr().unwrap().port();
    run(listener)?.await
}
