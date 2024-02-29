// @generated automatically by Diesel CLI.

diesel::table! {
    album_images (id) {
        id -> Int4,
        album_id -> Int4,
        #[max_length = 50]
        file -> Varchar,
    }
}

diesel::table! {
    albums (id) {
        id -> Int4,
        creator_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 35]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        #[max_length = 50]
        directory -> Varchar,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    book_images (id) {
        id -> Int4,
        book_id -> Int4,
        #[max_length = 50]
        file -> Varchar,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        creator_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 35]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        #[max_length = 50]
        file -> Varchar,
        pages -> Int4,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    creators (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 35]
        first_name -> Nullable<Varchar>,
        #[max_length = 35]
        last_name -> Nullable<Varchar>,
        #[max_length = 50]
        other_name -> Nullable<Varchar>,
        #[max_length = 50]
        publisher -> Nullable<Varchar>,
        #[max_length = 20]
        default_name -> Varchar,
    }
}

diesel::table! {
    map_images (id) {
        id -> Int4,
        map_id -> Int4,
        #[max_length = 50]
        file -> Varchar,
    }
}

diesel::table! {
    map_pack_images (id) {
        id -> Int4,
        map_pack_id -> Int4,
        #[max_length = 50]
        file -> Varchar,
    }
}

diesel::table! {
    map_packs (id) {
        id -> Int4,
        creator_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 50]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        #[max_length = 50]
        directory -> Varchar,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    maps (id) {
        id -> Int4,
        creator_id -> Int4,
        map_pack_id -> Nullable<Int4>,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 35]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        height -> Nullable<Int4>,
        width -> Nullable<Int4>,
        #[max_length = 50]
        file -> Varchar,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    stl_images (id) {
        id -> Int4,
        stl_id -> Int4,
        #[max_length = 50]
        file -> Varchar,
    }
}

diesel::table! {
    stls (id) {
        id -> Int4,
        creator_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 35]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        #[max_length = 50]
        file -> Varchar,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    token_pack_images (id) {
        id -> Int4,
        token_pack_id -> Int4,
        #[max_length = 50]
        file -> Varchar,
    }
}

diesel::table! {
    token_packs (id) {
        id -> Int4,
        creator_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 35]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        #[max_length = 50]
        directory -> Varchar,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        creator_id -> Int4,
        token_pack_id -> Nullable<Int4>,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 35]
        thumb -> Varchar,
        #[max_length = 280]
        summary -> Varchar,
        height -> Nullable<Int4>,
        width -> Nullable<Int4>,
        #[max_length = 50]
        file -> Varchar,
        #[max_length = 50]
        main_image -> Varchar,
        #[max_length = 50]
        display_name -> Varchar,
        is_free -> Bool,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        creator_id -> Int4,
        album_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 50]
        file -> Varchar,
        #[max_length = 50]
        main_image -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_albums (user_id, album_id) {
        user_id -> Int4,
        album_id -> Int4,
    }
}

diesel::table! {
    user_books (user_id, book_id) {
        user_id -> Int4,
        book_id -> Int4,
    }
}

diesel::table! {
    user_maps (user_id, map_id) {
        user_id -> Int4,
        map_id -> Int4,
    }
}

diesel::table! {
    user_stls (user_id, stl_id) {
        user_id -> Int4,
        stl_id -> Int4,
    }
}

diesel::table! {
    user_token_packs (user_id, token_pack_id) {
        user_id -> Int4,
        token_pack_id -> Int4,
    }
}

diesel::table! {
    user_tokens (user_id, token_id) {
        user_id -> Int4,
        token_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 50]
        logo -> Varchar,
    }
}

diesel::joinable!(album_images -> albums (album_id));
diesel::joinable!(albums -> creators (creator_id));
diesel::joinable!(book_images -> books (book_id));
diesel::joinable!(books -> creators (creator_id));
diesel::joinable!(creators -> users (user_id));
diesel::joinable!(map_images -> maps (map_id));
diesel::joinable!(map_pack_images -> map_packs (map_pack_id));
diesel::joinable!(map_packs -> creators (creator_id));
diesel::joinable!(maps -> creators (creator_id));
diesel::joinable!(maps -> map_packs (map_pack_id));
diesel::joinable!(stl_images -> stls (stl_id));
diesel::joinable!(stls -> creators (creator_id));
diesel::joinable!(token_pack_images -> token_packs (token_pack_id));
diesel::joinable!(token_packs -> creators (creator_id));
diesel::joinable!(tokens -> creators (creator_id));
diesel::joinable!(tokens -> token_packs (token_pack_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> creators (creator_id));
diesel::joinable!(user_albums -> albums (album_id));
diesel::joinable!(user_albums -> users (user_id));
diesel::joinable!(user_books -> books (book_id));
diesel::joinable!(user_books -> users (user_id));
diesel::joinable!(user_maps -> maps (map_id));
diesel::joinable!(user_maps -> users (user_id));
diesel::joinable!(user_stls -> stls (stl_id));
diesel::joinable!(user_stls -> users (user_id));
diesel::joinable!(user_token_packs -> token_packs (token_pack_id));
diesel::joinable!(user_token_packs -> users (user_id));
diesel::joinable!(user_tokens -> tokens (token_id));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_images,
    albums,
    book_images,
    books,
    creators,
    map_images,
    map_pack_images,
    map_packs,
    maps,
    stl_images,
    stls,
    token_pack_images,
    token_packs,
    tokens,
    tracks,
    user_albums,
    user_books,
    user_maps,
    user_stls,
    user_token_packs,
    user_tokens,
    users,
);
