-- Your SQL goes here
create table posts (
    id serial primary key not null,
    category_id integer not null references categories (id),
    title varchar not null,
    body text not null,
    published boolean not null default false,
    created timestamp not null,
    updated timestamp not null
);
