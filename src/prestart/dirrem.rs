use std::path::PathBuf;
use log::error;
use rusty_engine::fileutils::get_home_path;

/// Cleanup all directories and files generated by the app (this is to uninstall the game)
pub fn dir_cleanup() {
    let _home = get_home_path();
    
}

fn del_if_exists(path: &PathBuf) {
    if path.exists() {
        if let Err(why) = std::fs::remove_dir(path) {
            error!("Unable to cleanup dir {path:?}: {why:?}");
        }
    }
}
