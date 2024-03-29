# 🧱 Architecture

## Dependencies

                                   ╭───────╮
   ╭─────┬────────────────┬──────ᐅ │ utils │ᐊ ─╮
   │     │                │        ╰───────╯   │
   │     │                │            ᐃ       │
   │     │                │            │       │
   │ ╭───┴─────╮     ╭────┴───╮    ╭───┴────╮  │
   │ │ actions │ᐊ ─┬─┤ server ├──ᐅ │ sanity │  │
   │ ╰───┬─────╯   │ ╰────────╯    ╰────────╯  │
   │     │         │     ᐃ                     │
   │     ├─────────╯     │                     │
   │     │               │                     │
   │     ᐁ               ᐁ                     │
   │ ╭──────────╮    ╭────────────╮            │
   ╰─┤ database ├──ᐅ │ test-utils ├────────────╯
     ╰──────────╯    ╰────────────╯

## Crates

The API crate, the one that provides the HTTP endpoints is the `server`. It's
job is to instantiate the HTTP server, define the available routes and handle
authentification and authorization.

Note that endpoints (or handlers) must manage as few tasks as possible.
Typically, a handler will validate the inputs provided, check the user
authorizations, call an action that will return data and simply pass this data
back to the caller.

The `actions` crate is used to store all actions (i.e. logic) of the
application. As stated previously, if logic begins to heavy in handlers, it must
be moved to this crate. This follows the KISS (Keep It Simple and Stupid)
paradigm and allows to remove, move or replace any part of the software witout
impacting everything. Most of the time, this crate will call database actions.

The `database` crate is used to:

1. Initialize the database and provide connection(s) to it.
2. Define all data models according to database structure.
3. List of migrations to be applied.
4. Provide functions to make basic and advanced requests.

Used by all these crates, you'll find utility crates `utils` and `test-utils`.
The first one provides some common utility functions that can be usd by all
parts of the application. The second one is dedicated to the unit tests. It
simplifies the writing of tests and initialize a server to be queried.

A side crate is the `sanity`, that adds some routes to the application that show
an HTML dashboard. This dashboard shows the sanity of the repository, things
that can be fixed or improved.

> **TODO**
> worker/jobs

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
