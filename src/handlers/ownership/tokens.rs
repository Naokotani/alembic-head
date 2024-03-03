use crate::schema::user_tokens;
use diesel::prelude::*;
use crate::types::asset::Ownership;

#[derive(Queryable, Selectable)]
#[derive(Insertable)]
pub struct UserToken {
    user_id: i32,
    token_id: i32,
}

impl UserToken {
    pub fn new(user_id: i32, token_id: i32) -> Self {
        UserToken {
            user_id,
            token_id,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> usize {
        diesel::insert_into(user_tokens::table)
            .values(self)
            .execute(conn)
            .expect("Error saving stl ownership")
    }
    
    pub fn check_ownership(conn: &mut PgConnection, u_id: i32, b_id: i32) -> Ownership {

        use crate::schema::user_tokens::dsl::*;

        let result = user_tokens
            .filter(token_id.eq(b_id))
            .filter(user_id.eq(u_id))
            .execute(conn)
            .expect("Error loading stl ownership");

        match result {
            1 => Ownership::Owned,
            _ => Ownership::Unowned,
    }
        
    }
}
