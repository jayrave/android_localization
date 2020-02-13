use regex::Regex;

use android_localization_utilities::DevExpt;

pub mod commands {
    pub const LOCALIZE: &str = "localize";
    pub const LOCALIZED: &str = "localized";
    pub const VALIDATE: &str = "validate";
}

pub mod args {
    pub const RES_DIR: &str = "res-dir";
    pub const LOCALIZE_OUTPUT_DIR: &str = "output-dir";
    pub const LOCALIZED_INPUT_FILE: &str = "input-file";
    pub const MAPPING: &str = "mapping";
    pub const SKIP_UNLOCALIZED: &str = "skip-unlocalized";
}

lazy_static::lazy_static! {
    pub static ref TEXT_TO_TEXT_REGEX: Regex = Regex::new("^([a-zA-Z]+)=([a-zA-Z]+)$").expt("Invalid regex!");
}
