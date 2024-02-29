-- Your SQL goes here

CREATE TABLE user_albums (
			 user_id INTEGER NOT NULL,
			 album_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, album_id),
			 FOREIGN KEY (user_id) REFERENCES users(user_id),
			 FOREIGN KEY (album_id) REFERENCES albums(album_id)
);
