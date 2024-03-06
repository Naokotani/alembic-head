use super::creator::Creator;
use super::ownership::albums::UserAlbum;
use crate::schema::albums;
use crate::schema::tracks;
use crate::types::asset::{Asset, AssetType, Ownership, Page, Summary};
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Track {
    pub id: i32,
    pub creator_id: i32,
    pub album_id: i32,
    pub title: String,
    pub file: String,
    pub main_image: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = tracks)]
pub struct TrackCreate {
    pub creator_id: i32,
    pub album_id: i32,
    pub title: String,
    pub file: String,
    pub main_image: String,
}

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AlbumQuery {
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
#[diesel(table_name = albums)]
pub struct AlbumCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
}

pub struct Album {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
    pub tracks: Vec<Track>,
}

impl AlbumCreate {
    pub fn new(
        creator_id: i32,
        title: String,
        thumb: String,
        summary: String,
        directory: String,
        is_free: bool,
        main_image: String,
    ) -> Self {
        AlbumCreate {
            creator_id,
            title,
            thumb,
            summary,
            directory,
            is_free,
            main_image,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> AlbumQuery {
        diesel::insert_into(albums::table)
            .values(self)
            .returning(AlbumQuery::as_returning())
            .get_result(conn)
            .expect("Error saving Book")
    }
}

impl TrackCreate {
    pub fn new(
        creator_id: i32,
        album_id: i32,
        title: String,
        directory: &str,
        main_image: String,
    ) -> Self {
        let slug = title.to_lowercase().trim().replace(" ", "-");
        let file = format!("{}/{}", directory, slug);
        TrackCreate {
            creator_id,
            album_id,
            title,
            file,
            main_image,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Track {
        diesel::insert_into(tracks::table)
            .values(self)
            .returning(Track::as_returning())
            .get_result(conn)
            .expect("Error saving Book")
    }
}

impl Asset for Album {
    fn read(conn: &mut PgConnection, a_id: i32) -> Album {
        use crate::schema::tracks::dsl::*;

        let album = get_album(conn, a_id);

        let track = tracks
            .filter(album_id.eq(a_id))
            .select(Track::as_select())
            .get_results(conn)
            .expect("Error loading posts");

        Album {
            id: album.id,
            creator_id: album.creator_id,
            title: album.title,
            thumb: album.thumb,
            summary: album.summary,
            directory: album.directory,
            is_free: album.is_free,
            main_image: album.main_image,
            tracks: track,
        }
    }

    fn destroy(conn: &mut PgConnection, a_id: i32) -> usize {
        use crate::schema::tracks::dsl::*;

        let changes = diesel::delete(tracks.filter(album_id.eq(a_id)))
            .execute(conn)
            .expect("Error deleting posts");

        changes + destroy_album(conn, a_id)
    }

    fn update(&self, conn: &mut PgConnection) -> usize {
        use crate::schema::albums::dsl::*;

        let album = AlbumQuery {
            id: self.id,
            creator_id: self.creator_id.to_owned(),
            title: self.title.to_owned(),
            thumb: self.thumb.to_owned(),
            summary: self.summary.to_owned(),
            directory: self.directory.to_owned(),
            is_free: self.is_free,
            main_image: self.main_image.to_owned(),
        };

        let changes = diesel::update(albums)
            .filter(id.eq(album.id))
            .set(album)
            .execute(conn)
            .expect("Failed to update user");

        update_tracks(conn, &self.tracks) + changes
    }

    fn summarize(&self, conn: &mut PgConnection, user_id: i32) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let asset_type = AssetType::Album;
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
        let asset_type = AssetType::Album;
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
            UserAlbum::check_ownership(conn, user_id, self.id)
        }
    }
}

fn get_album(conn: &mut PgConnection, album_id: i32) -> AlbumQuery {
    use crate::schema::albums::dsl::*;

    albums
        .filter(id.eq(album_id))
        .select(AlbumQuery::as_select())
        .get_result(conn)
        .expect("Error loading posts")
}

fn destroy_album(conn: &mut PgConnection, album_id: i32) -> usize {
    use crate::schema::albums::dsl::*;

    diesel::delete(albums.filter(id.eq(album_id)))
        .execute(conn)
        .expect("Error deleting posts")
}

fn update_tracks(conn: &mut PgConnection, tracks_vec: &Vec<Track>) -> usize {
    use crate::schema::tracks::dsl::*;

    let mut changes: usize = 0;
    for track in tracks_vec {
        let result = diesel::update(tracks)
            .filter(id.eq(track.id))
            .set(track)
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

    #[test]
    fn album_full() {
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
            Some(String::from("frank")),
            Some(String::from("Random House")),
            DisplayName::Other,
        );

        let album = AlbumCreate::new(
            creator.id,
            String::from("Operation Doomsday"),
            String::from("thumb.jpg"),
            String::from("A great album by Domm"),
            String::from("directory/"),
            false,
            String::from("image.jpg"),
        )
        .create(conn);

        vec![TrackCreate::new(
            creator.id,
            album.id,
            String::from("Doomsday"),
            &album.directory,
            String::from("track.jpg"),
        )
        .create(conn)];

        let album_full = Album::read(conn, album.id);

        assert_eq!(album_full.tracks[0].title, "Doomsday");

        let page = album_full.paginate(conn, user.id);

        assert_eq!(page.display_name, "frank");

        let delete = Album::destroy(conn, album.id);

        assert_eq!(delete, 2);

        Creators::destroy(conn, creator.id);
        User::destroy(conn, user.id);
    }
}
