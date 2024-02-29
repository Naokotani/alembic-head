-- Your SQL goes here

CREATE TABLE creators (
creator_id SERIAL PRIMARY KEY,
user_id INTEGER NOT NULL,
FOREIGN KEY (user_id) REFERENCES users(user_id),
first_name VARCHAR(35),
last_name VARCHAR(35),
other_name VARCHAR(50),
publisher VARCHAR(50),
default_name VARCHAR(20) NOT NULL
);