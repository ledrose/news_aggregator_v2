-- Your SQL goes here
CREATE TABLE sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE themes (
    id SERIAL PRIMARY KEY,
    theme_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE sourceThemes (
    id SERIAL PRIMARY KEY,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    theme_id INTEGER NOT NULL REFERENCES themes(id),
    source_theme_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    header VARCHAR NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    theme_id INTEGER NOT NULL REFERENCES themes(id),
    text VARCHAR NOT NULL
);

