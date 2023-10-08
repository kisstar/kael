use clap::ArgMatches;
use kael_bili;

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("merge", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input");
            let output = sub_matches.get_one::<String>("output");
            let bili_args = kael_bili::Args { input, output };

            kael_bili::run(bili_args);
        }
        _ => println!("Unknown command"),
    }
}
