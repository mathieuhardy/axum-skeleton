//! This file contains all SQL requests as variables.

/// Undocumented.
pub const SQL_USERS_FIND_BY_FILTERS: &str = "select
    *
from
    users
where
    ($1 is null or $1 = name) and
    ($2 is null or $2 = email)
order by
    name asc;";

/// Undocumented.
pub const SQL_USERS_FIND_BY_NAME: &str = "select
    *
from
    users
where
    ($1 is null or $1 = name) and
    ($2 is null or $2 = email)
order by
    name asc;";
