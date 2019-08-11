use crate::constants;
use clap::App;
use clap::Arg;
use clap::SubCommand;

pub fn build() -> App<'static, 'static> {
    App::new("Android Strings")
        .about("To help with localization & common validations")
        .subcommand(build_localize_sub_command())
        .subcommand(build_localized_sub_command())
        .subcommand(build_validate_sub_command())
}

fn build_localize_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::LOCALIZE)
        .about("Creates CSVs of text that need to be localized")
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg(
            "Values qualifier (eg., fr) to CSV file name (eg., french)",
        ))
        .arg(
            Arg::with_name(constants::arg::LOCALIZE_OUTPUT)
                .help("Specifies output dir to write CSV files into")
                .long(constants::arg::LOCALIZE_OUTPUT)
                .short(constants::arg::short::LOCALIZE_OUTPUT)
                .takes_value(true)
                .required(true),
        )
}

fn build_localized_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::LOCALIZED)
        .about("Populates strings XML files from localized text in CSVs")
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg(
            "CSV file name (eg., french) to values qualifier (eg., fr)",
        ))
        .arg(
            Arg::with_name(constants::arg::LOCALIZED_INPUT)
                .help("Specifies input dir to read CSV files from")
                .long(constants::arg::LOCALIZED_INPUT)
                .short(constants::arg::short::LOCALIZED_INPUT)
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
