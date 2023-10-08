mod cli;
mod exec;

fn main() {
    let matches = cli::create_commander().get_matches();

    exec::run(matches);
}
