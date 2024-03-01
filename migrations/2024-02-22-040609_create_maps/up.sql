-- Your SQL goes here

CREATE TABLE maps (
id SERIAL PRIMARY KEY,
creator_id INTEGER NOT NULL,
FOREIGN KEY (creator_id) REFERENCES creators(id),
map_pack_id INTEGER,
FOREIGN KEY (map_pack_id) REFERENCES map_packs(id),
title VARCHAR(50) NOT NULL,
thumb VARCHAR(35) NOT NULL,
summary VARCHAR(280) NOT NULL,
height INTEGER,
width INTEGER,
file VARCHAR(50) NOT NULL,
main_image VARCHAR(50) NOT NULL,
is_free boolean NOT NULL
);
