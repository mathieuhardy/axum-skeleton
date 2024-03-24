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
    INSERT INTO users (first_name, last_name, email, password)
    VALUES
        -- Original password: Z0*zZZZZ
        ('Giga', 'Chad', 'giga@chad.com', '$argon2id$v=19$m=16,t=2,p=1$WlpaWlpaWlo$kKCIyiEfQQAj7k/dvZFC1Q'),

        -- Original password: johndoeisthebest
        ('John', 'Doe', 'john@doe.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$zs3MjnjdDjde5NfooJ0f+g'),

        -- Original password: nothisisjaneofcourse
        ('Jane', 'Doe', 'jane@doe.com', '$argon2id$v=19$m=16,t=2,p=1$YWJjZGVmZ2hpamtsbW5vcA$4kRXsgWWfcwrxbN9NOkX0A');
END
$$;
