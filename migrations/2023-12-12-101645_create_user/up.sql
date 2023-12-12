-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    passwd_hash VARCHAR NOT NULL,
    role INTEGER NOT NULL DEFAULT 0
)