set -ex

cargo build --all --release --target $TARGET --verbose
cp target/$TARGET/release/android_localization_cli android_localization-$TRAVIS_TAG-$TARGET