-- ID of the user's confirmation to find

SELECT
    uc.id,
    uc.user_id,
    uc.expires_at
FROM user_confirmations uc
WHERE uc.id = $1
LIMIT 1;
