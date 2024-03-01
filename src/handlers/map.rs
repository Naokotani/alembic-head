use diesel::prelude::*;
use crate::schema::maps;
use crate::schema::map_packs;

#[derive(Queryable, Selectable)]
#[diesel(table_name = maps)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Map {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = maps)]
pub struct MapCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = map_packs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MapPack {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = map_packs)]
pub struct MapPackCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
}
