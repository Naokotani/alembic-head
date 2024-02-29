use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub logo: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct UserNew {
    pub username: String,
    pub email: String,
    pub logo: String,
}


impl UserNew {
    pub fn create(conn: &mut PgConnection, username: String, email: String, logo: String) -> User {
        let user_new = UserNew {
            username,
            email,
            logo,
        };

        diesel::insert_into(users::table)
            .values(&user_new)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new post")
    }

}

impl User {
    pub fn read(conn: &mut PgConnection, id: i32) -> Self {
        use crate::schema::users::dsl::*;

        let results = users
            .limit(1)
            .select(User::as_select())
            .load(conn)
            .expect("Error loading posts");

        results.into_iter().next().expect("Empty reults vector from user query")
    }
}
