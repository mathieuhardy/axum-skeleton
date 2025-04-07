-- $1: User ID
-- $2: Password

UPDATE users SET password = $2 WHERE id = $1;
