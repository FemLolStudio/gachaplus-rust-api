# Build container
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin gachaplus_rust_api

# Final image
FROM debian:stable-slim

WORKDIR /app
COPY --from=builder /build/target/release/gachaplus_rust_api /app/gachaplus_rust_api
COPY --from=builder /build/files/                            /app/files/


# Running
EXPOSE 8080
CMD ["./gachaplus_rust_api"]