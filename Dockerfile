# ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
# ┃ This docker is built on multi-layer system in order to avoid rebuilding    ┃
# ┃ everything if it's not needed.                                             ┃
# ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

# ┌────────────────────────────────────────────────────────────────────────────┐
# │ chef                                                                       │
# │                                                                            │
# │  ┃ This layer builds a rust Docker image, and installs the `cargo-chef`    │
# │  ┃ utility.                                                                │
# └────────────────────────────────────────────────────────────────────────────┘

FROM rust:1.75.0 as chef

WORKDIR /app

RUN cargo install cargo-chef --locked

# ┌────────────────────────────────────────────────────────────────────────────┐
# │ planner                                                                    │
# │                                                                            │
# │  ┃ This layer simply creates a recipe used to build dependencies using the │
# │  ┃ `cargo-chef` utility.                                                   │
# └────────────────────────────────────────────────────────────────────────────┘

FROM chef as planner

# Import the whole project and prepare a recipe
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# ┌────────────────────────────────────────────────────────────────────────────┐
# │ builder                                                                    │
# │                                                                            │
# │  ┃ This layer uses the previously generated recipe and builds all crates.  │
# │  ┃ Then it runs the build of the rest of the project (local crates).       │
# └────────────────────────────────────────────────────────────────────────────┘

FROM chef as builder

# Import generated recipe from previous layer
COPY --from=planner /app/recipe.json recipe.json

# Run build of dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Import the whole project and build the application
COPY . .

RUN cargo build --release

# ┌────────────────────────────────────────────────────────────────────────────┐
# │ runtime                                                                    │
# │                                                                            │
# │  ┃ This is the final stage of building. Use a minimal Linux distribution,  │
# │  ┃ copy the generated binaries and configuration files into the final      │
# │  ┃ image.                                                                  │
# └────────────────────────────────────────────────────────────────────────────┘

FROM alpine:3.19.0 as runtime

WORKDIR axum

# Binaries
COPY --from=builder /app/target/release/axum-skeleton .

# Configurations
COPY --from=builder /app/crates/sanity/config ./config/sanity
COPY --from=builder /app/crates/server/config ./config/server

# Data
COPY --from=builder /app/crates/sanity/data/dashboard ./data/sanity/dashboard
COPY --from=builder /app/crates/server/data/images ./data/images

ENTRYPOINT ["./axum-skeleton"]
