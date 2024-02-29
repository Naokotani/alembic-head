use crate::schema::creators::{self};
use crate::types::user::DisplayName;
use diesel::prelude::*;

pub struct Creator {
    pub creator_id: i32,
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub other_name: String,
    pub publisher: String,
    pub default_name: DisplayName,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = creators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreatorQuery {
    creator_id: i32,
    user_id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
    other_name: Option<String>,
    publisher: Option<String>,
    default_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = creators)]
pub struct CreatorNew {
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub other_name: Option<String>,
    pub publisher: Option<String>,
    pub default_name: String,
}

impl Creator {
    pub fn new(creator: CreatorQuery) -> Self {

        let user_id = creator.user_id;

        let first_name = creator.first_name.unwrap_or_else(|| String::new());

        let last_name = creator.last_name.unwrap_or_else(|| String::new());

        let other_name = creator.other_name.unwrap_or_else(|| String::new());

        let publisher = creator.publisher.unwrap_or_else(|| String::new());
    
        let default_name = DisplayName::retreieve(&creator.default_name);

        Creator {
            creator_id: creator.creator_id,
            user_id,
            first_name,
            last_name,
            other_name,
            publisher,
            default_name,
        }
        
    }
}

impl CreatorNew {
    pub fn create(
        conn: &mut PgConnection,
        user_id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        other_name: Option<String>,
        publisher: Option<String>,
        name: DisplayName,
    ) -> Creator {
        let name = String::from(name.store());
        let creator_new = CreatorNew {
            user_id,
            first_name,
            last_name,
            other_name,
            publisher,
            default_name: name,
        };

        let creator = diesel::insert_into(creators::table)
            .values(&creator_new)
            .returning(CreatorQuery::as_returning())
            .get_result(conn)
            .expect("Error saving new post");

        Creator::new(creator)
    }
}

impl CreatorQuery {
    pub fn read(conn: &mut PgConnection, id: i32) -> Creator {

        use crate::schema::creators::dsl::*;
        let results: Vec<CreatorQuery> = creators
            .filter(creator_id.eq(id))
            .limit(1)
            .select(CreatorQuery::as_select())
            .load(conn)
            .expect("Error loading creators");

        let result = results.into_iter().next();
        
        Creator::new(result.expect("Query filed for creator"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::connect;

    #[test]
    fn creator_full() {
        let conn = &mut connect::establish_connection();


        let creator = CreatorNew::create(conn,
                                         1,
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

        
    }

}
