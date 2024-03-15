//! This file contains all SQL requests as variables.

#[rustfmt::skip]
#[allow(clippy::all)]
/// Auto-generated.
pub const SQL_USERS_FIND_BY_FILTERS: &str = "-- Fetch some users providing filters:
--
-- Arguments:
--   $1: First name of the user.
--   $2: Last name of the user.
--   $3: Email of the user.

SELECT
    *
FROM
    users
WHERE
    ($1 IS NULL OR $1 = first_name) AND
    ($2 IS NULL OR $2 = last_name) AND
    ($3 IS NULL OR $3 = email)
ORDER BY
    (first_name, last_name) ASC;";
