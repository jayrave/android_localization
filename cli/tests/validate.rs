use regex::Regex;
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

    let regex =
        Regex::new("valid_input/values/strings.xml|valid_input/values-fr/strings.xml").unwrap();

    assert_eq!(
        output_lines.next().unwrap(),
        "No issues found. Validated the following files - "
    );
    assert_eq!(output_lines.next().unwrap(), "");
    assert!(regex.is_match(output_lines.next().unwrap()));
    assert!(regex.is_match(output_lines.next().unwrap()));
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
        .ends_with("There are some validation issues! TODO => format\n"));
}
