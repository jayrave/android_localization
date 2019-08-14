mod helpers;

#[test]
fn one_locale_per_file_with_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir_path = temp_dir.path().to_str().unwrap();
    android_localization_cli::do_the_thing(vec![
        "does_not_matter",
        "localize",
        "--res-dir",
        "./tests_data/localize/input",
        "--output-dir",
        output_dir_path,
        "--mapping",
        "fr=french",
        "--mapping",
        "es=spanish",
    ])
    .unwrap();

    helpers::assert_eq_of_file_contents_to_either_or(
        temp_dir.path().to_str().unwrap(),
        "to_localize_1.csv",
        "./tests_data/localize/output_one_locale_per_file_with_mapping/",
        "es_fr.csv",
        "fr_es.csv",
    );
}

#[test]
fn one_locale_per_file_without_mapping() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir_path = temp_dir.path().to_str().unwrap();
    android_localization_cli::do_the_thing(vec![
        "does_not_matter",
        "localize",
        "--res-dir",
        "./tests_data/localize/input",
        "--output-dir",
        output_dir_path,
    ])
    .unwrap();

    helpers::assert_eq_of_file_contents_to_either_or(
        temp_dir.path().to_str().unwrap(),
        "to_localize_1.csv",
        "./tests_data/localize/output_one_locale_per_file_without_mapping/",
        "es_fr.csv",
        "fr_es.csv",
    );
}
