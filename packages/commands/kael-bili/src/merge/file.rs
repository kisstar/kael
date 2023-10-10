use fs_extra::dir::create;
use fs_extra::file::read_to_string;
use std::path::Path;
use std::{
    fs::File,
    io::{prelude::*, SeekFrom},
};

const PLACEHOLDER_VALUE: u8 = 48; // ASCII
const PLACEHOLDER_COUNT: usize = 9;

// read the file and remove the leading zero bytes
pub fn calibrate_media_file(file_path: &str) {
    let mut data = vec![];
    let mut media_file = File::options()
        .read(true)
        .write(true)
        .open(file_path)
        .unwrap();

    media_file.read_to_end(&mut data).unwrap();

    let is_all_zero = data
        .iter()
        .take(PLACEHOLDER_COUNT)
        .all(|&x| x == PLACEHOLDER_VALUE);

    if is_all_zero {
        media_file.seek(SeekFrom::Start(0)).unwrap();
        media_file.write_all(&data[PLACEHOLDER_COUNT..]).unwrap();
    }
}

type VideoInfo = crate::VideoInfo;

// parsing JSON strings to obtain media information
pub fn parse_video_info(file_path: &str) -> VideoInfo {
    let json_string = read_to_string(file_path).unwrap();
    let video_info: VideoInfo = serde_json::from_str(&json_string).unwrap();

    video_info
}

pub fn ensure_output_dir(output: &str, video_info: &VideoInfo) {
    let group_dir = format!("{}/{}", output, video_info.groupTitle);
    let group_dir_path = Path::new(&group_dir);

    if !group_dir_path.exists() {
        create(group_dir, false).unwrap();
    }
}
