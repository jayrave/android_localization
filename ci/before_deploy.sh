set -ex

# Build the required executable
cargo build --all --release --target $TARGET --verbose

# Rename the executable
cp target/$TARGET/release/android_localization_cli$EXE_SUFFIX android_localization$EXE_SUFFIX

# Zip up the executable with name that resembles android_localization-0.1.4-x86_64-unknown-linux-gnu
tar czf android_localization-$TRAVIS_TAG-$TARGET.tar.gz android_localization$EXE_SUFFIX
