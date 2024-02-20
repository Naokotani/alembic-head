use tracing::{Level, event};
use crate::types::book::Book;
use crate::types::token::TokenPack;
use crate::types::map::Map;
use crate::types::album::Album;

struct User {
    id: i32,
    name: String,
    email: String,
    owned: Products,
}

impl User {
    pub fn new(id: i32, name: String, email: String) -> Self {
        event!(Level::INFO, "Creating new user id: {} name: {}", id, name);
        let books: Vec<Book> = Vec::new();
        let token_packs: Vec<TokenPack> = Vec::new();
        let maps: Vec<Map> = Vec::new();
        let albums: Vec<Album> = Vec::new();

        let owned = Products {
            books,
            token_packs,
            maps,
            albums,
        };

        User {
            id,
            name,
            email,
            owned,
        }
    }
}

struct Creator {
    id: i32,
    products: Products,
}

impl Creator {
    pub fn new(id: i32) -> Self {
        let books: Vec<Book> = Vec::new();
        let token_packs: Vec<TokenPack> = Vec::new();
        let maps: Vec<Map> = Vec::new();
        let albums: Vec<Album> = Vec::new();

        let products = Products {
            books,
            token_packs,
            maps,
            albums,
        };

        Creator {
            id,
            products,
        }
        

    }
}

struct Products {
    books: Vec<Book>,
    token_packs: Vec<TokenPack>,
    maps: Vec<Map>,
    albums: Vec<Album>,

}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log;

    #[test_log::test]
    pub fn new_user() {
        let user = User::new(1, String::from("Chris"), String::from("chris@chris.com"));
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Chris");
    }
    
}
