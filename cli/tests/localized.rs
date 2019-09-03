use std::fs;
use std::fs::File;
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
    let mut res_dir_path = temp_dir.path().to_path_buf();
    res_dir_path.push("res");

    let mut default_values_dir_path = res_dir_path.clone();
    default_values_dir_path.push("values");
    let mut default_strings_file_path = default_values_dir_path.clone();
    default_strings_file_path.push("strings.xml");

    let mut fr_values_dir_path = res_dir_path.clone();
    fr_values_dir_path.push("values-fr");
    let mut fr_strings_file_path = fr_values_dir_path.clone();
    fr_strings_file_path.push("strings.xml");

    let mut es_values_dir_path = res_dir_path.clone();
    es_values_dir_path.push("values-es");
    let mut es_strings_file_path = es_values_dir_path.clone();
    es_strings_file_path.push("strings.xml");

    // Create required dirs & files
    fs::create_dir_all(default_values_dir_path.clone()).unwrap();
    fs::create_dir_all(fr_values_dir_path.clone()).unwrap();
    fs::create_dir_all(es_values_dir_path.clone()).unwrap();
    let mut default_strings_file = File::create(default_strings_file_path.clone()).unwrap();
    let mut fr_strings_file = File::create(fr_strings_file_path.clone()).unwrap();
    let mut es_strings_file = File::create(es_strings_file_path.clone()).unwrap();

    // Write out required contents into files
    default_strings_file
        .write(
            file_utilities::read_file_contents(
                &format!(
                    "./tests_data/localized/{}/input/sample_res/values/",
                    input_type
                ),
                "strings.xml",
            )
            .as_bytes(),
        )
        .unwrap();

    fr_strings_file
        .write(
            file_utilities::read_file_contents(
                &format!(
                    "./tests_data/localized/{}/input/sample_res/values-fr/",
                    input_type
                ),
                "strings.xml",
            )
            .as_bytes(),
        )
        .unwrap();

    es_strings_file
        .write(
            file_utilities::read_file_contents(
                &format!(
                    "./tests_data/localized/{}/input/sample_res/values-es/",
                    input_type
                ),
                "strings.xml",
            )
            .as_bytes(),
        )
        .unwrap();

    test(String::from(res_dir_path.to_str().unwrap()))
}

fn assert_status_and_stdout(output: Output) {
    assert!(output.status.success());

    let output = String::from_utf8(output.stdout).unwrap();
    let mut output_lines = output.split("\n");

    let mut fr_path = PathBuf::from("values-fr");
    fr_path.push("strings.xml");
    let fr_path = fr_path.to_str().unwrap();

    let mut es_path = PathBuf::from("values-es");
    es_path.push("strings.xml");
    let es_path = es_path.to_str().unwrap();

    assert_eq!(
        output_lines.next().unwrap(),
        "Localized texts written to - "
    );
    assert_eq!(output_lines.next().unwrap(), "");
    test_utilities::eq::assert_eq_to_either_or_by(
        output_lines.next().unwrap(),
        fr_path,
        es_path,
        |actual, expected| actual.contains(expected),
    );
    test_utilities::eq::assert_eq_to_either_or_by(
        output_lines.next().unwrap(),
        fr_path,
        es_path,
        |actual, expected| actual.contains(expected),
    );
    assert_eq!(output_lines.next().unwrap(), "");
    assert_eq!(output_lines.next(), None);
}

fn assert_output_files(output_res_path: String) {
    file_utilities::assert_eq_of_file_contents(
        "./tests_data/localized/success/output/",
        "french_strings.xml",
        &format!("{}/values-fr/", output_res_path),
        "strings.xml",
    );

    file_utilities::assert_eq_of_file_contents(
        "./tests_data/localized/success/output/",
        "spanish_strings.xml",
        &format!("{}/values-es/", output_res_path),
        "strings.xml",
    );
}
