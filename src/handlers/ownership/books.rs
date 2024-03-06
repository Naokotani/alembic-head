use crate::schema::user_books;
use crate::types::asset::Ownership;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
pub struct UserBook {
    user_id: i32,
    book_id: i32,
}

impl UserBook {
    pub fn new(user_id: i32, book_id: i32) -> Self {
        UserBook { user_id, book_id }
    }

    pub fn create(&self, conn: &mut PgConnection) -> usize {
        diesel::insert_into(user_books::table)
            .values(self)
            .execute(conn)
            .expect("Error saving Book")
    }

    pub fn check_ownership(conn: &mut PgConnection, u_id: i32, b_id: i32) -> Ownership {
        use crate::schema::user_books::dsl::*;

        let result = user_books
            .filter(book_id.eq(b_id))
            .filter(user_id.eq(u_id))
            .execute(conn)
            .expect("Error loading posts");

        match result {
            1 => Ownership::Owned,
            _ => Ownership::Unowned,
        }
    }
}
