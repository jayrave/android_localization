use crate::constants;
use android_localization_core;
use clap::ArgMatches;
use console::style;
use std::collections::HashMap;
use std::fmt;

/// In this file, you most probably would see wide spread usages of `Option#unwrap`.
/// Please don't let that bother you as the requirements are correctly setup in
/// `args_parser.rs` & unwrapped values are guaranteed to be present there
pub fn do_the_thing(matches: &ArgMatches) -> Result<(), ()> {
    if let Some(from_csv_command) = matches.subcommand_matches(constants::command::LOCALIZED) {
        return do_from_csv(&from_csv_command);
    }

    if let Some(to_csv_command) = matches.subcommand_matches(constants::command::LOCALIZE) {
        return do_to_csv(&to_csv_command);
    }

    if let Some(validations_command) = matches.subcommand_matches(constants::command::VALIDATE) {
        return do_validations(&validations_command);
    }

    exit_on_failure(String::from("Command couldn't be recognized"))
}

fn do_to_csv(matches: &ArgMatches) -> Result<(), ()> {
    exit_based_on_result(
        "Texts to be localized written to",
        android_localization_core::localize::do_the_thing(
            matches.value_of(constants::arg::RES_DIR).unwrap(),
            matches.value_of(constants::arg::LOCALIZE_OUTPUT).unwrap(),
            build_mappings(matches),
        ),
    )
}

fn do_from_csv(matches: &ArgMatches) -> Result<(), ()> {
    exit_based_on_result(
        "Localized texts written to",
        android_localization_core::localized::do_the_thing(
            matches.value_of(constants::arg::RES_DIR).unwrap(),
            matches.value_of(constants::arg::LOCALIZED_INPUT).unwrap(),
            build_mappings(matches),
        ),
    )
}

fn do_validations(matches: &ArgMatches) -> Result<(), ()> {
    let result = android_localization_core::validator::do_the_thing(
        matches.value_of(constants::arg::RES_DIR).unwrap(),
    );
    match result {
        Err(error) => exit_based_on_result("", Err(error)),

        Ok(validation_result) => match validation_result {
            Ok(file_names) => {
                let result: Result<Vec<String>, String> = Ok(file_names);
                exit_based_on_result("No issues found. Validated the following files", result)
            }

            Err(_error) => exit_on_failure(String::from(
                "There are some validation issues! TODO => format",
            )),
        },
    }
}

fn build_mappings(matches: &ArgMatches) -> HashMap<String, String> {
    match matches.values_of(constants::arg::MAPPING) {
        None => HashMap::new(),
        Some(values) => values
            .map(|mapping| {
                let captures = constants::TEXT_TO_TEXT_REGEX.captures(mapping).unwrap();
                (
                    String::from(captures.get(1).unwrap().as_str()),
                    String::from(captures.get(2).unwrap().as_str()),
                )
            })
            .collect(),
    }
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
