-- Your SQL goes here
create table streams(
    id varchar not null primary key default gen_random_uuid(),
    streamed_by varchar not null,
    title varchar not null,
    description varchar not null,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    foreign key (streamed_by) references users(id)
);