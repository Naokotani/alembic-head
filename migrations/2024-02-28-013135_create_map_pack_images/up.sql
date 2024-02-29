-- Your SQL goes here

CREATE TABLE map_pack_images (
  image_id SERIAL PRIMARY KEY,
  map_pack_id INTEGER NOT NULL,
  FOREIGN KEY(map_pack_id) REFERENCES map_packs(map_pack_id),
  file VARCHAR(50) NOT NULL
);
