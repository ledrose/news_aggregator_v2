-- Your SQL goes here
CREATE TABLE sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL,
    source_type VARCHAR DEFAULT NULL,
    link VARCHAR DEFAULT NULL
);

CREATE TABLE themes (
    id SERIAL PRIMARY KEY,
    theme_name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE sourceThemes (
    id SERIAL PRIMARY KEY,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    theme_id INTEGER NOT NULL REFERENCES themes(id) DEFAULT 1,
    source_theme_name VARCHAR NOT NULL,
    UNIQUE(source_id,source_theme_name)
);

CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    header VARCHAR NOT NULL,
    date_time TIMESTAMPTZ NOT NULL,
    source_id INTEGER NOT NULL REFERENCES sources(id),
    theme_id INTEGER NOT NULL REFERENCES sourceThemes(id) DEFAULT 1,
    description VARCHAR,
    link VARCHAR NOT NULL
);

INSERT INTO themes VALUES (1,'Другое');
INSERT INTO sources VALUES (1,'Lenta','rss','https://lenta.ru/rss');