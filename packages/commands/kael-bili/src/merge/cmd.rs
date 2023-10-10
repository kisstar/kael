use duct::cmd;
use kael_logger::error;

type VideoInfo = crate::VideoInfo;

pub fn merge(media_file_paths: &Vec<String>, output: &str, video_info: &VideoInfo) {
    let command = "ffmpeg";
    let input_aorv_file = &media_file_paths[0];
    let input_vora_file = &media_file_paths[1];
    let out_file = format!(
        "{}/{}/{}.mp4",
        output, video_info.groupTitle, video_info.title
    );

    // merge files
    match cmd!(
        command,
        "-i",
        input_aorv_file,
        "-i",
        input_vora_file,
        "-codec",
        "copy",
        out_file
    )
    .run()
    {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("{} command does not exist", command);
            } else {
                error!("{}", e);
            }
        }
    }
}
