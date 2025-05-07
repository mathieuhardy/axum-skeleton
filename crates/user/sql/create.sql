-- $1: First name
-- $2: Last name
-- $3: Email
-- $4: Role
-- $5: Password

WITH created_user AS (
    INSERT INTO users (first_name, last_name, email, role, password)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING
        id,
        first_name,
        last_name,
        email,
        role AS "role!: _",
        password,
        created_at,
        updated_at
)
SELECT
    u.*,
    TO_JSONB(uc) AS "pending_confirmation: _"
FROM created_user u
LEFT JOIN user_confirmations uc ON uc.user_id = u.id;
