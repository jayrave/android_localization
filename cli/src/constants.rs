use android_localization_helpers::DevExpt;
use regex::Regex;

pub mod command {
    pub const LOCALIZE: &str = "localize";
    pub const LOCALIZED: &str = "localized";
    pub const VALIDATE: &str = "validate";
}

pub mod arg {
    pub const RES_DIR: &str = "res-dir";
    pub const LOCALIZE_OUTPUT_DIR: &str = "output-dir";
    pub const LOCALIZED_INPUT_FILE: &str = "input-file";
    pub const MAPPING: &str = "mapping";
}

lazy_static::lazy_static! {
    pub static ref TEXT_TO_TEXT_REGEX: Regex = Regex::new("^([a-zA-Z]+)=([a-zA-Z]+)$").expt("Invalid regex!");
}
