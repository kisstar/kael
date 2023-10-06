use clap::{Arg, Command};

pub fn register(commander: Command) -> Command {
    commander.subcommand(
        Command::new("bili")
            .about("A series of commands provided for BiliBili.")
            .arg(
                // Merge the media files in the `INDIR` directory into MP4 files and output them to `OUTDIR`.
                Arg::new("merge")
                    .long("merge")
                    .short('m')
                    .num_args(2)
                    .value_names(["INDIR", "OUTDIR"])
                    .help("Merge the cache files of Bilibili official program."),
            ),
    )
}
