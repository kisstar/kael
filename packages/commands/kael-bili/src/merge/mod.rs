mod cmd;
mod file;

use fs_extra::dir::get_dir_content;
use indicatif::ProgressBar;
use kael_logger::error;

const VIDEO_INFO_FILENAME: &str = ".videoInfo";
const MEDIA_FILENAME_SUFFIX: &str = ".m4s";

pub fn run(args: crate::Args) {
    if let Some(input) = args.input {
        let cache_dir_content = get_dir_content(input).unwrap();
        let media_dirs = cache_dir_content.directories;

        if media_dirs.len() <= 1 {
            error!("No media directories found.");
        }

        let spinner = ProgressBar::new_spinner();

        for (i, media_dir) in media_dirs.iter().enumerate() {
            if i == 0 {
                continue;
            }

            spinner.set_message(format!(
                "Start processing media files in directory: {}",
                media_dir
            ));

            let video_info_file = format!("{}/{}", media_dir, VIDEO_INFO_FILENAME);
            let video_info = file::parse_video_info(&video_info_file);
            let media_dir_content = get_dir_content(media_dir).unwrap();
            let mut media_file_paths = vec![];

            // create a folder with a group name
            if let Some(output) = args.output {
                file::ensure_output_dir(output, &video_info);
            }

            for file_path in media_dir_content.files {
                if file_path.ends_with(MEDIA_FILENAME_SUFFIX) {
                    let tmp_file = file::calibrate_media_file(&file_path);

                    media_file_paths.push(tmp_file);
                }
            }

            cmd::merge(&media_file_paths, args.output.unwrap(), &video_info);
            file::clear_effects(&media_file_paths);
        }

        spinner.finish_with_message("All media files have been processed.");
    } else {
        error!("Please specify the directory where the local cache files are located.");
    }
}
