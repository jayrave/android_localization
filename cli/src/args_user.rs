use clap::ArgMatches;
use console::style;
use constants;
use core;
use std::collections::HashMap;
use std::fmt;
use std::process;

/// In this file, you most probably would see wide spread usages of `Option#unwrap`.
/// Please don't let that bother you as the requirements are correctly setup in
/// `args_parser.rs` & unwrapped values are guaranteed to be present there
pub fn do_the_thing(matches: &ArgMatches) {
    if let Some(from_csv_command) = matches.subcommand_matches(constants::command::FROM_TRANSLATE) {
        do_from_csv(&from_csv_command);
    }

    if let Some(to_csv_command) = matches.subcommand_matches(constants::command::TO_TRANSLATE) {
        do_to_csv(&to_csv_command)
    }

    if let Some(validations_command) = matches.subcommand_matches(constants::command::VALIDATE) {
        do_validations(&validations_command)
    }
}

fn do_to_csv(matches: &ArgMatches) {
    exit_appropriately(
        "Texts to be translated written to",
        core::to_translate::do_the_thing(
            matches.value_of(constants::arg::RES_DIR).unwrap(),
            matches
                .value_of(constants::arg::TO_TRANSLATE_OUTPUT)
                .unwrap(),
            build_mappings(matches),
        ),
    );
}

fn do_from_csv(matches: &ArgMatches) {
    exit_appropriately(
        "Translated texts written to",
        core::from_translate::do_the_thing(
            matches.value_of(constants::arg::RES_DIR).unwrap(),
            matches
                .value_of(constants::arg::FROM_TRANSLATE_INPUT)
                .unwrap(),
            build_mappings(matches),
        ),
    );
}

fn do_validations(matches: &ArgMatches) {
    exit_appropriately(
        "No issues found. Validated the following files",
        core::validator::do_the_thing(matches.value_of(constants::arg::RES_DIR).unwrap()),
    )
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
            }).collect(),
    }
}

fn exit_appropriately<E: fmt::Display>(success_prefix: &str, result: Result<Vec<String>, E>) {
    match result {
        Ok(file_names) => {
            let output = format!("{} - \n\n{}", success_prefix, file_names.join("\n"));
            println!("{}", style(output).green());
            process::exit(0)
        }

        Err(error) => {
            eprintln!("{}", style(error.to_string()).red());
            process::exit(1)
        }
    }
}
