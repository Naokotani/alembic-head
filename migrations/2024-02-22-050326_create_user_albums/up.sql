-- Your SQL goes here

CREATE TABLE user_albums (
			 user_id INTEGER NOT NULL,
			 album_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, album_id),
			 FOREIGN KEY (user_id) REFERENCES users(id),
			 FOREIGN KEY (album_id) REFERENCES albums(id)
);
