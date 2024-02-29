use diesel::prelude::*;
use crate::schema::tokens;
use crate::schema::token_packs;

#[derive(Queryable, Selectable)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub token_id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub display_name: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct TokenCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub display_name: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = token_packs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenPack {
    pub token_pack_id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub directory: String,
    pub display_name: String,
    pub is_free: bool,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = token_packs)]
pub struct TokenPackCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub directory: String,
    pub display_name: String,
    pub is_free: bool,
    pub main_image: String,
}
