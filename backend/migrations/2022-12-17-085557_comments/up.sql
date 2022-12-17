-- Your SQL goes here
create table comments(
    id uuid not null primary key default gen_random_uuid(),
    user_id varchar not null,
    stream_id uuid not null,
    content varchar not null,
    created_at timestamp not null default now(),
    foreign key (user_id) references users(id),
    foreign key (stream_id) references streams(id)
);