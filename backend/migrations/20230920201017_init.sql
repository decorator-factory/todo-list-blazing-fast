-- Add migration script here
create table todo_items(
    id integer null primary key,
    title text not null,
    done integer not null
) strict;

insert into todo_items (title, done) values
    ('Learn Rust', 0),
    ('Procrastinate', 1),
    ('Learn React', 0);
