use regex::Regex;

pub mod command {
    pub const TO_TRANSLATE: &str = "localize";
    pub const FROM_TRANSLATE: &str = "localized";
    pub const VALIDATE: &str = "validate";
}

pub mod arg {
    pub const RES_DIR: &str = "res";
    pub const TO_TRANSLATE_OUTPUT: &str = "output";
    pub const FROM_TRANSLATE_INPUT: &str = "input";
    pub const MAPPING: &str = "mapping";

    pub mod short {
        pub const RES_DIR: &str = "r";
        pub const TO_TRANSLATE_OUTPUT: &str = "o";
        pub const FROM_TRANSLATE_INPUT: &str = "i";
        pub const MAPPING: &str = "m";
    }
}

lazy_static! {
    pub static ref TEXT_TO_TEXT_REGEX: Regex = Regex::new("^([a-zA-Z]+)=([a-zA-Z]+)$").unwrap();
}
