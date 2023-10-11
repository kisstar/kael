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

            let video_info_file = format!("{}/{}", media_dir, ".videoInfo");
            let video_info = file::parse_video_info(&video_info_file);
            let media_dir_content = get_dir_content(media_dir).unwrap();
            let mut media_file_paths = vec![];

            // create a folder with a group name
            if let Some(output) = args.output {
                file::ensure_output_dir(output, &video_info);
            }

            for file_path in media_dir_content.files {
                if file_path.ends_with(".m4s") {
                    let tmp_file = file::calibrate_media_file(&file_path);

                    media_file_paths.push(tmp_file);
                }
            }

            cmd::merge(&media_file_paths, args.output.unwrap(), &video_info);

            let effect_files: Vec<String> = media_file_paths
                .into_iter()
                .filter(|p| p.ends_with(".bak"))
                .collect();

            fs_extra::remove_items(&effect_files).unwrap();
        }
    } else {
        error!("Please specify the directory where the local cache files are located.");
    }
}
