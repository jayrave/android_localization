use clap::App;
use clap::Arg;
use clap::SubCommand;
use constants;

pub fn build() -> App<'static, 'static> {
    App::new("Android Strings")
        .about("To help with translations & common validations")
        .subcommand(build_to_csv_sub_command())
        .subcommand(build_from_csv_sub_command())
        .subcommand(build_validate_sub_command())
}

fn build_to_csv_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::TO_CSV)
        .about("create CSVs of strings that need to be translated")
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg())
        .arg(
            Arg::with_name(constants::arg::TO_CSV_OUTPUT)
                .long(constants::arg::TO_CSV_OUTPUT)
                .takes_value(true)
                .required(true),
        )
}

fn build_from_csv_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::FROM_CSV)
        .about("populate strings XML from translations in CSVs")
        .arg(build_res_dir_arg())
        .arg(build_mapping_arg())
        .arg(
            Arg::with_name(constants::arg::FROM_CSV_INPUT)
                .long(constants::arg::FROM_CSV_INPUT)
                .takes_value(true)
                .required(true),
        )
}

fn build_validate_sub_command() -> App<'static, 'static> {
    SubCommand::with_name(constants::command::VALIDATE)
        .about("run some common validations on non-default string files")
        .arg(build_res_dir_arg())
}

fn build_res_dir_arg() -> Arg<'static, 'static> {
    Arg::with_name(constants::arg::RES_DIR)
        .long(constants::arg::RES_DIR)
        .takes_value(true)
        .required(true)
}

fn build_mapping_arg() -> Arg<'static, 'static> {
    Arg::with_name(constants::arg::MAPPING)
        .long(constants::arg::MAPPING)
        .takes_value(true)
        .validator(mapping_validator)
        .multiple(true)
        .required(true)
}

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
        Err(String::from(format!("Mapping should be of the format xx=XX; Found: {}", mapping)))
    }
}
