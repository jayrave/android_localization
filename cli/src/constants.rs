use regex::Regex;

pub mod command {
    pub const TO_CSV: &str = "to_csv";
    pub const FROM_CSV: &str = "from_csv";
    pub const VALIDATE: &str = "validate";
}

pub mod arg {
    pub const RES_DIR: &str = "res_dir";
    pub const TO_CSV_OUTPUT: &str = "translations_dir";
    pub const FROM_CSV_INPUT: &str = "translations_dir";
    pub const MAPPING: &str = "mapping";
}

lazy_static! {
    pub static ref TEXT_TO_TEXT_REGEX: Regex = Regex::new("^([a-zA-Z]+)=([a-zA-Z]+)$").unwrap();
}
