-- Your SQL goes here

CREATE TABLE album_images (
  image_id SERIAL PRIMARY KEY,
  album_id INTEGER NOT NULL,
  FOREIGN KEY(album_id) REFERENCES albums(album_id),
  file VARCHAR(50) NOT NULL
);
