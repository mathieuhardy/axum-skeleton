-- $1: ID of the user to check

SELECT 1 AS exists FROM users WHERE id = $1 LIMIT 1;
