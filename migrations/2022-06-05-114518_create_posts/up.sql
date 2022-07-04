-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY NOT NULL,
    category_id INTEGER NOT NULL REFERENCES categories (id),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);
