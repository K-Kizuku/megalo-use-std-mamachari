-- Your SQL goes here
create table users(
    id varchar not null primary key default gen_random_uuid(),
    name varchar not null,
    email varchar unique not null,
    description varchar not null,
    password varchar not null,
    created_at timestamp default now(),
    updated_at timestamp default now()
);