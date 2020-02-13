set -ex

cargo build --all --target $TARGET --verbose
cargo test --all --target $TARGET --verbose
cargo fmt -- --check
cargo clippy -- -D warnings
