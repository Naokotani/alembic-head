-- Your SQL goes here

CREATE TABLE user_token_packs (
			 user_id INTEGER NOT NULL,
			 token_pack_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, token_pack_id),
			 FOREIGN KEY (user_id) REFERENCES users(id),
			 FOREIGN KEY (token_pack_id) REFERENCES token_packs(id)
);
