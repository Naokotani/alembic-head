-- Your SQL goes here

CREATE TABLE users (
user_id SERIAL PRIMARY KEY,
username VARCHAR(50) NOT NULL,
email VARCHAR(50) NOT NULL,
logo VARCHAR(50) NOT NULL
);