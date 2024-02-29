-- Your SQL goes here

CREATE TABLE user_books (
			 user_id INTEGER NOT NULL,
			 book_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, book_id),
			 FOREIGN KEY (user_id) REFERENCES users(user_id),
			 FOREIGN KEY (book_id) REFERENCES books(book_id)
);
