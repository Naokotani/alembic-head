use super::creator::Creator;
use super::ownership::maps::UserMap;
use crate::schema::map_packs;
use crate::schema::maps;
use crate::types::asset::{Asset, AssetType, Ownership, Page, Summary};
use diesel::prelude::*;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Map {
    pub id: i32,
    pub creator_id: i32,
    pub map_pack_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub main_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = maps)]
pub struct MapCreate {
    pub creator_id: i32,
    pub map_pack_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub main_image: String,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = map_packs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MapPackQuery {
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
#[diesel(table_name = map_packs)]
pub struct MapPackCreate {
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
}

pub struct MapPack {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub thumb: String,
    pub summary: String,
    pub directory: String,
    pub is_free: bool,
    pub main_image: String,
    pub maps: Vec<Map>,
}

impl MapPackCreate {
    pub fn new(
        creator_id: i32,
        title: String,
        thumb: String,
        summary: String,
        directory: String,
        is_free: bool,
        main_image: String,
    ) -> Self {
        MapPackCreate {
            creator_id,
            title,
            thumb,
            summary,
            directory,
            is_free,
            main_image,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> MapPackQuery {
        diesel::insert_into(map_packs::table)
            .values(self)
            .returning(MapPackQuery::as_returning())
            .get_result(conn)
            .expect("Error saving Book")
    }
}

impl MapCreate {
    pub fn new(
        creator_id: i32,
        map_pack_id: i32,
        title: String,
        thumb: String,
        summary: String,
        height: Option<i32>,
        width: Option<i32>,
        directory: &str,
        main_image: String,
    ) -> Self {
        let slug = title.to_lowercase().trim().replace(" ", "-");
        let file = format!("{}/{}", directory, slug);
        MapCreate {
            creator_id,
            map_pack_id,
            thumb,
            title,
            summary,
            height,
            width,
            file,
            main_image,
        }
    }

    pub fn create(&self, conn: &mut PgConnection) -> Map {
        diesel::insert_into(maps::table)
            .values(self)
            .returning(Map::as_returning())
            .get_result(conn)
            .expect("Error saving map")
    }
}

impl Asset for MapPack {
    fn read(conn: &mut PgConnection, pack_id: i32) -> MapPack {
        use crate::schema::maps::dsl::*;

        let map_pack = get_map_pack(conn, pack_id);

        let map = maps
            .filter(map_pack_id.eq(pack_id))
            .select(Map::as_select())
            .get_results(conn)
            .expect("Error loading posts");

        MapPack {
            id: map_pack.id,
            creator_id: map_pack.creator_id,
            title: map_pack.title,
            thumb: map_pack.thumb,
            summary: map_pack.summary,
            directory: map_pack.directory,
            is_free: map_pack.is_free,
            main_image: map_pack.main_image,
            maps: map,
        }
    }

    fn destroy(conn: &mut PgConnection, pack_id: i32) -> usize {
        use crate::schema::maps::dsl::*;

        let changes = diesel::delete(maps.filter(map_pack_id.eq(pack_id)))
            .execute(conn)
            .expect("Error deleting posts");

        changes + destroy_map_pack(conn, pack_id)
    }

    fn update(&self, conn: &mut PgConnection) -> usize {
        use crate::schema::map_packs::dsl::*;

        let map_pack = MapPackQuery {
            id: self.id,
            creator_id: self.creator_id.to_owned(),
            title: self.title.to_owned(),
            thumb: self.thumb.to_owned(),
            summary: self.summary.to_owned(),
            directory: self.directory.to_owned(),
            is_free: self.is_free,
            main_image: self.main_image.to_owned(),
        };

        let changes = diesel::update(map_packs)
            .filter(id.eq(map_pack.id))
            .set(map_pack)
            .execute(conn)
            .expect("Failed to update user");

        update_maps(conn, &self.maps) + changes
    }

    fn summarize(&self, conn: &mut PgConnection, user_id: i32) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let asset_type = AssetType::Map;
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
        let asset_type = AssetType::Map;
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
            UserMap::check_ownership(conn, user_id, self.id)
        }
    }
}

fn get_map_pack(conn: &mut PgConnection, pack_id: i32) -> MapPackQuery {
    use crate::schema::map_packs::dsl::*;

    map_packs
        .filter(id.eq(pack_id))
        .select(MapPackQuery::as_select())
        .get_result(conn)
        .expect("Error loading map pack")
}

fn destroy_map_pack(conn: &mut PgConnection, pack_id: i32) -> usize {
    use crate::schema::map_packs::dsl::*;

    diesel::delete(map_packs.filter(id.eq(pack_id)))
        .execute(conn)
        .expect("Error deleting map_packs")
}

fn update_maps(conn: &mut PgConnection, maps_vec: &Vec<Map>) -> usize {
    use crate::schema::maps::dsl::*;

    let mut changes: usize = 0;
    for map in maps_vec {
        let result = diesel::update(maps)
            .filter(id.eq(map.id))
            .set(map)
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
    fn map_full() {
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

        let map_pack = MapPackCreate::new(
            creator.id,
            String::from("Epic Fights"),
            String::from("thumb.jpg"),
            String::from("Lots of great locations"),
            String::from("directory"),
            false,
            String::from("image.jpg"),
        )
        .create(conn);

        vec![MapCreate::new(
            creator.id,
            map_pack.id,
            String::from("Windy Glade"),
            String::from("thumb.jpg"),
            String::from("What a fight area!"),
            Some(450),
            Some(450),
            &map_pack.directory,
            String::from("image.jpg"),
        )
        .create(conn)];

        let album_full = MapPack::read(conn, map_pack.id);

        assert_eq!(album_full.maps[0].title, "Windy Glade");

        let page = album_full.paginate(conn, user.id);

        assert_eq!(page.display_name, "Chris Hughes");
        assert_eq!(page.asset_type, AssetType::Map);



        let delete = MapPack::destroy(conn, map_pack.id);

        assert_eq!(delete, 2);

        Creators::destroy(conn, creator.id);
        User::destroy(conn, user.id);
    }
}
