mod args_parser;
mod args_user;
mod constants;
use std::ffi::OsString;

pub fn execute_for_commands<I, T>(itr: I) -> Result<(), ()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    args_user::execute_for_matches(args_parser::build().get_matches_from(itr))
}
