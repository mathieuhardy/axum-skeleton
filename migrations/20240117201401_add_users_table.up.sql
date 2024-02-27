-- Create needed extensions

create extension if not exists "uuid-ossp";

-- Create functions

-- TODO: updated_at

-- Create tables

create table users (
    id         uuid primary key default uuid_generate_v4(),
    name       varchar not null,
    email      varchar not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),

    unique(email)
);

-- For testing purpose

insert into users (name, email)
values
    ('John Doe', 'john@doe.com'),
    ('Jane Doe', 'jane@doe.com');
