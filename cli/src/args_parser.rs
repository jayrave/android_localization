use clap::Arg;
use clap::SubCommand;
use clap::{App, AppSettings};

use crate::constants;

mod doc {
    pub static NAME: &str = "Android Localization";
    pub static BINARY_NAME: &str = "android_localization";
    pub static SHORT: &str = "To help with localization & common validations";
    pub static LONG: &str = r#"
Helps in automating shipping off texts to be localized (by writing out CSVs
of texts to be localized) & to update `strings.xml` when the localized texts
come back (by reading localized texts from CSVs). This utility also can do
some rudimentary validations

Note: This work is still in alpha stage. Issues/comments/contributions are
welcome at https://github.com/jayrave/android_localization
    "#;

    pub mod localize {
        pub static SHORT: &str = "Creates CSVs of texts that need to be localized";
        pub static LONG: &str = r#"
Creates CSVs of texts that need to be localized. When writing out the CSV,
foreign locales that don't have the same set of strings are grouped
together in the same file. An example file would be -

string_name, default_locale         , spanish, french
string_1   , string_1 default locale,
string_3   , string_3 default locale,
"#;

        pub mod args {
            pub static OUTPUT_DIR: &str = "Specifies output dir to write CSV files to";
            pub mod mapping {
                pub static SHORT: &str = "Locale ID (fr) to CSV file name (french); Eg., fr=french";
                pub static LONG: &str = r#"
Mappings help to give an easier name to remember a locale by. For eg., when
a mapping like `fr=french` is given, the written CSV file will have a header
called `french` instead of `fr` to denote the missing values in `fr` locale

Multiple mappings can be passed in - each for a locale. Another handy use
case for these mappings is that they can be used as a filter. For eg., if an
Android project happens to have 3 locales - de, es & fr and if only mappings
for 2 of them are defined, the written CSV file would only carry those headers

Note: When no mappings are given, all foreign locales will be included in
the CSV file with their appropriate locale IDs as headers
            "#;
            }
        }
    }

    pub mod localized {
        pub static SHORT: &str = "Populates strings XML files from localized texts in CSVs";
        pub static LONG: &str = r#"
Populates strings XML files from localized texts in CSVs. The input CSV
file is expected to be in a particular format. This format is the same
as what would be written by the `localize` command but with localized
texts filled in. An example file would be -

string_name, default_locale         , spanish        , french
string_1   , string_1 default locale, spanish value 1, french value 1
string_3   , string_3 default locale, spanish value 2, french value 2

When populating the `strings.xml` file, translated texts will be written
only if that particular string's value has stayed the same in the default
locale
"#;

        pub mod args {
            pub static INPUT_FILE: &str = "Specifies input CSV file to read localized texts from";
            pub mod mapping {
                pub static SHORT: &str = "CSV file name (french) to locale ID (fr); Eg., french=fr";
                pub static LONG: &str = r#"
Mappings help to give an easier name to remember a locale by. For eg., when
a mapping like `french=fr` is given, the read CSV file will have a header
called `french` instead of `fr` to denote the localized values in `fr` locale

Multiple mappings can be passed in - each for a locale. Another handy use
case for these mappings is that they can be used as a filter. For eg., if
localized texts are available for 3 locales - de, es & fr and if only mappings
for 2 of them are defined, the `strings.xml` files of only those locales would
be updated

Note: When no mappings are given, all foreign locales will be updated with
the consideration that the headers are the locale IDs
            "#;
            }
        }
    }

    pub mod validate {
        pub static SHORT: &str = "Runs some common validations on XML string files";
        pub static LONG: &str = r#"
The following validations are run on the `strings.xml` files
    - Unescaped apostrophe (`'` without a preceeding `\`)
    - Format string mismatch with default locale (this could be either the
      number of format strings or the type of data they refer to)

Note: There are known corner cases whether these validations would be failing
incorrectly. As of now, this validation is not aware of the allowed grammar
of the `strings.xml` files. This uses some naive regex to validate
        "#;

        pub mod args {
            pub static SKIP_UNLOCALIZED: &str =
                "Set this to not fail validation in case there are unlocalized default strings";
        }
    }

    pub mod common {
        pub static RES_DIR_SHORT: &str = "Points to the `res` dir of an Android module";
        pub static RES_DIR_LONG: &str = r#"
This utility expects the Android module to follow the standard structure.
Eg., if there is a default locale & 2 foreign locales (french & spanish),
the `strings.xml` files are expected to be found in their respective
values folders => values, values-fr & values-es
"#;
    }
}

pub fn build() -> App<'static, 'static> {
    App::new(doc::NAME)
        .bin_name(doc::BINARY_NAME)
        .about(doc::SHORT)
        .long_about(doc::LONG)
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(build_localize_sub_command())
        .subcommand(build_localized_sub_command())
        .subcommand(build_validate_sub_command())
}

fn build_localize_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::commands::LOCALIZE)
        .about(doc::localize::SHORT)
        .long_about(doc::localize::LONG)
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg(
            doc::localize::args::mapping::SHORT,
            doc::localize::args::mapping::LONG.trim_start(),
        ))
        .arg(
            Arg::with_name(constants::args::LOCALIZE_OUTPUT_DIR)
                .help(doc::localize::args::OUTPUT_DIR)
                .long(constants::args::LOCALIZE_OUTPUT_DIR)
                .takes_value(true)
                .required(true),
        )
}

fn build_localized_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::commands::LOCALIZED)
        .about(doc::localized::SHORT)
        .long_about(doc::localized::LONG)
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg(
            doc::localized::args::mapping::SHORT,
            doc::localized::args::mapping::LONG.trim_start(),
        ))
        .arg(
            Arg::with_name(constants::args::LOCALIZED_INPUT_FILE)
                .help(doc::localized::args::INPUT_FILE)
                .long(constants::args::LOCALIZED_INPUT_FILE)
                .takes_value(true)
                .required(true),
        )
}

fn build_validate_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::commands::VALIDATE)
        .about(doc::validate::SHORT)
        .long_about(doc::validate::LONG)
        .arg(build_res_dir_arg())
        .arg(
            Arg::with_name(constants::args::SKIP_UNLOCALIZED)
                .help(doc::validate::args::SKIP_UNLOCALIZED)
                .long(constants::args::SKIP_UNLOCALIZED)
                .takes_value(false)
                .required(false),
        )
}

fn build_res_dir_arg() -> Arg<'static, 'static> {
    Arg::with_name(constants::args::RES_DIR)
        .help(doc::common::RES_DIR_SHORT)
        .long_help(doc::common::RES_DIR_LONG.trim_start())
        .long(constants::args::RES_DIR)
        .takes_value(true)
        .required(true)
}

fn build_mapping_arg(short_help: &'static str, long_help: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(constants::args::MAPPING)
        .help(short_help)
        .long_help(long_help.trim_start())
        .long(constants::args::MAPPING)
        .takes_value(true)
        .validator(mapping_validator)
        .multiple(true)
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
fn mapping_validator(mapping: String) -> Result<(), String> {
    let valid_mapping = match constants::TEXT_TO_TEXT_REGEX.captures(&mapping) {
        None => false,
        Some(capture) => {
            let capture_group_1 = capture.get(1);
            let capture_group_2 = capture.get(2);
            let capture_group_3 = capture.get(3);
            capture_group_1.is_some() && capture_group_2.is_some() && capture_group_3.is_none()
        }
    };

    if valid_mapping {
        Ok(())
    } else {
        Err(format!(
            "Mapping should be of the format xx=yy; Found: {}",
            mapping
        ))
    }
}
