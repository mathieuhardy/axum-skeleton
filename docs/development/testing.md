# 💯 Testing

## Tests plans

Tests plans describe the list of tests to be performed. Tests coverage are not
sufficient as they don't ensure that all cases are correctly handled but it's a
start.

Eatch tests plan contains tests cases that are identified by a unique identifier
whose syntax is a path starting by `/TC/` (e.g. `/TC/MOD/SUB-MOD/TEST_01`);

- 👥 [Users](testing/plans/users.md)

## End-to-end tests

Every route defined in the `api` folders of every hexagonal crate must be tested
and if possible with different inputs in order to check input validations and
errors management.

Every test can instantiate the server and obtain a HTTP client by calling;

```rust
let client = init_server().await.unwrap();
```

The client allows to perform CRUD requests with dedicated methods. For more
information, see the documentation of the [reqwest][0] crate.

## Unit tests

For every other parts of the code, functions, derives macros, etc, must be
tested with unit tests. A test coverage can be performed to check what's
missing (see [Sanity](sanity.md)).

[0]: https://docs.rs/reqwest/latest/reqwest/
