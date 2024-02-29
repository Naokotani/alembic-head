-- Your SQL goes here

CREATE TABLE user_tokens (
			 user_id INTEGER NOT NULL,
			 token_id INTEGER NOT NULL,
			 PRIMARY KEY (user_id, token_id),
			 FOREIGN KEY (user_id) REFERENCES users(user_id),
			 FOREIGN KEY (token_id) REFERENCES tokens(token_id)
);
