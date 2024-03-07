use crate::handlers::user::User;
use crate::schema::{creators, users};
use crate::types::user::DisplayName;
use diesel::prelude::*;

#[derive(Debug)]
pub struct Creator {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub other_name: String,
    pub publisher: String,
    pub default_name: DisplayName,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = creators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Creators {
    id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
    other_name: Option<String>,
    publisher: Option<String>,
    default_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = creators)]
pub struct CreatorNew {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub other_name: Option<String>,
    pub publisher: Option<String>,
    pub default_name: String,
}

impl Creator {
    pub fn new(creator: Creators) -> Self {
        let id = creator.id;

        let first_name = creator.first_name.unwrap_or_default();
        let last_name = creator.last_name.unwrap_or_default();
        let other_name = creator.other_name.unwrap_or_default();
        let publisher = creator.publisher.unwrap_or_default();
        let default_name = DisplayName::retreieve(&creator.default_name);

        Creator {
            id,
            first_name,
            last_name,
            other_name,
            publisher,
            default_name,
        }
    }

    pub fn creator_with_user(conn: &mut PgConnection, user_id: i32) -> (Self, User) {
        let (creators, user) = creators::table
            .inner_join(users::table)
            .filter(users::id.eq(user_id))
            .select((Creators::as_select(), User::as_select()))
            .get_result::<(Creators, User)>(conn)
            .expect("Failed to get user/creator");

        (Creator::new(creators), user)
    }

    pub fn get_display_name(&self) -> String {
        match self.default_name {
            DisplayName::Name => format!("{} {}", self.first_name, self.last_name),
            DisplayName::Other => self.other_name.to_string(),
            DisplayName::NamePublisher => {
                format!(
                    "{} {} publisher: {}",
                    self.first_name, self.last_name, self.publisher
                )
            }
            DisplayName::OtherPublisher => {
                format!("{} publisher: {}", self.other_name, self.publisher)
            }
        }
    }
}

impl CreatorNew {
    pub fn create(
        conn: &mut PgConnection,
        id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        other_name: Option<String>,
        publisher: Option<String>,
        name: DisplayName,
    ) -> Creator {
        let name = String::from(name.store());
        let creator_new = CreatorNew {
            id,
            first_name,
            last_name,
            other_name,
            publisher,
            default_name: name,
        };

        let creator = diesel::insert_into(creators::table)
            .values(&creator_new)
            .returning(Creators::as_returning())
            .get_result(conn)
            .expect("Error saving new post");

        Creator::new(creator)
    }
}

impl Creators {
    pub fn read(conn: &mut PgConnection, creator_id: i32) -> Creator {
        use crate::schema::creators::dsl::*;
        let result = creators
            .filter(id.eq(creator_id))
            .select(Creators::as_select())
            .get_result(conn)
            .expect("Error loading creators");

        Creator::new(result)
    }

    pub fn update_names(
        conn: &mut PgConnection,
        creator_id: i32,
        first: Option<String>,
        last: Option<String>,
        other: Option<String>,
        publish: Option<String>,
        default: DisplayName,
    ) -> usize {
        use crate::schema::creators::dsl::*;

        diesel::update(creators)
            .filter(id.eq(creator_id))
            .set((
                first_name.eq(first),
                last_name.eq(last),
                other_name.eq(other),
                publisher.eq(publish),
                default_name.eq(default.store()),
            ))
            .execute(conn)
            .expect("Creator update failed")
    }

    pub fn destroy(conn: &mut PgConnection, creator_id: i32) -> usize {
        use crate::schema::creators::dsl::*;

        diesel::delete(creators.filter(id.eq(creator_id)))
            .execute(conn)
            .expect("Error deleting posts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::connect;
    use crate::handlers::user;

    #[test]
    fn creator_full() {
        let conn = &mut connect::establish_connection();

        let user = user::UserNew::create(
            conn,
            String::from("naokotani"),
            String::from("nao@gmail.com"),
            String::from("logo.svg"),
        );

        let creator = CreatorNew::create(
            conn,
            user.id,
            Some(String::from("Chris")),
            Some(String::from("Hughes")),
            Some(String::from("naokotani")),
            Some(String::from("Random House")),
            DisplayName::Name,
        );

        assert_eq!(creator.first_name, "Chris");
        assert_eq!(creator.last_name, "Hughes");
        assert_eq!(creator.other_name, "naokotani");
        assert_eq!(creator.publisher, "Random House");
        assert_eq!(creator.default_name, DisplayName::Name);

        let creator = Creators::read(conn, creator.id);

        assert_eq!(creator.first_name, "Chris");
        assert_eq!(creator.last_name, "Hughes");
        assert_eq!(creator.other_name, "naokotani");
        assert_eq!(creator.publisher, "Random House");

        let update = Creators::update_names(
            conn,
            creator.id,
            None,
            None,
            Some(String::from("Galator")),
            None,
            DisplayName::Other,
        );

        assert_eq!(update, 1);

        let creator = Creators::read(conn, creator.id);

        assert_eq!(creator.first_name, "");
        assert_eq!(creator.last_name, "");
        assert_eq!(creator.other_name, "Galator");
        assert_eq!(creator.publisher, "");

        let delete = Creators::destroy(conn, creator.id);

        assert_eq!(delete, 1);

        let conn = &mut connect::establish_connection();

        let delete = user::User::destroy(conn, user.id);

        assert_eq!(delete, 1);
    }
}
