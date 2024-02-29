-- Your SQL goes here

CREATE TABLE books (
id SERIAL PRIMARY KEY,
creator_id INTEGER NOT NULL,
FOREIGN KEY (creator_id) REFERENCES creators(id),
title VARCHAR(50) NOT NULL,
thumb VARCHAR(35) NOT NULL,
summary VARCHAR(280) NOT NULL,
file VARCHAR(50) NOT NULL,
pages INTEGER NOT NULL,
main_image VARCHAR(50) NOT NULL,
display_name VARCHAR(50) NOT NULL,
is_free boolean NOT NULL
);
