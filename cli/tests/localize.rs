use std::process::{Command, Output};
use tempfile::TempDir;

mod helpers;

#[test]
fn one_locale_per_file_with_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localize",
            "--res-dir",
            "./tests_data/localize/input",
            "--output-dir",
            temp_dir.path().to_str().unwrap(),
            "--mapping",
            "fr=french",
            "--mapping",
            "es=spanish",
        ])
        .output()
        .unwrap();

    assert_status_and_stdout(output);
    assert_output_files(
        temp_dir,
        "./tests_data/localize/output_one_locale_per_file_with_mapping/",
    );
}

#[test]
fn one_locale_per_file_without_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localize",
            "--res-dir",
            "./tests_data/localize/input",
            "--output-dir",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert_status_and_stdout(output);
    assert_output_files(
        temp_dir,
        "./tests_data/localize/output_one_locale_per_file_without_mapping/",
    );
}

#[test]
fn errors_are_printed_out() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localize",
            "--res-dir",
            "./tests_data/localize/non_existent_dir",
            "--output-dir",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .ends_with("localize/non_existent_dir) doesn't exist\n"));
}

fn assert_status_and_stdout(output: Output) {
    assert!(output.status.success());

    let output = String::from_utf8(output.stdout).unwrap();
    let mut output_lines = output.split("\n");

    assert_eq!(
        output_lines.next().unwrap(),
        "Texts to be localized written to - "
    );
    assert_eq!(output_lines.next().unwrap(), "");
    assert!(output_lines.next().unwrap().ends_with("to_localize_1.csv"));
    assert_eq!(output_lines.next().unwrap(), "");
    assert_eq!(output_lines.next(), None);
}

fn assert_output_files(temp_dir: TempDir, expected_output_dir_path: &str) {
    helpers::assert_eq_of_file_contents_to_either_or(
        temp_dir.path().to_str().unwrap(),
        "to_localize_1.csv",
        expected_output_dir_path,
        "es_fr.csv",
        "fr_es.csv",
    );
}
