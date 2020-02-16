set -ex

# Build the required executable
cargo build --all --release --target $TARGET --verbose

# Rename the executable
cp target/$TARGET/release/android_localization_cli$EXE_SUFFIX android_localization$EXE_SUFFIX

# Zip up the executable with name that resembles android_localization-0.1.4-x86_64-unknown-linux-gnu
current_version=`grep "version" cli/Cargo.toml | sed "s/version = //g" | sed "s/'//g"`
echo "using $current_version as version from cli/Cargo.toml"
tar czf android_localization-$current_version-$TARGET.tar.gz android_localization$EXE_SUFFIX
