use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
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

    pub fn update(conn: &mut PgConnection, user: User) -> usize {
        use crate::schema::users::dsl::*;

        diesel::update(users).set(&user).execute(conn).expect("Failed to update user")
    }

    pub fn destroy(conn: &mut PgConnection, id: i32) -> usize {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(id)))
            .execute(conn)
            .expect("Error deleting posts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::connect;

    #[test]
    fn user_full() {
        
        let conn = &mut connect::establish_connection();

        let user = UserNew::create(conn,
                                          String::from("naokotani"),
                                          String::from("nao@gmail.com"),
                                          String::from("logo.svg"));

        
        assert_eq!(user.username, "naokotani");
        assert_eq!(user.email, "nao@gmail.com");
        assert_eq!(user.logo, "logo.svg");

        let user = User::read(conn, user.id);

        assert_eq!(user.username, "naokotani");
        assert_eq!(user.email, "nao@gmail.com");
        assert_eq!(user.logo, "logo.svg");

        let update = User::update(conn, User {
            id: user.id,
            username: String::from("bob"),
            email: String::from("bill@hotmail.com"),
            logo: String::from("slick.svg"),
        });

        assert_eq!(update, 1);

        let user = User::read(conn, user.id);

        assert_eq!(user.username, "bob");
        assert_eq!(user.email, "bill@hotmail.com");
        assert_eq!(user.logo, "slick.svg");

        let delete = User::destroy(conn, user.id);

        assert_eq!(delete, 1);
    }
}
