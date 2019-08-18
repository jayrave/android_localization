mod args_parser;
mod args_user;
mod constants;
use std::ffi::OsString;

pub fn do_the_thing<I, T>(itr: I) -> Result<(), ()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    args_user::do_the_thing(&args_parser::build().get_matches_from(itr))
}
