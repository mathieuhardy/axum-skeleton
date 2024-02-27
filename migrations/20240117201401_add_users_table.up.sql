create extension if not exists "uuid-ossp";

create table users (
    id         uuid primary key default uuid_generate_v4(),
    name       varchar not null,
    email      varchar not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),

    unique(email)
)
