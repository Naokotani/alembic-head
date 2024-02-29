-- Your SQL goes here

CREATE TABLE user_maps (
			 user_id INTEGER NOT NULL,
			 map_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, map_id),
			 FOREIGN KEY (user_id) REFERENCES users(id),
			 FOREIGN KEY (map_id) REFERENCES maps(id)
);
