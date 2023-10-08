mod bili;

use clap::ArgMatches;

pub fn run(arg_matches: ArgMatches) {
    match arg_matches.subcommand() {
        Some(("bili", sub_matches)) => {
            bili::handle(sub_matches);
        }
        _ => println!("Unknown command"),
    }
}
