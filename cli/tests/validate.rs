mod helpers;

use std::path::PathBuf;
use std::process::Command;

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

    let mut default_path = PathBuf::from("valid_input/values");
    default_path.push("strings.xml");
    let default_path = default_path.to_str().unwrap();

    let mut fr_path = PathBuf::from("valid_input/values-fr");
    fr_path.push("strings.xml");
    let fr_path = fr_path.to_str().unwrap();

    assert_eq!(
        output_lines.next().unwrap(),
        "No issues found. Validated the following files - "
    );
    assert_eq!(output_lines.next().unwrap(), "");
    helpers::assert_eq_to_either_or(
        output_lines.next().unwrap(),
        default_path,
        fr_path,
        |actual, expected| actual.contains(expected),
    );
    helpers::assert_eq_to_either_or(
        output_lines.next().unwrap(),
        default_path,
        fr_path,
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
        .ends_with("Found 2 issues across 2 files!\n"));
}
