-- Your SQL goes here

CREATE TABLE tracks (
track_id SERIAL PRIMARY KEY,
creator_id INTEGER NOT NULL,
FOREIGN KEY (creator_id) REFERENCES creators(creator_id),
album_id INTEGER NOT NULL,
FOREIGN KEY (album_id) REFERENCES albums(album_id),
title VARCHAR(50) NOT NULL,
file VARCHAR(50) NOT NULL,
main_image VARCHAR(50)
);
