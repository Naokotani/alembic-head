-- Your SQL goes here

CREATE TABLE book_images (
  image_id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL,
  FOREIGN KEY(book_id) REFERENCES books(book_id),
  file VARCHAR(50) NOT NULL
);
