# TODO

- Guards/fairing
- OpenApi
- Reddis
- Tower_http::trace::TraceLayer
- [tracing](https://docs.rs/tracing/latest/tracing/)
- GraphQL
- Job queue
- Docs migration sqlx
- URL overload ?

# Docs

- Documentation of macros, derives

## Devops

- CircleCI DLC

## Tests

- unit tests for derive macro in test-utils
- Setup teardown
- Dedicated database URL for tests
- Script to initialize database for tests

## Sanity

- cargo build --timings (+clean before)
- generate report for test coverage
- add test plan in markdown files
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```

## Links

- [Crates](https://gist.github.com/vi/6620975b737a1caecf607e88cf6b7fea)

- [Axum Postgres Skeleton](https://github.com/koskeller/axum-postgres-skeleton)
- [Axum Postgres Skeleton (2)](https://github.com/Sirneij/cryptoflow)

- [Axum testing](https://github.com/tokio-rs/axum/tree/main/examples/testing)
- [Axum validator](https://github.com/tokio-rs/axum/tree/main/examples/validator)
- [Axum SQLX Postgres](https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres)
- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)
