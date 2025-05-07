-- ID of the user to find

SELECT
    u.id,
    u.first_name,
    u.last_name,
    u.email,
    u.role AS "role!: _",
    u.password,
    u.created_at,
    u.updated_at,
    TO_JSONB(uc) AS "pending_confirmation: _"
FROM users u
LEFT JOIN user_confirmations uc ON uc.user_id = u.id
WHERE u.id = $1
LIMIT 1;
