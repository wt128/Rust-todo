-- Your SQL goes here
create table todos (
    id serial primary key,
    title varchar(50) NOT NULL,
    content text NOT NULL,
    created_at timestamp,
    updated_at timestamp
)