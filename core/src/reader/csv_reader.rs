mod error;
mod multi_locale_reader;
mod single_locale_reader;

pub use self::error::Error;
pub use self::multi_locale_reader::read as multi_locale_read;
pub use self::single_locale_reader::read as single_locale_read;
