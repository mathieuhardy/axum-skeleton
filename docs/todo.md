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
- Derive macros

## Devops

- Docker
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

- [Axum testing](https://github.com/tokio-rs/axum/tree/main/examples/testing)
- [Axum validator](https://github.com/tokio-rs/axum/tree/main/examples/validator)
- [Axum SQLX Postgres](https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres)
- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)
