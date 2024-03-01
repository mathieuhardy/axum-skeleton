-- Create needed extensions

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create functions

CREATE FUNCTION set_updated_at()
RETURNS TRIGGER
AS
$$
BEGIN
    IF (
        new IS DISTINCT FROM old AND
        new.updated_at IS not DISTINCT FROM old.updated_at
    ) THEN
        new.updated_at := current_timestamp;
    END IF;

    RETURN new;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION create_updated_at_trigger(table_name regclass)
RETURNS VOID
AS
$$
BEGIN
    EXECUTE format('create trigger set_updated_at before update on %s
                    for each row execute procedure set_updated_at()', table_name);
END;
$$ LANGUAGE plpgsql;

-- Create tables

CREATE TABLE users (
    id            UUID primary KEY DEFAULT uuid_generate_v4(),
    first_name    VARCHAR not null,
    last_name     VARCHAR not null,
    email         VARCHAR not null,
    created_at    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    UNIQUE(email)
);

SELECT create_updated_at_trigger('users');
