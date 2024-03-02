use diesel::prelude::*;
use crate::schema::maps;
use crate::schema::map_packs;
use crate::types::asset::{Asset, AssetType, Ownership, Summary, Page};
use crate::handlers::creator::Creator;

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = maps)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Map {
    pub id: i32,
    pub creator_id: i32,
    pub map_pack_id: i32,
    pub title: String,
    pub thumb: String,
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
    pub file: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub main_image: String,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = map_packs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MapPack {
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

pub struct MapPackFull {
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

    pub fn create(&self, conn: &mut PgConnection) -> MapPack {
        diesel::insert_into(map_packs::table)
            .values(self)
            .returning(MapPack::as_returning())
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


impl Asset for MapPackFull {
    fn read(conn: &mut PgConnection, pack_id: i32) -> MapPackFull {
        use crate::schema::maps::dsl::*;

        let map_pack = get_map_pack(conn, pack_id);

        let map = maps
            .filter(map_pack_id.eq(pack_id))
            .select(Map::as_select())
            .get_results(conn)
            .expect("Error loading posts");

        MapPackFull {
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

        let mut changes = diesel::delete(maps.filter(map_pack_id.eq(pack_id)))
            .execute(conn)
            .expect("Error deleting posts");

        changes + destroy_map_pack(conn, pack_id)
    }

    fn update(&self, conn: &mut PgConnection) -> usize {
        use crate::schema::map_packs::dsl::*;

        let map_pack = MapPack {
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

        update_maps(conn, self.id, &self.maps) + changes
    }

    fn summarize(&self, conn: &mut PgConnection, u_id: i32) -> Summary {
        let (creator, user) = Creator::creator_with_user(conn, self.creator_id);
        let asset_type = AssetType::Album;
        let display_name = creator.get_display_name();
        let ownership = if self.is_free {
            Ownership::Free
        } else {
            Ownership::Unowned
        };

        Summary {
            display_name,
            ownership,
            asset_type,
            logo: user.logo,
        }
    }

    fn paginate(&self, conn: &mut PgConnection, user_id: i32) -> Page {
        let (creator, user) = Creator::creator_with_user(conn, self.id);
        let display_name = creator.get_display_name();
        let asset_type = AssetType::Album;
        let extra_images = asset_type.images();
        let ownership = if self.is_free {
            Ownership::Free
        } else {
            Ownership::Unowned
        };

        Page {
            display_name,
            ownership,
            asset_type,
            logo: user.logo,
            extra_images,
        }
    }
}

fn get_map_pack(conn: &mut PgConnection, pack_id: i32) -> MapPack {
    use crate::schema::map_packs::dsl::*;

    map_packs
        .filter(id.eq(pack_id))
        .select(MapPack::as_select())
        .get_result(conn)
        .expect("Error loading map pack")
}

fn destroy_map_pack(conn: &mut PgConnection, pack_id: i32) -> usize {
    use crate::schema::map_packs::dsl::*;

    diesel::delete(map_packs.filter(id.eq(pack_id)))
        .execute(conn)
        .expect("Error deleting map_packs")
}

fn update_maps(conn: &mut PgConnection, a_id: i32, maps_vec: &Vec<Map>) -> usize {
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
