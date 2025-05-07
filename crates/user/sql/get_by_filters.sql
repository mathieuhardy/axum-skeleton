-- $1: First name (optional)
-- $2: Last name (optional)
-- $3: Email (optional)
-- $4: Role (optional)

SELECT
    u.id AS "id!: _",
    u.first_name AS "first_name!: _",
    u.last_name AS "last_name!: _",
    u.email AS "email!: _",
    u.role AS "role!: _",
    u.password AS "password!: _",
    u.created_at AS "created_at!: _",
    u.updated_at AS "updated_at!: _",
    TO_JSONB(uc) AS "pending_confirmation?: _"
FROM users u
LEFT JOIN user_confirmations uc ON uc.user_id = u.id
WHERE
    ($1::VARCHAR IS NULL OR u.first_name = $1::varchar) AND
    ($2::VARCHAR IS NULL OR u.last_name = $2::varchar) AND
    ($3::VARCHAR IS NULL OR u.email = $3::varchar) AND
    ($4::user_role IS NULL OR u.role = $4::user_role);
