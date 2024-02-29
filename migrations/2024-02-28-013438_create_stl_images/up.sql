-- Your SQL goes here

CREATE TABLE stl_images (
  image_id SERIAL PRIMARY KEY,
  stl_id INTEGER NOT NULL,
  FOREIGN KEY(stl_id) REFERENCES stls(stl_id),
  file VARCHAR(50) NOT NULL
);
