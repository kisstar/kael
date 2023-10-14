mod args;
mod cmd;
mod dir;
mod file;

use indicatif::ProgressBar;
use kael_logger::error;
use std::process;

const VIDEO_INFO_FILENAME: &str = ".videoInfo";
const MEDIA_FILENAME_SUFFIX: &str = ".m4s";

pub fn run(args: crate::Args) {
    let merge_args = args::parse(args).unwrap_or_else(|| {
        error!("Please specify the directory where the local cache files are located.");
        process::exit(1);
    });
    let cache_dir_content = dir::get_dir_contents(merge_args.input).unwrap_or_else(|_| {
        error!("Failed to read directory {}.", merge_args.input);
        process::exit(1);
    });
    let media_dirs = cache_dir_content.directories;
    let spinner = ProgressBar::new_spinner();

    for media_dir in media_dirs.iter().skip(1) {
        spinner.set_message(format!(
            "Start processing media files in directory: {}",
            media_dir
        ));

        let video_info_file = format!("{}/{}", media_dir, VIDEO_INFO_FILENAME);
        let video_info = file::parse_video_info(&video_info_file).unwrap_or_else(|_| {
            error!("Failed to obtain video information.");
            process::exit(1);
        });
        let media_dir_content = dir::get_dir_contents(media_dir).unwrap_or_else(|_| {
            error!("Failed to read directory {}.", media_dir);
            process::exit(1);
        });
        let mut media_file_paths = vec![];

        for file_path in media_dir_content.files {
            if file_path.ends_with(MEDIA_FILENAME_SUFFIX) {
                let tmp_file = file::calibrate_media_file(&file_path).unwrap_or_else(|_| {
                    error!("Processing {} file failed", file_path);
                    process::exit(1);
                });

                media_file_paths.push(tmp_file);
            }
        }

        file::ensure_output_dir(merge_args.output, &video_info).unwrap_or_else(|_| {
            error!("Failed to create directory.");
            process::exit(1);
        });
        cmd::merge(&media_file_paths, merge_args.output, &video_info);
        file::clear_effects(&media_file_paths).unwrap_or_else(|_| {
            error!("Clearing side effects failed");
            process::exit(1);
        });
    }

    spinner.finish_with_message("All media files have been processed.");
}
