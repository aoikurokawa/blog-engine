-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    post_id INTEGER NOT NULL REFERENCES posts (id),
    body TEXT NOT NULL
);