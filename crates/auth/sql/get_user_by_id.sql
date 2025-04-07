-- ID of the user to find

SELECT
    id,
    email,
    role AS "role: _",
    password
FROM users
WHERE id = $1
LIMIT 1;
