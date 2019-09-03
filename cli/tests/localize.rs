use std::process::{Command, Output};

use tempfile::TempDir;

mod file_utilities;

#[test]
fn succeeds_with_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localize",
            "--res-dir",
            "./tests_data/localize/success/input",
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
        "./tests_data/localize/success/output_with_mapping/",
    );
}

#[test]
fn succeeds_without_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localize",
            "--res-dir",
            "./tests_data/localize/success/input",
            "--output-dir",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert_status_and_stdout(output);
    assert_output_files(
        temp_dir,
        "./tests_data/localize/success/output_without_mapping/",
    );
}

#[test]
fn warns_if_nothing_to_localize() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localize",
            "--res-dir",
            "./tests_data/localize/warn/input",
            "--output-dir",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Nothing found to localize\n"));
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
        .contains("Res dir path doesn't exist or it is not a directory\n"));
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
    file_utilities::assert_eq_of_file_contents_to_either_or(
        &format!("{}/to_localize_1.csv", temp_dir.path().to_str().unwrap()),
        &format!("{}/es_fr.csv", expected_output_dir_path),
        &format!("{}/fr_es.csv", expected_output_dir_path),
    );
}
