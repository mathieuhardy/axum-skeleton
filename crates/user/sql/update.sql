-- $1: User ID
-- $2: First name
-- $3: Last name
-- $4: Email
-- $5: Role
-- $6: Password

WITH updated_user AS (
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
        role AS "role!: _",
        password,
        created_at,
        updated_at
)
SELECT
    u.*,
    TO_JSONB(uc) AS "pending_confirmation: _"
FROM updated_user u
LEFT JOIN user_confirmations uc ON uc.user_id = u.id;
