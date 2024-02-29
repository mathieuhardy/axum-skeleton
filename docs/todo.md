# TODO

- Reddis
- Guards/fairing
- OpenApi
- GraphQL
- Job queue

## Docs

- Documentation of macros, derives
- Sanity dashboard

## Devops

- GitHub actions
- Kubernetes, terraform, helm

## Tests

- use hook in unit tests of server
- unit tests for derive macro in test-utils

## Sanity

- add test plan in markdown files
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```

## Links

### Axum

- [Axum validator](https://github.com/tokio-rs/axum/tree/main/examples/validator)
- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)

### Various

- [Crates](https://gist.github.com/vi/6620975b737a1caecf607e88cf6b7fea)
- [OpenApi](https://docs.rs/okapi-operation/latest/okapi_operation/#example-using-axum-but-without-axum_integration-feature)
- [Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/)
- [Job queue](https://cetra3.github.io/blog/implementing-a-jobq)

### Versioning

- [Best pratice](https://www.reddit.com/r/rust/comments/xnnnzq/whats_the_best_practice_for_shipping_multiple)
- [Versio](https://crates.io/crates/versio)
- [Cargo-edit](https://crates.io/crates/cargo-edit)
- [Cargo-workspaces](https://crates.io/crates/cargo-workspaces)
