-- Your SQL goes here

CREATE TABLE album_images (
  id SERIAL PRIMARY KEY,
  album_id INTEGER NOT NULL,
  FOREIGN KEY(album_id) REFERENCES albums(id),
  file VARCHAR(50) NOT NULL
);
