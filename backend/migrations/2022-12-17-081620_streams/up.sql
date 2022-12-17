-- Your SQL goes here
alter table streams alter column created_at set not null;
alter table streams alter column updated_at set not null;
alter table streams alter column is_streaming set not null;