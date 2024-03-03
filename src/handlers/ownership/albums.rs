use crate::schema::user_albums;
use diesel::prelude::*;
use crate::types::asset::Ownership;

#[derive(Queryable, Selectable)]
#[derive(Insertable)]
pub struct UserAlbum {
    user_id: i32,
    album_id: i32,
}

impl UserAlbum {
    pub fn new(user_id: i32, album_id: i32) -> Self {
        UserAlbum {
            user_id,
            album_id,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> usize {
        diesel::insert_into(user_albums::table)
            .values(self)
            .execute(conn)
            .expect("Error saving Album")
    }
    
    pub fn check_ownership(conn: &mut PgConnection, u_id: i32, b_id: i32) -> Ownership {

        use crate::schema::user_albums::dsl::*;

        let result = user_albums
            .filter(album_id.eq(b_id))
            .filter(user_id.eq(u_id))
            .execute(conn)
            .expect("Error loading posts");

        match result {
            1 => Ownership::Owned,
            _ => Ownership::Unowned,
    }
        
    }
}
