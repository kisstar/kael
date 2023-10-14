pub struct MergeArgs<'a> {
    pub input: &'a String,
    pub output: &'a String,
}

pub fn parse(args: crate::Args) -> Option<MergeArgs> {
    let input = args.input?;
    let output = args.output?;

    Some(MergeArgs { input, output })
}
