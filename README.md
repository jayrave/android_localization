[![Build Status](https://travis-ci.org/jayrave/android_localization.svg?branch=develop)](https://travis-ci.org/jayrave/android_localization)

android_localization is a command line program to ease working with `strings.xml` for localizing to non-default locales

# Installation
Pre-built binaries can be found for the following platforms in the [release tab](https://github.com/jayrave/android_localization/releases/latest)
- Linux (64-bit)
- OSX (64-bit)
- Windows (64-bit)

Compiling from source under the assumption that the appropriate [rust toolchain is already installed](https://rustup.rs/):

```bash
git clone git@github.com:jayrave/android_localization.git
cd android_localization
cargo build --release
```

# Commands
- **localize** - Creates CSVs of texts that need to be localized
- **localized** - Populates strings XML files from localized texts in CSVs
- **validate** - Runs some common validations on XML string files

# Quick tour
You are working on your Android app or library & now it is time to localize to non-defaults locales. Probably you wanna find the texts that are yet to be localized, ship them off to a localization service, put the texts in when it comes back & make sure that it didn't get messed up in any way. This CLI helps you automate everything except the actual localization.

```bash
./android_localization localize --output-dir ~/to_localize --res-dir ~/my_app/app/src/main/res
./android_localization localized --input-file ~/localized/texts.csv --res-dir ~/my_app/app/src/main/res
./android_localization validate --res-dir ~/my_app/app/src/main/res
```
![](assets/demo.png)
