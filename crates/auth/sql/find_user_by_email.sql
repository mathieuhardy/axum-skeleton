-- $1: Email of the user to find

SELECT
    id,
    email,
    role AS "role: _",
    password
FROM users
WHERE email = $1
LIMIT 1;
