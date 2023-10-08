mod bili;

use clap::{crate_name, crate_version, Command};
use kael_helpers::compose;

fn register_commands(commander: Command) -> Command {
    compose!(bili::register)(commander)
}

fn set_basic_info(commander: Command) -> Command {
    commander
        .about("A comprehensive command-line tool.")
        .version(crate_version!())
        .propagate_version(true)
}

pub fn create_commander() -> Command {
    let commander = Command::new(crate_name!());

    compose!(register_commands, set_basic_info)(commander)
}
