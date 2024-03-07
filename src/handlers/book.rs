use super::creator::Creator;
use super::ownership::books::UserBook;
use crate::schema::books;
use crate::types::asset::{Asset, AssetType, Ownership, Page, Summary};
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub pages: i32,
    pub main_image: String,
    pub is_free: bool,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct BookCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub pages: i32,
    pub main_image: String,
    pub is_free: bool,
}

impl BookCreate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        creator_id: i32,
        title: String,
        thumb: String,
        summary: String,
        file: String,
        pages: i32,
        main_image: String,
        is_free: bool,
    ) -> Self {
        BookCreate {
            creator_id,
            title,
            thumb,
            summary,
            file,
            pages,
            main_image,
            is_free,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Book {
        diesel::insert_into(books::table)
            .values(self)
            .returning(Book::as_returning())
            .get_result(conn)
            .expect("Error saving Book")
    }
}

impl Asset for Book {
    fn read(conn: &mut PgConnection, book_id: i32) -> Self {
        use crate::schema::books::dsl::*;

        books
            .filter(id.eq(book_id))
            .select(Book::as_select())
            .get_result(conn)
            .expect("Error loading posts")
    }

    fn destroy(conn: &mut PgConnection, book_id: i32) -> usize {
        use crate::schema::books::dsl::*;

        diesel::delete(books.filter(id.eq(book_id)))
            .execute(conn)
            .expect("Error deleting posts")
    }

    fn update(&self, conn: &mut PgConnection) -> usize {
        use crate::schema::books::dsl::*;

        diesel::update(books)
            .filter(id.eq(self.id))
            .set(self)
            .execute(conn)
            .expect("Failed to update user")
    }

    fn summarize(&self, conn: &mut PgConnection, user_id: i32) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let asset_type = AssetType::Book;
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
        let asset_type = AssetType::Book;
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
            UserBook::check_ownership(conn, user_id, self.id)
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
    fn book_full() {
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

        let mut book = BookCreate::new(
            creator.id,
            String::from("Dungeons and Dragons"),
            String::from("thumb.jpg"),
            String::from("What a book!"),
            String::from("file.pdf"),
            385,
            String::from("image.jpg"),
            false,
        )
        .create(conn);

        assert_eq!(book.title, "Dungeons and Dragons");
        assert_eq!(book.thumb, "thumb.jpg");
        assert_eq!(book.summary, "What a book!");
        assert_eq!(book.file, "file.pdf");
        assert_eq!(book.pages, 385);
        assert_eq!(book.main_image, "image.jpg");
        assert_eq!(book.is_free, false);

        let summary = book.summarize(conn, user.id);

        assert_eq!(summary.display_name, "Chris Hughes");
        assert_eq!(summary.logo, "logo.svg");

        let page = book.paginate(conn, user.id);

        assert_eq!(page.display_name, "Chris Hughes");
        assert_eq!(page.logo, "logo.svg");
        assert_eq!(page.ownership, Ownership::Unowned);

        book.title = String::from("For Whom the Bell Tolls");

        let update = book.update(conn);

        assert_eq!(update, 1);

        let book = Book::read(conn, book.id);

        assert_eq!(book.title, "For Whom the Bell Tolls");
        assert_eq!(book.summary, "What a book!");

        let delete = Book::destroy(conn, book.id);

        assert_eq!(delete, 1);

        Creators::destroy(conn, creator.id);
        User::destroy(conn, user.id);
    }
}
