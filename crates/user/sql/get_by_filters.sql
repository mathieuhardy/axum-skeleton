-- $1: First name (optional)
-- $2: Last name (optional)
-- $3: Email (optional)
-- $4: Role (optional)

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
WHERE
    ($1::VARCHAR IS NULL OR first_name = $1) AND
    ($2::VARCHAR IS NULL OR last_name = $2) AND
    ($3::VARCHAR IS NULL OR email = $3) AND
    ($4::user_role IS NULL OR role = $4);
