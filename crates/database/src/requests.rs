const SQL_USERS_FIND_BY_NAME: &str = "select
    *
from
    users
where
    name = $1;";
