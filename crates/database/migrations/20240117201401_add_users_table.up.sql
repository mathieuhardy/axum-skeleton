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

-- Create types

CREATE TYPE user_role AS ENUM ('admin', 'normal', 'guest');

-- Create tables

CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name  VARCHAR,
    last_name   VARCHAR,
    email       VARCHAR NOT NULL,
    role        user_role NOT NULL DEFAULT 'guest',
    password    VARCHAR NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    UNIQUE(email)
);

SELECT create_updated_at_trigger('users');

CREATE TABLE user_confirmations (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at  TIMESTAMP WITH TIME ZONE NOT NULL,

    UNIQUE(user_id)
);

-- Insert fake data (must not be done for production usage)
INSERT INTO users (first_name, last_name, email, role, password)
VALUES
    -- Original password: johndoeisthebest
    ('John', 'Doe', 'john@doe.com', 'admin', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$zs3MjnjdDjde5NfooJ0f+g'),

    -- Original password: nothisisjaneofcourse
    ('Jane', 'Doe', 'jane@doe.com', 'normal', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$4kRXsgWWfcwrxbN9NOkX0A');
