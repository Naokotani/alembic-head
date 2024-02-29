-- Your SQL goes here

CREATE TABLE stl_images (
  id SERIAL PRIMARY KEY,
  stl_id INTEGER NOT NULL,
  FOREIGN KEY(stl_id) REFERENCES stls(id),
  file VARCHAR(50) NOT NULL
);
