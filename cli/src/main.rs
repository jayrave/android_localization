use std::env;
use std::process;

fn main() {
    match android_localization_cli::execute_for_commands(&mut env::args_os()) {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
