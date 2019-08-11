use std::env;
use std::process;

fn main() {
    match android_localization_cli::do_the_thing(&mut env::args_os()) {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
