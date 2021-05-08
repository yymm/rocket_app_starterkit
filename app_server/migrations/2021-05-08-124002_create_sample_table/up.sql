-- Your SQL goes here
create table if not exists users (
  id serial primary key,
  name varchar(200) unique not null,
  password varchar(200) not null,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);
