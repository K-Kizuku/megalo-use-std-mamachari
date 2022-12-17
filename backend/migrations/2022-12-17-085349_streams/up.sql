-- Your SQL goes here
create table streams(
    id uuid not null primary key default gen_random_uuid(),
    streamed_by varchar not null,
    title varchar not null,
    description varchar not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    is_streaming boolean not null default FALSE,
    foreign key (streamed_by) references users(id)
);