-- Your SQL goes here
ALTER TABLE favorites DROP COLUMN content;
ALTER TABLE favorites ADD COLUMN created_at timestamp not null default now();