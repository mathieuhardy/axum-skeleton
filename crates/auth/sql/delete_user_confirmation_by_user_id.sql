-- $1: ID of the user

DELETE FROM user_confirmations WHERE user_id = $1;
