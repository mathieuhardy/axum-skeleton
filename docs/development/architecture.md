# 🧱 Architecture

## Dependencies

The hexagonal architecture's goal is to isolate as much as possible the
components of the application. This allows to easily replace any part, test them
easiy and even to use them in other applications.

That doen't mean that all crates follow the hexagonal architecture. For example,
the `utils` crate is a simple library and doesn't need a repository or any API.

We can list utility crates:

- `security`: utility functions related to security (e.g. password hashing).
- `test-utils`: utility functions for the testing of the application.
- `utils`: utility functions for the whole application.

There's also some shared crates:

- `common-core`: contains the core of the application. It defines the main
  structures and traits that are used by all crates.
- `common-web`: contains the web-related structures and traits that are used by all
  crates. It also provides some utility functions to work with Axum.

And then logic crates:

- `auth`: contains the authentication logic.
- `database`: contains the database(s) related utilities.
- `k8s`: specific endpoints for Kubernetes.
- `sanity`: related to the sanity dashboard.
- `user`: management of users in the application.


## HTTP layers

See the excellent documentation from Axum about [Middlewares][0] (called layers
here).

**Cors**

This layer configures the CORS, getting values from YAML configuration files
stored in the `server` crate.

**Timeout**

This layer configures the timeout value for the HTTP endpoint. Value is read
from configuration file in the `server` crate.

**Compression**

This layer enables the possibilty to compress responses before sending.

**RequestId**

This layer sets (if not provided) a request-id header value. It also propagates
the value to responses.

This layer must be added before the tracing layer otherwise it won't be logged.

**Sensitive**

These layers mark the Authorization header as sensitive so it won't be logged.

**Tracing**

This layer enabled the tracing and logging feature for endpoints.

[0]: https://docs.rs/axum/latest/axum/middleware/index.html
