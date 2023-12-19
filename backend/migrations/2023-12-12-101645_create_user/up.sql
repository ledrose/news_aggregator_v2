-- Your SQL goes here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

INSERT INTO roles VALUES (1,'User');
INSERT INTO roles VALUES (2,'Admin');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    passwd_hash VARCHAR NOT NULL,
    role_id INTEGER NOT NULL REFERENCES roles(id) DEFAULT 1
);

