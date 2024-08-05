use std::path::{Path, PathBuf};

fn get_home_path() -> PathBuf {
    PathBuf::new()
}

pub fn client_dirs_exist() -> bool {
    let home = get_home_path();
    let dirs_exist = exists("/client_config");
    return dirs_exist;
}

fn exists(path: &str) -> bool {
    Path::new(path).exists()
}