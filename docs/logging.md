# Logging

Logging is performed using the [tracing][0] crate and these macros are available:

- `event!(Level::ERROR, ...)`
- `event!(Level::WARN, ...)`
- `event!(Level::INFO, ...)`
- `event!(Level::DEBUG, ...)`
- `event!(Level::TRACE, ...)`

Configuration is made via the `RUST_LOG` environment variable either in `.env`
file or the variables defined in the platform.

[0]: https://docs.rs/tracing/latest/tracing
