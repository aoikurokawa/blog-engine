-- Add migration script here
CREATE TABLE blogs(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at timestamptz NOT NULL
);