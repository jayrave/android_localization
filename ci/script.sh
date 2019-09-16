set -ex

cargo build --all --target $TARGET --verbose
cargo test --all --target $TARGET --verbose