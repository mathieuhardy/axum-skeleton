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

    -- Insert test data
    INSERT INTO users (name, email)
    VALUES
        ('John Doe', 'john@doe.com'),
        ('Jane Doe', 'jane@doe.com');
END
$$;



