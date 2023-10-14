mod merge;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Args<'a> {
    pub check: Option<&'a bool>,
    pub input: Option<&'a String>,
    pub output: Option<&'a String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VideoInfo {
    pub groupTitle: String,
    pub title: String,
}

pub fn run(args: Args) {
    merge::run(args);
}
