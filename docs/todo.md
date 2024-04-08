# ✅ TODO

- Redis
- OpenApi
- GraphQL
- Job queue
- Websockets
- Remove sensitive data from tracing

## Devops

- Kubernetes, terraform, helm

## Links

- [Template](https://github.com/janos-r/axum-template)

### Axum

- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)

### Various

- [Crates](https://gist.github.com/vi/6620975b737a1caecf607e88cf6b7fea)
- [Access](https://github.com/casbin-rs/axum-casbin)
- [OpenAPI](https://docs.rs/aide/latest/aide/axum/index.html)
- [OpenApi](https://docs.rs/okapi-operation/latest/okapi_operation/#example-using-axum-but-without-axum_integration-feature)
- [Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/)
- [Job queue](https://cetra3.github.io/blog/implementing-a-jobq)

### Websockets

- [1](https://crates.io/crates/axum-typed-websockets)
- [2](https://blog.devgenius.io/beyond-the-threads-websockets-in-rust-for-seamless-communication-e40d10e8a0e3)
- [3](https://blog.logrocket.com/build-websocket-server-with-rust/#what-websocket)

### Versioning

- [Best pratice](https://www.reddit.com/r/rust/comments/xnnnzq/whats_the_best_practice_for_shipping_multiple)
- [Versio](https://crates.io/crates/versio)
- [Cargo-edit](https://crates.io/crates/cargo-edit)
- [Cargo-workspaces](https://crates.io/crates/cargo-workspaces)

### HTTP codes

- 202 (Accepted): for jobs
- 208 (Already reported): post the same file

- 301 (Moved permanently): for GET, HEAD
- 308 (Permanent redirect): for POST

- 401 (Unauthorized): for user auth
- 403 (Forbidden): for accesses
- 423 (Locked): for anything that can be locked
- 498 (Token expired): for sessions ?

### Hashing

https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
