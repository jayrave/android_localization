use regex::Regex;

pub mod command {
    pub const LOCALIZE: &str = "localize";
    pub const LOCALIZED: &str = "localized";
    pub const VALIDATE: &str = "validate";
}

pub mod arg {
    pub const RES_DIR: &str = "res-dir";
    pub const LOCALIZE_OUTPUT_DIR: &str = "output-dir";
    pub const LOCALIZED_INPUT_DIR: &str = "input-dir";
    pub const MAPPING: &str = "mapping";

    pub mod short {
        pub const RES_DIR: &str = "r";
        pub const LOCALIZE_OUTPUT_DIR: &str = "o";
        pub const LOCALIZED_INPUT_DIR: &str = "i";
        pub const MAPPING: &str = "m";
    }
}

lazy_static::lazy_static! {
    pub static ref TEXT_TO_TEXT_REGEX: Regex = Regex::new("^([a-zA-Z]+)=([a-zA-Z]+)$").unwrap();
}
