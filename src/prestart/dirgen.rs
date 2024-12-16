use std::fs;
use log::error;
use rusty_engine::fileutils::get_home_path;

/// Generates needed directories & files for clients
pub fn client_gen() {
    let home = get_home_path();
    // Create root config directory:
    if let Err(why) = fs::create_dir(home.join("client_config")) {
        error!("Unable to create root client config directory: {why:?}");
    }
}

/// Generates needed directories & files for servers
pub fn server_gen() {
    let home = get_home_path();
    // Create root config directory:
    if let Err(why) = fs::create_dir(home.join("server_config")) {
        error!("Unable to create root client config directory: {why:?}");
    }

}