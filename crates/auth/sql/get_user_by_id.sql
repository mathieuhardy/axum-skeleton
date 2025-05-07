-- ID of the user to find

SELECT
    u.id,
    u.email,
    u.role AS "role: _",
    u.password,
    uc.id IS NULL AS "email_confirmed!: _"
FROM users u
LEFT JOIN user_confirmations uc ON uc.user_id = u.id
WHERE u.id = $1
LIMIT 1;
