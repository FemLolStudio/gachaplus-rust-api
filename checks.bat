cargo fmt
cargo sqlx prepare
cargo check --all
cargo rustc -- -D warnings
cargo test