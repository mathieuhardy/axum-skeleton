DO
$$
DECLARE
	rec RECORD;
BEGIN
    -- Clear all tables
	FOR rec IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public' and tablename <> '_sqlx_migrations')
    LOOP
	    EXECUTE 'TRUNCATE TABLE ' || rec.tablename || ' CASCADE';
	END LOOP;

    CREATE TABLE unit_tests (
        id         UUID primary KEY DEFAULT uuid_generate_v4(),
        name       VARCHAR not null,
        email      VARCHAR not null,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
        updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

        UNIQUE(email)
    );

    SELECT create_updated_at_trigger('unit_tests');

    -- Insert test data
    INSERT INTO unit_tests (name, email)
    VALUES
        ('John Doe', 'john@doe.com'),
        ('Jane Doe', 'jane@doe.com');
END
$$;



