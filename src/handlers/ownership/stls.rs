use crate::schema::user_stls;
use crate::types::asset::Ownership;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
pub struct UserStl {
    user_id: i32,
    stl_id: i32,
}

impl UserStl {
    pub fn new(user_id: i32, stl_id: i32) -> Self {
        UserStl { user_id, stl_id }
    }

    pub fn create(&self, conn: &mut PgConnection) -> usize {
        diesel::insert_into(user_stls::table)
            .values(self)
            .execute(conn)
            .expect("Error saving stl ownership")
    }

    pub fn check_ownership(conn: &mut PgConnection, u_id: i32, b_id: i32) -> Ownership {
        use crate::schema::user_stls::dsl::*;

        let result = user_stls
            .filter(stl_id.eq(b_id))
            .filter(user_id.eq(u_id))
            .execute(conn)
            .expect("Error loading stl ownership");

        match result {
            1 => Ownership::Owned,
            _ => Ownership::Unowned,
        }
    }
}
