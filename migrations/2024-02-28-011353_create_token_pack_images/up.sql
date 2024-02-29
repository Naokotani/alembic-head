-- Your SQL goes here
CREATE TABLE token_pack_images (
  id SERIAL PRIMARY KEY,
  token_pack_id INTEGER NOT NULL,
  FOREIGN KEY(token_pack_id) REFERENCES token_packs(id),
  file VARCHAR(50) NOT NULL
);
