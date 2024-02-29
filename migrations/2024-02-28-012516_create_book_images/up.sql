-- Your SQL goes here

CREATE TABLE book_images (
  id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL,
  FOREIGN KEY(book_id) REFERENCES books(id),
  file VARCHAR(50) NOT NULL
);
