-- Your SQL goes here
create table favorites(
    user_id varchar not null,
    stream_id varchar not null,
    content varchar not null,
    primary key(user_id, stream_id),
    foreign key (user_id) references users(id),
    foreign key (stream_id) references streams(id)
);