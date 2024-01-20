# Axum skeleton

## Description

This repository is a proof-of-concept of a backend server using Axum. This
skeleton can be used as starter kit.

## Documentations

- [⚙️ Backend configuration](docs/configuration.md)
- [📄 Logging system](docs/logging.md)

# TODO

- reddis
- app state
- routes
- shutdown signal
- tower_http::trace::TraceLayer
- script to check upgradable dependencies
- [tracing](https://docs.rs/tracing/latest/tracing/)
- generate report for test coverage

## Sanity

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```
