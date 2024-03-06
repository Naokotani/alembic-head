use super::creator::Creator;
use super::ownership::stls::UserStl;
use crate::schema::stls;
use crate::types::asset::{Asset, AssetType, Ownership, Page, Summary};
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Stl {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = stls)]
pub struct StlCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub is_free: bool,
    pub main_image: String,
}

impl StlCreate {
    pub fn new(
        creator_id: i32,
        title: String,
        thumb: String,
        summary: String,
        file: String,
        main_image: String,
        is_free: bool,
    ) -> Self {
        StlCreate {
            creator_id,
            title,
            thumb,
            summary,
            file,
            main_image,
            is_free,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Stl {
        diesel::insert_into(stls::table)
            .values(self)
            .returning(Stl::as_returning())
            .get_result(conn)
            .expect("Error saving Stl")
    }
}

impl Asset for Stl {
    fn read(conn: &mut PgConnection, stl_id: i32) -> Self {
        use crate::schema::stls::dsl::*;

        stls.filter(id.eq(stl_id))
            .select(Stl::as_select())
            .get_result(conn)
            .expect("Error loading posts")
    }

    fn destroy(conn: &mut PgConnection, stl_id: i32) -> usize {
        use crate::schema::stls::dsl::*;

        diesel::delete(stls.filter(id.eq(stl_id)))
            .execute(conn)
            .expect("Error deleting posts")
    }

    fn update(&self, conn: &mut PgConnection) -> usize {
        use crate::schema::stls::dsl::*;

        diesel::update(stls)
            .filter(id.eq(self.id))
            .set(self)
            .execute(conn)
            .expect("Failed to update user")
    }

    fn summarize(&self, conn: &mut PgConnection, user_id: i32) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let asset_type = AssetType::Stl;
        let display_name = creator.get_display_name();
        let ownership = self.check_ownership(conn, user_id);

        Summary {
            display_name,
            ownership,
            asset_type,
            logo: user.logo,
        }
    }

    fn paginate(&self, conn: &mut PgConnection, user_id: i32) -> Page {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let display_name = creator.get_display_name();
        let asset_type = AssetType::Stl;
        let extra_images = Vec::new();
        let ownership = self.check_ownership(conn, user_id);

        Page {
            display_name,
            ownership,
            asset_type,
            logo: user.logo,
            extra_images,
        }
    }

    fn check_ownership(&self, conn: &mut PgConnection, user_id: i32) -> Ownership {
        if self.is_free {
            Ownership::Free
        } else {
            UserStl::check_ownership(conn, user_id, self.id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::connect;
    use crate::handlers::creator::{CreatorNew, Creators};
    use crate::handlers::user::{User, UserNew};
    use crate::types::user::DisplayName;

    #[test]
    fn stl_full() {
        let conn = &mut connect::establish_connection();

        let user = UserNew::create(
            conn,
            String::from("naokotani"),
            String::from("nao@gmail.com"),
            String::from("logo.svg"),
        );

        assert_eq!(user.username, "naokotani");

        let creator = CreatorNew::create(
            conn,
            user.id,
            Some(String::from("Chris")),
            Some(String::from("Hughes")),
            Some(String::from("naokotani")),
            Some(String::from("Random House")),
            DisplayName::Name,
        );

        let mut stl = StlCreate::new(
            creator.id,
            String::from("Dungeons and Dragons"),
            String::from("thumb.jpg"),
            String::from("What a stl!"),
            String::from("file.pdf"),
            String::from("image.jpg"),
            false,
        )
        .create(conn);

        assert_eq!(stl.title, "Dungeons and Dragons");
        assert_eq!(stl.thumb, "thumb.jpg");
        assert_eq!(stl.summary, "What a stl!");
        assert_eq!(stl.file, "file.pdf");
        assert_eq!(stl.main_image, "image.jpg");
        assert_eq!(stl.is_free, false);

        let summary = stl.summarize(conn, user.id);

        assert_eq!(summary.display_name, "Chris Hughes");
        assert_eq!(summary.logo, "logo.svg");

        let page = stl.paginate(conn, user.id);

        assert_eq!(page.display_name, "Chris Hughes");
        assert_eq!(page.logo, "logo.svg");
        assert_eq!(page.ownership, Ownership::Unowned);

        stl.title = String::from("For Whom the Bell Tolls");

        let update = stl.update(conn);

        assert_eq!(update, 1);

        let stl = Stl::read(conn, stl.id);

        assert_eq!(stl.title, "For Whom the Bell Tolls");
        assert_eq!(stl.summary, "What a stl!");

        let delete = Stl::destroy(conn, stl.id);

        assert_eq!(delete, 1);

        Creators::destroy(conn, creator.id);
        User::destroy(conn, user.id);
    }
}
