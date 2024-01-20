# Logging

Logging is performed using the [log][0] crate and these macros are available:

- `log::error!(...)`
- `log::warn!(...)`
- `log::info!(...)`
- `log::debug!(...)`
- `log::trace!(...)`

Configuration is made via the `RUST_LOG` environment variable either in `.env`
file or the variables defined in the platform.

[0]: https://docs.rs/log/latest/log
