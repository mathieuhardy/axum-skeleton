# TODO

- reddis
- tower_http::trace::TraceLayer
- [tracing](https://docs.rs/tracing/latest/tracing/)
- graphQL
- job queue
- docs migration sqlx
- tests-u

## Sanity

- generate report for test coverage
- dashboard with endpoints (/sanity/coverage/...)

```shell
cargo clippy --workspace --all-targets --all-features --color always --keep-going -Z unstable-options --locked --offline 2>&1 | egrep "generated \d+ warning"
```
