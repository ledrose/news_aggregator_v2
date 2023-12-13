-- Your SQL goes here
CREATE TABLE sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE theme (
    id SERIAL PRIMARY KEY,
    theme_name VARCHAR
);

CREATE TABLE sourceThemes (
    id SERIAL PRIMARY KEY,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    theme_id INTEGER NOT NULL REFERENCES theme(id),
    source_theme_name VARCHAR NOT NULL
);

CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    header VARCHAR NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    theme_id INTEGER NOT NULL REFERENCES theme(id),
    text VARCHAR NOT NULL
);

