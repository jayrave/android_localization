set -ex

cargo build --all --target $TARGET --verbose
cargo test --all --target $TARGET --verbose
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
