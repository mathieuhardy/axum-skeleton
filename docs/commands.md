# Commands

## Build, test, run

```shell
# Build the project (debug version)
cargo build

# Build the project (release version)
cargo build --release

# Run unit tests
cargo test --workspace

# Run the application
caro run
```

## Advanced commands

Advanced commands, like sanity checks, are available in `Makefile.toml`. First
install the `cargo-make` tool:

```shell
cargo install cargo-make
```

Then you can call the command `makers` to get the list of available commands.
