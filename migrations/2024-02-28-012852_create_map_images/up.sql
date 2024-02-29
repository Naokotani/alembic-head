-- Your SQL goes here

CREATE TABLE map_images (
  id SERIAL PRIMARY KEY,
  map_id INTEGER NOT NULL,
  FOREIGN KEY(map_id) REFERENCES maps(id),
  file VARCHAR(50) NOT NULL
);
