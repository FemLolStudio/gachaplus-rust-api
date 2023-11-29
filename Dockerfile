# Build konténer
FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release

# Végleges kép
FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /build/target/release/gachaplus_rust_api /app/gachaplus_rust_api
COPY --from=builder /build/files/                            /app/files/
EXPOSE 8080
CMD ["./gachaplus_rust_api"]