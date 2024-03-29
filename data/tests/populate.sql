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
    INSERT INTO users (first_name, last_name, email, role, password)
    VALUES
        -- Original password: Z0*zZZZZ
        ('Giga', 'Chad', 'giga@chad.com', 'admin', '$argon2id$v=19$m=16,t=2,p=1$WlpaWlpaWlo$kKCIyiEfQQAj7k/dvZFC1Q'),

        -- Original password: Z0*zZZZZ
        ('John', 'Doe', 'john@doe.com', 'normal', '$argon2id$v=19$m=16,t=2,p=1$WlpaWlpaWlo$kKCIyiEfQQAj7k/dvZFC1Q'),

        -- Original password: Z0*zZZZZ
        ('Pae', 'Sano', 'pae@sano.com', 'guest', '$argon2id$v=19$m=16,t=2,p=1$WlpaWlpaWlo$kKCIyiEfQQAj7k/dvZFC1Q');
END
$$;
