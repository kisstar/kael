use env_logger::Builder;

pub use log::{debug, error, info, warn};

pub fn init() {
    let mut builder = Builder::from_env(env_logger::Env::default().default_filter_or("info"));

    builder.init();
}
