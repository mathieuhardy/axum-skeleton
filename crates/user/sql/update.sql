-- $1: User ID
-- $2: First name
-- $3: Last name
-- $4: Email
-- $5: Role
-- $6: Password

UPDATE users
SET
    first_name = $2,
    last_name = $3,
    email = $4,
    role = $5,
    password = $6
WHERE id = $1
RETURNING
    id,
    first_name,
    last_name,
    email,
    role AS "role: _",
    password,
    created_at,
    updated_at;
