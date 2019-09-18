set -ex

# Build the required executable
cargo build --all --release --target $TARGET --verbose

# Give a more manageable name for the executable
cp target/$TARGET/release/android_localization_cli android_localization

# Zip up the executable with name that resembles android_localization-0.1.4-x86_64-unknown-linux-gnu
tar czf android_localization-$TRAVIS_TAG-$TARGET.tar.gz android_localization