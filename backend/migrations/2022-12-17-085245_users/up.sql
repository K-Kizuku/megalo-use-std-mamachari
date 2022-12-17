-- Your SQL goes here
create table users(
    id varchar not null primary key,
    name varchar not null,
    email varchar unique not null,
    description varchar not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);