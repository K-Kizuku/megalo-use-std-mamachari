-- Your SQL goes here
create table favorites(
    user_id varchar not null,
    stream_id uuid not null,
    created_at timestamp not null default now(),
    primary key(user_id, stream_id),
    foreign key (user_id) references users(id),
    foreign key (stream_id) references streams(id)
);