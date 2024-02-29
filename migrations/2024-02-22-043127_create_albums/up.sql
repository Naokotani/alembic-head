-- Your SQL goes here

CREATE TABLE albums (
album_id SERIAL PRIMARY KEY,
creator_id INTEGER NOT NULL,
FOREIGN KEY (creator_id) REFERENCES creators(creator_id),
title VARCHAR(50) NOT NULL,
thumb VARCHAR(35) NOT NULL,
summary VARCHAR(280) NOT NULL,
directory VARCHAR(50) NOT NULL,
main_image VARCHAR(50) NOT NULL,
display_name VARCHAR(50) NOT NULL,
is_free boolean NOT NULL
);
