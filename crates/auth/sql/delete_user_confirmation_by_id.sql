-- $1: ID of the user's confirmation to delete

DELETE FROM user_confirmations WHERE id = $1;
