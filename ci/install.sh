set -ex

# Install required rust components
rustup target add $TARGET
rustup component add rustfmt
rustup component add clippy
