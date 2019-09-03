use std::path::PathBuf;
use std::process::Command;

use test_utilities;

#[test]
fn success_is_printed_out() {
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "validate",
            "--res-dir",
            "./tests_data/validate/valid_input",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());

    let output = String::from_utf8(output.stdout).unwrap();
    let mut output_lines = output.split("\n");

    let default_path = String::from(
        PathBuf::from("valid_input/values/strings.xml")
            .to_str()
            .unwrap(),
    );
    let fr_path = String::from(
        PathBuf::from("valid_input/values-fr/strings.xml")
            .to_str()
            .unwrap(),
    );

    assert_eq!(
        output_lines.next().unwrap(),
        "No issues found. Validated the following files - "
    );
    assert_eq!(output_lines.next().unwrap(), "");
    test_utilities::eq::assert_eq_to_either_or_by(
        output_lines.next().unwrap(),
        &default_path,
        &fr_path,
        |actual, expected| actual.contains(expected),
    );
    test_utilities::eq::assert_eq_to_either_or_by(
        output_lines.next().unwrap(),
        &default_path,
        &fr_path,
        |actual, expected| actual.contains(expected),
    );
    assert_eq!(output_lines.next().unwrap(), "");
    assert_eq!(output_lines.next(), None);
}

#[test]
fn errors_are_printed_out() {
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "validate",
            "--res-dir",
            "./tests_data/validate/invalid_input",
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Found 2 issues across 2 files!\n"));
}
