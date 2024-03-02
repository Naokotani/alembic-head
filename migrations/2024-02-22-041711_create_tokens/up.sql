-- Your SQL goes here

CREATE TABLE tokens (
id SERIAL PRIMARY KEY,
creator_id INTEGER NOT NULL,
FOREIGN KEY (creator_id) REFERENCES creators(id),
token_pack_id INTEGER NOT NULL,
FOREIGN KEY (token_pack_id) REFERENCES token_packs(id),
title VARCHAR(50) NOT NULL,
thumb VARCHAR(35) NOT NULL,
summary VARCHAR(280) NOT NULL,
height INTEGER,
width INTEGER,
file VARCHAR(50) NOT NULL,
main_image VARCHAR(50) NOT NULL,
is_free boolean NOT NULL
);
