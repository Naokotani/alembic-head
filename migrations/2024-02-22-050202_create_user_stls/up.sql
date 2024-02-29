-- Your SQL goes here

CREATE TABLE user_stls (
			 user_id INTEGER NOT NULL,
			 stl_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, stl_id),
			 FOREIGN KEY (user_id) REFERENCES users(user_id),
			 FOREIGN KEY (stl_id) REFERENCES stls(stl_id)
);
