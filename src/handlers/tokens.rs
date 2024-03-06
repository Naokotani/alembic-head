use super::creator::Creator;
use super::ownership::tokens::UserToken;
use crate::schema::token_packs;
use crate::schema::tokens;
use crate::types::asset::{Asset, AssetType, Ownership, Page, Summary};
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: i32,
    pub creator_id: i32,
    pub token_pack_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub is_free: bool,
    pub main_image: String,
}

pub struct TokenPack {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
    pub tokens: Vec<Token>,
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct TokenCreate {
    pub creator_id: i32,
    pub token_pack_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = token_packs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenPackQuery {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = token_packs)]
pub struct TokenPackCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
}

impl TokenPackCreate {
    pub fn new(
        creator_id: i32,
        title: String,
        thumb: String,
        summary: String,
        directory: String,
        is_free: bool,
        main_image: String,
    ) -> Self {
        TokenPackCreate {
            creator_id,
            title,
            thumb,
            summary,
            directory,
            is_free,
            main_image,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> TokenPackQuery {
        diesel::insert_into(token_packs::table)
            .values(self)
            .returning(TokenPackQuery::as_returning())
            .get_result(conn)
            .expect("Error saving Token Pack")
    }
}

impl TokenCreate {
    pub fn new(
        creator_id: i32,
        token_pack_id: i32,
        title: String,
        thumb: String,
        summary: String,
        height: Option<i32>,
        width: Option<i32>,
        directory: &str,
        main_image: String,
        is_free: bool,
    ) -> Self {
        let slug = title.to_lowercase().trim().replace(" ", "-");
        let file = format!("{}/{}", directory, slug);
        TokenCreate {
            creator_id,
            token_pack_id,
            thumb,
            title,
            summary,
            height,
            width,
            file,
            main_image,
            is_free,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Token {
        diesel::insert_into(tokens::table)
            .values(self)
            .returning(Token::as_returning())
            .get_result(conn)
            .expect("Error saving token")
    }
}

impl Asset for TokenPack {
    fn read(conn: &mut PgConnection, pack_id: i32) -> TokenPack {
        use crate::schema::tokens::dsl::*;

        let token_pack = get_token_pack(conn, pack_id);

        let token = tokens
            .filter(token_pack_id.eq(pack_id))
            .select(Token::as_select())
            .get_results(conn)
            .expect("Error loading posts");

        TokenPack {
            id: token_pack.id,
            creator_id: token_pack.creator_id,
            title: token_pack.title,
            thumb: token_pack.thumb,
            summary: token_pack.summary,
            directory: token_pack.directory,
            is_free: token_pack.is_free,
            main_image: token_pack.main_image,
            tokens: token,
        }
    }

    fn destroy(conn: &mut PgConnection, pack_id: i32) -> usize {
        use crate::schema::tokens::dsl::*;

        let changes = diesel::delete(tokens.filter(token_pack_id.eq(pack_id)))
            .execute(conn)
            .expect("Error deleting posts");

        changes + destroy_token_pack(conn, pack_id)
    }

    fn update(&self, conn: &mut PgConnection) -> usize {
        use crate::schema::token_packs::dsl::*;

        let token_pack = TokenPackQuery {
            id: self.id,
            creator_id: self.creator_id.to_owned(),
            title: self.title.to_owned(),
            thumb: self.thumb.to_owned(),
            summary: self.summary.to_owned(),
            directory: self.directory.to_owned(),
            is_free: self.is_free,
            main_image: self.main_image.to_owned(),
        };

        let changes = diesel::update(token_packs)
            .filter(id.eq(token_pack.id))
            .set(token_pack)
            .execute(conn)
            .expect("Failed to update user");

        update_tokens(conn, &self.tokens) + changes
    }

    fn summarize(&self, conn: &mut PgConnection, user_id: i32) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let asset_type = AssetType::Token;
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
        let asset_type = AssetType::Token;
        let extra_images = asset_type.images();
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
            UserToken::check_ownership(conn, user_id, self.id)
        }
    }
}

fn get_token_pack(conn: &mut PgConnection, pack_id: i32) -> TokenPackQuery {
    use crate::schema::token_packs::dsl::*;

    token_packs
        .filter(id.eq(pack_id))
        .select(TokenPackQuery::as_select())
        .get_result(conn)
        .expect("Error loading token pack")
}

fn destroy_token_pack(conn: &mut PgConnection, pack_id: i32) -> usize {
    use crate::schema::token_packs::dsl::*;

    diesel::delete(token_packs.filter(id.eq(pack_id)))
        .execute(conn)
        .expect("Error deleting token_packs")
}

fn update_tokens(conn: &mut PgConnection, tokens_vec: &Vec<Token>) -> usize {
    use crate::schema::tokens::dsl::*;

    let mut changes: usize = 0;
    for token in tokens_vec {
        let result = diesel::update(tokens)
            .filter(id.eq(token.id))
            .set(token)
            .execute(conn)
            .expect("Failed to update user");
        changes += result;
    }
    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::connect;
    use crate::handlers::creator::{CreatorNew, Creators};
    use crate::handlers::user::{User, UserNew};
    use crate::types::user::DisplayName;

    // #[test]
    // fn token() {
    //     let conn = &mut connect::establish_connection();

    //     let token_pack = TokenPackCreate::new(
    //         1,
    //         String::from("Epic Fights"),
    //         String::from("thumb.jpg"),
    //         String::from("Lots of great locations"),
    //         String::from("directory"),
    //         false,
    //         String::from("image.jpg"),
    //     )
    //     .create(conn);

    //     vec![TokenCreate::new(
    //         1,
    //         token_pack.id,
    //         String::from("Windy Glade"),
    //         String::from("thumb.jpg"),
    //         String::from("What a fight area!"),
    //         Some(450),
    //         Some(450),
    //         &token_pack.directory,
    //         String::from("image.jpg"),
    //         false,
    //     )
    //     .create(conn)];

    //     let mut token_pack = TokenPack::read(conn, token_pack.id);

    //     assert_eq!(token_pack.tokens[0].title, "Windy Glade");

    //     let page = token_pack.paginate(conn, 2);

    //     assert_eq!(page.display_name, "Chris Hughes");
    //     assert_eq!(page.asset_type, AssetType::Token);

    //     let summary = token_pack.summarize(conn, 2);

    //     assert_eq!(summary.display_name, "Chris Hughes");
    //     assert_eq!(summary.asset_type, AssetType::Token);

    //     token_pack.is_free = true;
    //     token_pack.update(conn);

    //     let token_pack = TokenPack::read(conn, token_pack.id);
    //     let summary = token_pack.summarize(conn, 1);

    //     assert_eq!(summary.ownership, Ownership::Free);

    //     let delete = TokenPack::destroy(conn, token_pack.id);

    //     assert_eq!(delete, 2);
    // }

    #[test]
    fn token_full() {
        let conn = &mut connect::establish_connection();

        let user = UserNew::create(
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

        let token_pack = TokenPackCreate::new(
            creator.id,
            String::from("Epic Fights"),
            String::from("thumb.jpg"),
            String::from("Lots of great locations"),
            String::from("directory"),
            false,
            String::from("image.jpg"),
        )
        .create(conn);

        vec![TokenCreate::new(
            creator.id,
            token_pack.id,
            String::from("Windy Glade"),
            String::from("thumb.jpg"),
            String::from("What a fight area!"),
            Some(450),
            Some(450),
            &token_pack.directory,
            String::from("image.jpg"),
            false,
        )
        .create(conn)];

        let mut token_pack = TokenPack::read(conn, token_pack.id);

        assert_eq!(token_pack.tokens[0].title, "Windy Glade");

        let page = token_pack.paginate(conn, user.id);

        assert_eq!(page.display_name, "Chris Hughes");
        assert_eq!(page.asset_type, AssetType::Token);

        let summary = token_pack.summarize(conn, user.id);

        assert_eq!(summary.display_name, "Chris Hughes");
        assert_eq!(summary.asset_type, AssetType::Token);

        let update_names = Creators::update_names(
            conn,
            creator.id,
            None,
            None,
            Some(creator.other_name),
            Some(String::from("Random House")),
            DisplayName::OtherPublisher,
        );

        assert_eq!(update_names, 1);

        token_pack.is_free = true;
        token_pack.update(conn);

        let token_pack = TokenPack::read(conn, token_pack.id);
        let summary = token_pack.summarize(conn, user.id);

        assert_eq!(summary.display_name, "naokotani publisher: Random House");

        assert_eq!(summary.ownership, Ownership::Free);

        let delete = TokenPack::destroy(conn, token_pack.id);

        assert_eq!(delete, 2);

        Creators::destroy(conn, creator.id);
        User::destroy(conn, user.id);
    }
}
