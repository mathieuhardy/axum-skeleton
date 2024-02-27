# Axum skeleton

## Description

This repository is a proof-of-concept of a backend server using Axum. This
skeleton can be used as starter kit.

## Documentations

- [💻 Commands](docs/commands.md)
- [⚙️ Backend configuration](docs/configuration.md)
- [📄 Logging system](docs/logging.md)

# TODO

- reddis
- tower_http::trace::TraceLayer
- [tracing](https://docs.rs/tracing/latest/tracing/)
- graphQL
- job queue
- docs migration sqlx

## Sanity

- generate report for test coverage
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```
