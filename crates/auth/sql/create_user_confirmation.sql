-- $1: User ID
-- $2: Expires at

INSERT INTO user_confirmations (user_id, expires_at)
VALUES ($1, $2)
RETURNING
    id,
    user_id,
    expires_at;
