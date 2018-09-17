extern crate android_strings_core as core;
extern crate clap;
extern crate regex;

#[macro_use]
extern crate lazy_static;

mod args_parser;
mod args_user;
mod constants;

pub fn do_the_thing() {
    args_user::do_the_thing(args_parser::build().get_matches())
}
