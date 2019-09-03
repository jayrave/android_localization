use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output};

use tempfile::TempDir;

use test_utilities;

mod file_utilities;

#[test]
fn succeeds_with_mapping() {
    execute_with_copied_sample_res(
        tempfile::tempdir().unwrap(),
        "success",
        |output_res_path: String| {
            let output = Command::new("cargo")
                .args(vec![
                    "run",
                    "localized",
                    "--res-dir",
                    &output_res_path.clone(),
                    "--input-file",
                    "./tests_data/localized/success/input/localized_with_mapping.csv",
                    "--mapping",
                    "french=fr",
                    "--mapping",
                    "spanish=es",
                ])
                .output()
                .unwrap();

            assert_status_and_stdout(output);
            assert_output_files(output_res_path);
        },
    )
}

#[test]
fn succeeds_without_mapping() {
    execute_with_copied_sample_res(
        tempfile::tempdir().unwrap(),
        "success",
        |output_res_path: String| {
            let output = Command::new("cargo")
                .args(vec![
                    "run",
                    "localized",
                    "--res-dir",
                    &output_res_path.clone(),
                    "--input-file",
                    "./tests_data/localized/success/input/localized_without_mapping.csv",
                ])
                .output()
                .unwrap();

            assert_status_and_stdout(output);
            assert_output_files(output_res_path);
        },
    )
}

#[test]
fn warns_if_nothing_new_localized() {
    execute_with_copied_sample_res(
        tempfile::tempdir().unwrap(),
        "warn",
        |output_res_path: String| {
            let output = Command::new("cargo")
                .args(vec![
                    "run",
                    "localized",
                    "--res-dir",
                    &output_res_path.clone(),
                    "--input-file",
                    "./tests_data/localized/warn/input/localized.csv",
                ])
                .output()
                .unwrap();

            assert!(!output.status.success());
            assert!(String::from_utf8(output.stderr)
                .unwrap()
                .contains("No updated localized texts found\n"));
        },
    )
}

#[test]
fn errors_are_printed_out() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output = Command::new("cargo")
        .args(vec![
            "run",
            "localized",
            "--res-dir",
            &format!(
                "{}/non_existent",
                temp_dir.path().to_path_buf().to_str().unwrap()
            ),
            "--input-file",
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Res dir path doesn't exist or it is not a directory\n"));
}

fn execute_with_copied_sample_res<F>(temp_dir: TempDir, input_type: &str, test: F)
where
    F: FnOnce(String) -> (),
{
    // Build paths
    let mut res_path = temp_dir.path().to_path_buf();
    res_path.push("res");

    let mut default_strings =
        test_utilities::res::setup_empty_strings_for_default_locale(res_path.clone());
    let mut fr_strings =
        test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "fr");
    let mut es_strings =
        test_utilities::res::setup_empty_strings_for_locale(res_path.clone(), "es");

    // Write out required contents into files
    default_strings
        .file
        .write(
            test_utilities::file::read_content(format!(
                "./tests_data/localized/{}/input/sample_res/values/strings.xml",
                input_type
            ))
            .as_bytes(),
        )
        .unwrap();

    fr_strings
        .file
        .write(
            test_utilities::file::read_content(format!(
                "./tests_data/localized/{}/input/sample_res/values-fr/strings.xml",
                input_type
            ))
            .as_bytes(),
        )
        .unwrap();

    es_strings
        .file
        .write(
            test_utilities::file::read_content(format!(
                "./tests_data/localized/{}/input/sample_res/values-es/strings.xml",
                input_type
            ))
            .as_bytes(),
        )
        .unwrap();

    test(String::from(res_path.to_str().unwrap()))
}

fn assert_status_and_stdout(output: Output) {
    assert!(output.status.success());

    let output = String::from_utf8(output.stdout).unwrap();
    let mut output_lines = output.split("\n");

    let fr_path = String::from(PathBuf::from("values-fr/strings.xml").to_str().unwrap());
    let es_path = String::from(PathBuf::from("values-es/strings.xml").to_str().unwrap());

    assert_eq!(
        output_lines.next().unwrap(),
        "Localized texts written to - "
    );
    assert_eq!(output_lines.next().unwrap(), "");
    test_utilities::eq::assert_eq_to_either_or_by(
        output_lines.next().unwrap(),
        &fr_path,
        &es_path,
        |actual, expected| actual.contains(expected),
    );
    test_utilities::eq::assert_eq_to_either_or_by(
        output_lines.next().unwrap(),
        &fr_path,
        &es_path,
        |actual, expected| actual.contains(expected),
    );
    assert_eq!(output_lines.next().unwrap(), "");
    assert_eq!(output_lines.next(), None);
}

fn assert_output_files(output_res_path: String) {
    file_utilities::assert_eq_of_file_contents(
        "./tests_data/localized/success/output/french_strings.xml",
        &format!("{}/values-fr/strings.xml", output_res_path),
    );

    file_utilities::assert_eq_of_file_contents(
        "./tests_data/localized/success/output/spanish_strings.xml",
        &format!("{}/values-es/strings.xml", output_res_path),
    );
}
