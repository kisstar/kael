pub struct MergeArgs<'a> {
    pub check: &'a bool,
    pub input: &'a String,
    pub output: &'a String,
}

pub fn parse(args: crate::Args) -> Option<MergeArgs> {
    let check = args.check?;
    let input = args.input?;
    let output = args.output?;

    Some(MergeArgs {
        check,
        input,
        output,
    })
}
