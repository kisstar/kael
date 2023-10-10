mod cmd;
mod file;

use fs_extra::dir::get_dir_content;
use kael_logger::error;

pub fn run(args: crate::Args) {
    if let Some(input) = args.input {
        let cache_dir_content = get_dir_content(input).unwrap();
        let media_dirs = cache_dir_content.directories;

        if media_dirs.len() <= 1 {
            error!("No media directories found.");
        }

        for (i, media_dir) in media_dirs.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let mut outter_video_info = None;
            let mut media_file_paths = vec![];
            let media_dir_content = get_dir_content(media_dir).unwrap();

            for file_path in media_dir_content.files {
                if file_path.ends_with(".videoInfo") {
                    let video_info = file::parse_video_info(&file_path);

                    // create a folder with a group name
                    if let Some(output) = args.output {
                        file::ensure_output_dir(output, &video_info);
                    }

                    outter_video_info = Some(video_info);
                } else if file_path.ends_with(".m4s") {
                    file::calibrate_media_file(&file_path);
                    media_file_paths.push(file_path);
                }
            }

            match outter_video_info {
                Some(video_info) => {
                    cmd::merge(&media_file_paths, args.output.unwrap(), &video_info);
                }
                None => error!("No .videoInfo file found."),
            }
        }
    } else {
        error!("Please specify the directory where the local cache files are located.");
    }
}
