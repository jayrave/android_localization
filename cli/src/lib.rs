mod args_parser;
mod args_user;
mod constants;

pub fn do_the_thing() {
    args_user::do_the_thing(&args_parser::build().get_matches())
}
