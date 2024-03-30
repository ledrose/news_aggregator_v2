-- Your SQL goes here
CREATE TABLE feeds (
    id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users(id),
	name VARCHAR NOT NULL
);

CREATE TABLE feedSource (
	id SERIAL PRIMARY KEY,
	feed_id INTEGER NOT NULL REFERENCES feeds(id),
	source_theme_id INTEGER NOT NULL REFERENCES sourceThemes(id),
	UNIQUE(feed_id,source_theme_id)
);