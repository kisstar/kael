mod cli;

fn main() {
    cli::create_commander().get_matches();
}
