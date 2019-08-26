use crate::constants;
use android_localization_core;
use android_localization_helpers::DevExpt;
use clap::ArgMatches;
use console::style;
use std::collections::HashMap;
use std::fmt;

/// In this file, you most probably would see wide spread usages of `Option#unwrap`.
/// Please don't let that bother you as the requirements are correctly setup in
/// `args_parser.rs` & unwrapped values are guaranteed to be present there
pub fn execute_for_matches(matches: ArgMatches) -> Result<(), ()> {
    if let Some(localized_command) = matches.subcommand_matches(constants::command::LOCALIZED) {
        return localized(localized_command);
    }

    if let Some(localize_command) = matches.subcommand_matches(constants::command::LOCALIZE) {
        return localize(localize_command);
    }

    if let Some(validations_command) = matches.subcommand_matches(constants::command::VALIDATE) {
        return validate(validations_command);
    }

    exit_on_failure(String::from("Command couldn't be recognized"))
}

fn localize(matches: &ArgMatches) -> Result<(), ()> {
    exit_based_on_result(
        "Texts to be localized written to",
        android_localization_core::localize::localize(
            matches
                .value_of(constants::arg::RES_DIR)
                .expt(arg_missing_msg(constants::arg::RES_DIR)),
            matches
                .value_of(constants::arg::LOCALIZE_OUTPUT_DIR)
                .expt(arg_missing_msg(constants::arg::LOCALIZE_OUTPUT_DIR)),
            build_mappings(matches),
        ),
    )
}

fn localized(matches: &ArgMatches) -> Result<(), ()> {
    exit_based_on_result(
        "Localized texts written to",
        android_localization_core::localized::localized(
            matches
                .value_of(constants::arg::RES_DIR)
                .expt(arg_missing_msg(constants::arg::RES_DIR)),
            matches
                .value_of(constants::arg::LOCALIZED_INPUT_FILE)
                .expt(arg_missing_msg(constants::arg::LOCALIZED_INPUT_FILE)),
            build_mappings(matches),
        ),
    )
}

fn validate(matches: &ArgMatches) -> Result<(), ()> {
    let result = android_localization_core::validator::validate(
        matches
            .value_of(constants::arg::RES_DIR)
            .expt(arg_missing_msg(constants::arg::RES_DIR)),
    );
    match result {
        Err(error) => exit_based_on_result("", Err(error)),

        Ok(validation_result) => match validation_result {
            Ok(file_names) => {
                let result: Result<Vec<String>, String> = Ok(file_names);
                exit_based_on_result("No issues found. Validated the following files", result)
            }

            Err(invalid_strings_files) => exit_on_failure(
                android_localization_core::formatter::format_to_string(invalid_strings_files).unwrap_or_else(|_| String::from("Looks like this utility is experiencing issues while displaying some invalid strings! Please contact the dev (jayrave) about this error")),
            ),
        },
    }
}

fn build_mappings(matches: &ArgMatches) -> HashMap<String, String> {
    match matches.values_of(constants::arg::MAPPING) {
        None => HashMap::new(),
        Some(values) => values
            .map(|mapping| {
                let captures = constants::TEXT_TO_TEXT_REGEX
                    .captures(mapping)
                    .expt(invalid_mapping_validator_msg());
                (
                    String::from(
                        captures
                            .get(1)
                            .expt(invalid_mapping_validator_msg())
                            .as_str(),
                    ),
                    String::from(
                        captures
                            .get(2)
                            .expt(invalid_mapping_validator_msg())
                            .as_str(),
                    ),
                )
            })
            .collect(),
    }
}

fn arg_missing_msg(arg_name: &str) -> String {
    format!("{} arg is missing", arg_name)
}

fn invalid_mapping_validator_msg() -> &'static str {
    "Looks like mapping validator doesn't work!"
}

fn exit_based_on_result<E: fmt::Display>(
    success_prefix: &str,
    result: Result<Vec<String>, E>,
) -> Result<(), ()> {
    match result {
        Ok(file_names) => exit_on_success(format!(
            "{} - \n\n{}",
            success_prefix,
            file_names.join("\n")
        )),

        Err(error) => exit_on_failure(error.to_string()),
    }
}

fn exit_on_success(output: String) -> Result<(), ()> {
    println!("{}", style(output).green());
    Ok(())
}

fn exit_on_failure(error: String) -> Result<(), ()> {
    eprintln!("{}", style(error).red());
    Err(())
}
