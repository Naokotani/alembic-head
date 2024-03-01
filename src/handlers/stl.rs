use diesel::prelude::*;
use crate::schema::stls;

#[derive(Queryable, Selectable)]
#[diesel(table_name = stls)]
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
pub struct TokenCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub file: String,
    pub is_free: bool,
    pub main_image: String,
}
