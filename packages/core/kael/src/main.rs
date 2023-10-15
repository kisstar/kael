mod cli;
mod exec;

fn main() {
    kael_logger::init();

    let matches = cli::create_commander().get_matches();

    exec::run(matches);
}
