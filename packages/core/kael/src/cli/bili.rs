use clap::{arg, Command};

pub fn register(commander: Command) -> Command {
    let bili_commander = Command::new("bili")
        .about("A series of commands provided for BiliBili")
        .subcommand(
            // Merge the media files in the `INDIR` directory into MP4 files and output them to `OUTDIR`
            Command::new("merge")
                .about("Merge the cache files of Bilibili official program")
                .arg(arg!(-i --input <INPUT_DIR> "Directory where local cache files are located"))
                .arg(arg!(-o --output <OUTPUT_DIR> "Storage directory for merged media files")),
        );

    commander.subcommand(bili_commander)
}
