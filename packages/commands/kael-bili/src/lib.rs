#[derive(Debug)]
pub struct Args<'a> {
    pub input: Option<&'a String>,
    pub output: Option<&'a String>,
}

pub fn run(args: Args) {
    println!("{:#?}", args);
}
