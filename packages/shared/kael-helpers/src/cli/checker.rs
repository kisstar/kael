use which::which;

pub fn is_exist(command: &str) -> bool {
    match which(command) {
        Ok(_) => true,
        Err(_) => false,
    }
}
