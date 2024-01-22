select
    *
from
    users
where
    ($1 is null or $1 = name) and
    ($2 is null or $2 = email)
order by
    name asc;
