# 👥 Users

## Summary

This documentation lists all tests related to users management: creation,
update, fetch, delete, etc.

All test cases are described here. Each one has a unique identifier starting
with `TC/`. The full identifier must be declared as comments in test files so
that a matching is possible by a script to generate a coverage matrix.

## Coverage

TODO

## Tests

1. [Fetch](#fetch)
2. [Create](#create)
3. [Update](#update)
4. [Delete](#delete)

### <a name="fetch"></a>1. Fetch

---

**ID**

> /TC/USERS/GET/ALL

**Description**

> We must be able to get the list of all users. This route can be accessed only
> by an admin useror a user with privileges.

---

**ID**

> /TC/USERS/GET/FILTERED

**Description**

> We must be able to get a list of users filtered. This route can be accessed only
> by an admin user or a user with privileges.
>
> The list of filters available are:
>
> - First name
> - Last name
> - email

---

**ID**

> /TC/USERS/GET/ME

**Description**

> A user must be able to get its own information.
> The route must return an UNAUTHORIZED error code in case the user can't be
> identified.

---

**ID**

> /TC/USERS/GET/ID

**Description**

> We must be able to get information of a user by giving its ID. This route can
> be accessed only by an admin user or a user with privileges.

---

### <a name="create"></a>2. Create

### <a name="update"></a>3. Update

### <a name="delete"></a>4. Delete

---

**ID**

> /TC/USERS/DELETE/ID

**Description**

> We must be able to delete a user by giving its ID. This route can be accessed
> only by an admin user or a user with privileges.

---
