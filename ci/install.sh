set -ex

# Install required rust components
rustup target add $TARGET
rustup component add rustfmt --toolchain $TARGET
rustup component add clippy --toolchain $TARGET
