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

- unit tests for derive macro in test-utils

## Sanity

- add test plan in markdown files
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```

## Links

### Axum

- [Axum Postgres Skeleton](https://github.com/koskeller/axum-postgres-skeleton)
- [Axum Postgres Skeleton (2)](https://github.com/Sirneij/cryptoflow)

- [Axum testing](https://github.com/tokio-rs/axum/tree/main/examples/testing)
- [Axum validator](https://github.com/tokio-rs/axum/tree/main/examples/validator)
- [Axum SQLX Postgres](https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres)
- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)

### Various

- [Crates](https://gist.github.com/vi/6620975b737a1caecf607e88cf6b7fea)
- [Tracing](https://carlosmv.hashnode.dev/adding-logging-and-tracing-to-an-axum-app-rust)
- [OpenApi](https://docs.rs/okapi-operation/latest/okapi_operation/#example-using-axum-but-without-axum_integration-feature)
- [Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/)
- [Job queue](https://cetra3.github.io/blog/implementing-a-jobq)

### Versioning

- [Best pratice](https://www.reddit.com/r/rust/comments/xnnnzq/whats_the_best_practice_for_shipping_multiple)
- [Versio](https://crates.io/crates/versio)
- [Cargo-edit](https://crates.io/crates/cargo-edit)
- [Cargo-workspaces](https://crates.io/crates/cargo-workspaces)

### Conventional commit

- [Documentation](https://www.conventionalcommits.org/en/v1.0.0)
- [Regex](https://gist.github.com/marcojahn/482410b728c31b221b70ea6d2c433f0c)

### Git hooks

- [Hooks(1)](https://www.viget.com/articles/two-ways-to-share-git-hooks-with-your-team)
- [Hooks(2)](https://pumpingco.de/blog/the-ultimate-guide-to-git-hooks)
- [Hooks(3)](https://stackoverflow.com/questions/3462955/putting-git-hooks-into-a-repository)
- [Hooks(4)](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- [Hooks(5)](https://www.atlassian.com/git/tutorials/git-hooks)
