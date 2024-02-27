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
- Docker

## Sanity

- generate report for test coverage
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```
