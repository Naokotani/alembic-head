use diesel::prelude::*;
use crate::schema::books;

#[derive(Queryable, Selectable)]
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
    pub display_name: String,
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
    pub display_name: String,
    pub is_free: bool,
}
