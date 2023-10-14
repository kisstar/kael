use kael_helpers::cli;
use kael_logger::info;

pub fn check() -> bool {
    let exist_ffmpeg = cli::is_exist("ffmpeg");

    if exist_ffmpeg {
        info!("FFmpeg ...... [√]");
    } else {
        info!("FFmpeg ...... [x]");
    }

    exist_ffmpeg
}
