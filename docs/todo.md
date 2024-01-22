# TODO

- Remove sql/ path for scripts
- Remove <root>/scripts/sql
- Guards/fairing
- OpenApi
- Reddis
- Tower_http::trace::TraceLayer
- [tracing](https://docs.rs/tracing/latest/tracing/)
- GraphQL
- Job queue
- Docs migration sqlx
- URL overload ?
- Derive macros

## Docs

- Setup environement
- docker usage
- sqlx usage
- SQL scripts auto-process

## Devops

- CircleCI DLC

## Tests

- Dedicated database for tests
- Script to initialize database for tests

## Sanity

- generate report for test coverage
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```

## Links

- [Axum Postgres Skeleton](https://github.com/koskeller/axum-postgres-skeleton)
- [Axum Postgres Skeleton (2)](https://github.com/Sirneij/cryptoflow)

- [Axum testing](https://github.com/tokio-rs/axum/tree/main/examples/testing)
- [Axum validator](https://github.com/tokio-rs/axum/tree/main/examples/validator)
- [Axum SQLX Postgres](https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres)
- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)
