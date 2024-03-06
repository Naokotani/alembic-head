use crate::schema::user_maps;
use crate::types::asset::Ownership;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
pub struct UserMap {
    user_id: i32,
    map_id: i32,
}

impl UserMap {
    pub fn new(user_id: i32, map_id: i32) -> Self {
        UserMap { user_id, map_id }
    }

    pub fn create(&self, conn: &mut PgConnection) -> usize {
        diesel::insert_into(user_maps::table)
            .values(self)
            .execute(conn)
            .expect("Error saving Map")
    }

    pub fn check_ownership(conn: &mut PgConnection, u_id: i32, b_id: i32) -> Ownership {
        use crate::schema::user_maps::dsl::*;

        let result = user_maps
            .filter(map_id.eq(b_id))
            .filter(user_id.eq(u_id))
            .execute(conn)
            .expect("Error loading posts");

        match result {
            1 => Ownership::Owned,
            _ => Ownership::Unowned,
        }
    }
}
