use fs_extra::{
    dir::{create, get_dir_content},
    file::read_to_string,
};
use kael_logger::error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::File, io::{prelude::*, SeekFrom}, path::Path};

#[derive(Debug)]
pub struct Args<'a> {
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
    if let Some(input) = args.input {
        let cache_dir_content = get_dir_content(input).unwrap();
        let media_dirs = cache_dir_content.directories;

        if media_dirs.len() <= 1 {
            error!("No media directories found.");
        }

        for (i, dir) in media_dirs.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let media_dir_content = get_dir_content(dir).unwrap();

            for file_path in media_dir_content.files {
                if file_path.ends_with(".videoInfo") {
                    let json_string = read_to_string(file_path).unwrap();
                    let video_info: VideoInfo = serde_json::from_str(&json_string).unwrap();

                    // create a folder with a group name
                    if let Some(output) = args.output {
                        let group_dir = format!("{}/{}", output, video_info.groupTitle);
                        let group_dir_path = Path::new(&group_dir);

                        if !group_dir_path.exists() {
                            create(group_dir, false).unwrap();
                        }
                    }
                } else if file_path.ends_with(".m4s") {
                    // read the file and remove the leading zero bytes
                    let mut data = vec![];
                    let mut media_file = File::options()
                        .read(true)
                        .write(true)
                        .open(file_path)
                        .unwrap();

                    media_file.read_to_end(&mut data).unwrap();
                    media_file.seek(SeekFrom::Start(0)).unwrap();
                    media_file.write_all(&data[9..]).unwrap();
                }
            }

            // merge files
        }
    } else {
        error!("Please specify the directory where the local cache files are located.");
    }
}
