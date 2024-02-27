-- Create needed extensions

create extension if not exists "uuid-ossp";

-- Create functions

create function set_updated_at()
returns trigger
as
$$
begin
    if (
        new is distinct from old and
        new.updated_at is not distinct from old.updated_at
    ) then
        new.updated_at := current_timestamp;
    end if;

    return new;
end;
$$ language plpgsql;

create function create_updated_at_trigger(table_name regclass)
returns void
as
$$
begin
    execute format('create trigger set_updated_at before update on %s
                    for each row execute procedure set_updated_at()', table_name);
end;
$$ language plpgsql;

-- Create tables

create table users (
    id         uuid primary key default uuid_generate_v4(),
    name       varchar not null,
    email      varchar not null,
    created_at timestamp with time zone not null default now(),
    updated_at timestamp with time zone not null default now(),

    unique(email)
);

select create_updated_at_trigger('users');

-- For testing purpose

insert into users (name, email)
values
    ('John Doe', 'john@doe.com'),
    ('Jane Doe', 'jane@doe.com');
