# ⚡ Get started

## Build, test, run

```shell
# Build the project (debug version)
cargo build

# Build the project (release version)
cargo build --release

# Run unit tests
cargo test --workspace

# Run the application
cargo run -p axum-skeleton
```

## Advanced commands

Advanced commands, like sanity checks, are available in `Makefile.toml`. First
install the `cargo-make` tool:

```shell
cargo install cargo-make
```

Then you can call the command `makers` to get the list of available commands and
run them.

## Generate crates documentations

```shell
makers doc
```

The entry point documentation can be located here: `target/doc/axum_skeleton/index.html`.
