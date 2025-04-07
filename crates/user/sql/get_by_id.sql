-- ID of the user to find

SELECT
    id,
    first_name,
    last_name,
    email,
    role AS "role: _",
    password,
    created_at,
    updated_at
FROM users
WHERE id = $1
LIMIT 1;
