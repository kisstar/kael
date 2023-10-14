use fs_extra::{
    dir::{get_dir_content, DirContent},
    error::{Error, ErrorKind},
};
use kael_logger::error;

pub fn get_dir_contents(dirname: &str) -> Result<DirContent, Error> {
    match get_dir_content(dirname) {
        Ok(cache_dir_content) => Ok(cache_dir_content),
        Err(e) => match e.kind {
            ErrorKind::NotFound => {
                println!("Directory {} does not exist.", dirname);

                Err(e)
            }
            ErrorKind::PermissionDenied => {
                error!("No permission to access directory {}.", dirname);

                Err(e)
            }
            _ => Err(e),
        },
    }
}
