-- Your SQL goes here
alter table users alter column created_at set not null;
alter table users alter column updated_at set not null;
