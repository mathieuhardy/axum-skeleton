-- Fetch some users providing filters:
--
-- Arguments:
--   $1: First name of the user.
--   $2: Last name of the user.
--   $3: Email of the user.
--   $4: Password of the user.

SELECT
    *
FROM
    users
WHERE
    ($1 IS NULL OR $1 = first_name) AND
    ($2 IS NULL OR $2 = last_name) AND
    ($3 IS NULL OR $3 = email) AND
    ($4 IS NULL OR $4 = password)
ORDER BY
    (first_name, last_name) ASC;
