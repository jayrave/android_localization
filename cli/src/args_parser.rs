use clap::App;
use clap::Arg;
use clap::SubCommand;
use crate::constants;

pub fn build() -> App<'static, 'static> {
    App::new("Android Strings")
        .about("To help with translations & common validations")
        .subcommand(build_to_translate_sub_command())
        .subcommand(build_from_translate_sub_command())
        .subcommand(build_validate_sub_command())
}

fn build_to_translate_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::TO_TRANSLATE)
        .about("Creates CSVs of text that need to be translated")
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg(
            "Values qualifier (eg., fr) to CSV file name (eg., french)",
        )).arg(
            Arg::with_name(constants::arg::TO_TRANSLATE_OUTPUT)
                .help("Specifies output dir to write CSV files into")
                .long(constants::arg::TO_TRANSLATE_OUTPUT)
                .short(constants::arg::short::TO_TRANSLATE_OUTPUT)
                .takes_value(true)
                .required(true),
        )
}

fn build_from_translate_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::FROM_TRANSLATE)
        .about("Populates strings XML files from translations in CSVs")
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg(
            "CSV file name (eg., french) to values qualifier (eg., fr)",
        )).arg(
            Arg::with_name(constants::arg::FROM_TRANSLATE_INPUT)
                .help("Specifies input dir to read CSV files from")
                .long(constants::arg::FROM_TRANSLATE_INPUT)
                .short(constants::arg::short::FROM_TRANSLATE_INPUT)
                .takes_value(true)
                .required(true),
        )
}

fn build_validate_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::VALIDATE)
        .about("Runs some common validations on non-default string files")
        .arg(build_res_dir_arg())
}

fn build_res_dir_arg() -> Arg<'static, 'static> {
    Arg::with_name(constants::arg::RES_DIR)
        .help("Points to the `res` dir of an Android module")
        .long(constants::arg::RES_DIR)
        .short(constants::arg::short::RES_DIR)
        .takes_value(true)
        .required(true)
}

fn build_mapping_arg(help: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(constants::arg::MAPPING)
        .help(help)
        .long(constants::arg::MAPPING)
        .short(constants::arg::short::MAPPING)
        .takes_value(true)
        .validator(mapping_validator)
        .multiple(true)
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
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
