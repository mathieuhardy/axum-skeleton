# 💯 Testing

## Test plan

> **Note**
> TODO



## End-to-end tests

Every route defined in crate `server` must be tested and if possible with
different inputs in order to check input validations and errors management.

Every test should instantiate the server and obtain a HTTP client by calling;

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
