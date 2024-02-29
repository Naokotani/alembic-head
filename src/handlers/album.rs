use diesel::prelude::*;
use crate::schema::tracks;
use crate::schema::albums;

#[derive(Queryable, Selectable)]
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

#[derive(Queryable, Selectable)]
#[diesel(table_name = albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub directory: String,
    pub display_name: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = albums)]
pub struct AlbumCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub directory: String,
    pub display_name: String,
    pub is_free: bool,
    pub main_image: String,
}
